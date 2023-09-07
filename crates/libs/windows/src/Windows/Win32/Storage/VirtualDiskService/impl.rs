pub trait IEnumVdsObject_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppobjectarray: *mut ::core::option::Option<::windows_core::IUnknown>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::Iids for IEnumVdsObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumVdsObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppobjectarray), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumVdsObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsAdmin_Impl: ::windows_core::BaseImpl {
    fn RegisterProvider(this: &Self::This, providerid: &::windows_core::GUID, providerclsid: &::windows_core::GUID, pwszname: &::windows_core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: &::windows_core::PCWSTR, pwszversion: &::windows_core::PCWSTR, guidversionid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnregisterProvider(this: &Self::This, providerid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsAdmin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdmin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsAdmin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, providerclsid: ::windows_core::GUID, pwszname: ::windows_core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: ::windows_core::PCWSTR, pwszversion: ::windows_core::PCWSTR, guidversionid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterProvider(this, ::core::mem::transmute(&providerid), ::core::mem::transmute(&providerclsid), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwszmachinename), ::core::mem::transmute(&pwszversion), ::core::mem::transmute(&guidversionid)).into())
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterProvider(this, ::core::mem::transmute(&providerid)).into())
        }
        IVdsAdmin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterProvider: RegisterProvider::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsAdvancedDisk_Impl: ::windows_core::BaseImpl {
    fn GetPartitionProperties(this: &Self::This, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> ::windows_core::Result<()>;
    fn QueryPartitions(this: &Self::This, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> ::windows_core::Result<()>;
    fn CreatePartition(this: &Self::This, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS) -> ::windows_core::Result<IVdsAsync>;
    fn DeletePartition(this: &Self::This, ulloffset: u64, bforce: super::super::Foundation::BOOL, bforceprotected: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ChangeAttributes(this: &Self::This, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> ::windows_core::Result<()>;
    fn AssignDriveLetter(this: &Self::This, ulloffset: u64, wcletter: u16) -> ::windows_core::Result<()>;
    fn DeleteDriveLetter(this: &Self::This, ulloffset: u64, wcletter: u16) -> ::windows_core::Result<()>;
    fn GetDriveLetter(this: &Self::This, ulloffset: u64, pwcletter: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn FormatPartition(this: &Self::This, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: &::windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> ::windows_core::Result<IVdsAsync>;
    fn Clean(this: &Self::This, bforce: super::super::Foundation::BOOL, bforceoem: super::super::Foundation::BOOL, bfullclean: super::super::Foundation::BOOL) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsAdvancedDisk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsAdvancedDisk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartitionProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPartitionProperties(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&ppartitionprop)).into())
        }
        unsafe extern "system" fn QueryPartitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryPartitions(this, ::core::mem::transmute_copy(&pppartitionproparray), ::core::mem::transmute_copy(&plnumberofpartitions)).into())
        }
        unsafe extern "system" fn CreatePartition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePartition(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&ullsize), ::core::mem::transmute_copy(&para)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeletePartition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, bforce: super::super::Foundation::BOOL, bforceprotected: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePartition(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bforceprotected)).into())
        }
        unsafe extern "system" fn ChangeAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeAttributes(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&para)).into())
        }
        unsafe extern "system" fn AssignDriveLetter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, wcletter: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssignDriveLetter(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&wcletter)).into())
        }
        unsafe extern "system" fn DeleteDriveLetter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, wcletter: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDriveLetter(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&wcletter)).into())
        }
        unsafe extern "system" fn GetDriveLetter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, pwcletter: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDriveLetter(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&pwcletter)).into())
        }
        unsafe extern "system" fn FormatPartition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: ::windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatPartition(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwszlabel), ::core::mem::transmute_copy(&dwunitallocationsize), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bquickformat), ::core::mem::transmute_copy(&benablecompression)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bforce: super::super::Foundation::BOOL, bforceoem: super::super::Foundation::BOOL, bfullclean: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clean(this, ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bforceoem), ::core::mem::transmute_copy(&bfullclean)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsAdvancedDisk_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartitionProperties: GetPartitionProperties::<Identity, Impl, OFFSET>,
            QueryPartitions: QueryPartitions::<Identity, Impl, OFFSET>,
            CreatePartition: CreatePartition::<Identity, Impl, OFFSET>,
            DeletePartition: DeletePartition::<Identity, Impl, OFFSET>,
            ChangeAttributes: ChangeAttributes::<Identity, Impl, OFFSET>,
            AssignDriveLetter: AssignDriveLetter::<Identity, Impl, OFFSET>,
            DeleteDriveLetter: DeleteDriveLetter::<Identity, Impl, OFFSET>,
            GetDriveLetter: GetDriveLetter::<Identity, Impl, OFFSET>,
            FormatPartition: FormatPartition::<Identity, Impl, OFFSET>,
            Clean: Clean::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsAdvancedDisk2_Impl: ::windows_core::BaseImpl {
    fn ChangePartitionType(this: &Self::This, ulloffset: u64, bforce: super::super::Foundation::BOOL, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsAdvancedDisk2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsAdvancedDisk2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChangePartitionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, bforce: super::super::Foundation::BOOL, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangePartitionType(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&para)).into())
        }
        IVdsAdvancedDisk2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChangePartitionType: ChangePartitionType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsAdvancedDisk3_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> ::windows_core::Result<()>;
    fn GetUniqueId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IVdsAdvancedDisk3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsAdvancedDisk3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&padvdiskprop)).into())
        }
        unsafe extern "system" fn GetUniqueId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdvancedDisk3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUniqueId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsAdvancedDisk3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetUniqueId: GetUniqueId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsAdviseSink_Impl: ::windows_core::BaseImpl {
    fn OnNotify(this: &Self::This, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsAdviseSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdviseSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsAdviseSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNotify(this, ::core::mem::transmute_copy(&lnumberofnotifications), ::core::mem::transmute_copy(&pnotificationarray)).into())
        }
        IVdsAdviseSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsAsync_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Wait(this: &Self::This, phrresult: *mut ::windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows_core::Result<()>;
    fn QueryStatus(this: &Self::This, phrresult: *mut ::windows_core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pasyncout)).into())
        }
        unsafe extern "system" fn QueryStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryStatus(this, ::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pulpercentcompleted)).into())
        }
        IVdsAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsController_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows_core::Result<()>;
    fn GetSubSystem(this: &Self::This) -> ::windows_core::Result<IVdsSubSystem>;
    fn GetPortProperties(this: &Self::This, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows_core::Result<()>;
    fn FlushCache(this: &Self::This) -> ::windows_core::Result<()>;
    fn InvalidateCache(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryAssociatedLuns(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn SetStatus(this: &Self::This, status: VDS_CONTROLLER_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsController {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsController {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pcontrollerprop)).into())
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPortProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPortProperties(this, ::core::mem::transmute_copy(&sportnumber), ::core::mem::transmute_copy(&pportprop)).into())
        }
        unsafe extern "system" fn FlushCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushCache(this).into())
        }
        unsafe extern "system" fn InvalidateCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateCache(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAssociatedLuns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&status)).into())
        }
        IVdsController_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            GetPortProperties: GetPortProperties::<Identity, Impl, OFFSET>,
            FlushCache: FlushCache::<Identity, Impl, OFFSET>,
            InvalidateCache: InvalidateCache::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsControllerControllerPort_Impl: ::windows_core::BaseImpl {
    fn QueryControllerPorts(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::Iids for IVdsControllerControllerPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerControllerPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsControllerControllerPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryControllerPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerControllerPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryControllerPorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsControllerControllerPort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryControllerPorts: QueryControllerPorts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsControllerPort_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pportprop: *mut VDS_PORT_PROP) -> ::windows_core::Result<()>;
    fn GetController(this: &Self::This) -> ::windows_core::Result<IVdsController>;
    fn QueryAssociatedLuns(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, status: VDS_PORT_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsControllerPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsControllerPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pportprop)).into())
        }
        unsafe extern "system" fn GetController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontroller: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetController(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontroller, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAssociatedLuns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&status)).into())
        }
        IVdsControllerPort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetController: GetController::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsCreatePartitionEx_Impl: ::windows_core::BaseImpl {
    fn CreatePartitionEx(this: &Self::This, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsCreatePartitionEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsCreatePartitionEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsCreatePartitionEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePartitionEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsCreatePartitionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePartitionEx(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&ullsize), ::core::mem::transmute_copy(&ulalign), ::core::mem::transmute_copy(&para)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsCreatePartitionEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePartitionEx: CreatePartitionEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDisk_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pdiskproperties: *mut VDS_DISK_PROP) -> ::windows_core::Result<()>;
    fn GetPack(this: &Self::This) -> ::windows_core::Result<IVdsPack>;
    fn GetIdentificationData(this: &Self::This, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows_core::Result<()>;
    fn QueryExtents(this: &Self::This, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn ConvertStyle(this: &Self::This, newstyle: VDS_PARTITION_STYLE) -> ::windows_core::Result<()>;
    fn SetFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn ClearFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsDisk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDisk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiskproperties: *mut VDS_DISK_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pdiskproperties)).into())
        }
        unsafe extern "system" fn GetPack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppack: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIdentificationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdentificationData(this, ::core::mem::transmute_copy(&pluninfo)).into())
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryExtents(this, ::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into())
        }
        unsafe extern "system" fn ConvertStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstyle: VDS_PARTITION_STYLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStyle(this, ::core::mem::transmute_copy(&newstyle)).into())
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ClearFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        IVdsDisk_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPack: GetPack::<Identity, Impl, OFFSET>,
            GetIdentificationData: GetIdentificationData::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            ConvertStyle: ConvertStyle::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ClearFlags: ClearFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDisk2_Impl: ::windows_core::BaseImpl {
    fn SetSANMode(this: &Self::This, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsDisk2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDisk2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSANMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSANMode(this, ::core::mem::transmute_copy(&benable)).into())
        }
        IVdsDisk2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetSANMode: SetSANMode::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsDisk3_Impl: ::windows_core::BaseImpl {
    fn GetProperties2(this: &Self::This, pdiskproperties: *mut VDS_DISK_PROP2) -> ::windows_core::Result<()>;
    fn QueryFreeExtents(this: &Self::This, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsDisk3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDisk3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiskproperties: *mut VDS_DISK_PROP2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties2(this, ::core::mem::transmute_copy(&pdiskproperties)).into())
        }
        unsafe extern "system" fn QueryFreeExtents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDisk3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryFreeExtents(this, ::core::mem::transmute_copy(&ulalign), ::core::mem::transmute_copy(&ppfreeextentarray), ::core::mem::transmute_copy(&plnumberoffreeextents)).into())
        }
        IVdsDisk3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties2: GetProperties2::<Identity, Impl, OFFSET>,
            QueryFreeExtents: QueryFreeExtents::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsDiskOnline_Impl: ::windows_core::BaseImpl {
    fn Online(this: &Self::This) -> ::windows_core::Result<()>;
    fn Offline(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsDiskOnline {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskOnline_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDiskOnline {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Online<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskOnline_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Online(this).into())
        }
        unsafe extern "system" fn Offline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskOnline_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Offline(this).into())
        }
        IVdsDiskOnline_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Online: Online::<Identity, Impl, OFFSET>,
            Offline: Offline::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDiskPartitionMF_Impl: ::windows_core::BaseImpl {
    fn GetPartitionFileSystemProperties(this: &Self::This, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows_core::Result<()>;
    fn GetPartitionFileSystemTypeName(this: &Self::This, ulloffset: u64) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn QueryPartitionFileSystemFormatSupport(this: &Self::This, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows_core::Result<()>;
    fn FormatPartitionEx(this: &Self::This, ulloffset: u64, pwszfilesystemtypename: &::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &::windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsDiskPartitionMF {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDiskPartitionMF {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartitionFileSystemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPartitionFileSystemProperties(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&pfilesystemprop)).into())
        }
        unsafe extern "system" fn GetPartitionFileSystemTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, ppwszfilesystemtypename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartitionFileSystemTypeName(this, ::core::mem::transmute_copy(&ulloffset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszfilesystemtypename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryPartitionFileSystemFormatSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryPartitionFileSystemFormatSupport(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute_copy(&ppfilesystemsupportprops), ::core::mem::transmute_copy(&plnumberoffilesystems)).into())
        }
        unsafe extern "system" fn FormatPartitionEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, pwszfilesystemtypename: ::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatPartitionEx(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute(&pwszfilesystemtypename), ::core::mem::transmute_copy(&usfilesystemrevision), ::core::mem::transmute_copy(&uldesiredunitallocationsize), ::core::mem::transmute(&pwszlabel), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bquickformat), ::core::mem::transmute_copy(&benablecompression)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsDiskPartitionMF_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartitionFileSystemProperties: GetPartitionFileSystemProperties::<Identity, Impl, OFFSET>,
            GetPartitionFileSystemTypeName: GetPartitionFileSystemTypeName::<Identity, Impl, OFFSET>,
            QueryPartitionFileSystemFormatSupport: QueryPartitionFileSystemFormatSupport::<Identity, Impl, OFFSET>,
            FormatPartitionEx: FormatPartitionEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsDiskPartitionMF2_Impl: ::windows_core::BaseImpl {
    fn FormatPartitionEx2(this: &Self::This, ulloffset: u64, pwszfilesystemtypename: &::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &::windows_core::PCWSTR, options: u32) -> ::windows_core::Result<IVdsAsync>;
}
impl ::windows_core::Iids for IVdsDiskPartitionMF2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDiskPartitionMF2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FormatPartitionEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDiskPartitionMF2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffset: u64, pwszfilesystemtypename: ::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows_core::PCWSTR, options: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatPartitionEx2(this, ::core::mem::transmute_copy(&ulloffset), ::core::mem::transmute(&pwszfilesystemtypename), ::core::mem::transmute_copy(&usfilesystemrevision), ::core::mem::transmute_copy(&uldesiredunitallocationsize), ::core::mem::transmute(&pwszlabel), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsDiskPartitionMF2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FormatPartitionEx2: FormatPartitionEx2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDrive_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows_core::Result<()>;
    fn GetSubSystem(this: &Self::This) -> ::windows_core::Result<IVdsSubSystem>;
    fn QueryExtents(this: &Self::This, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn SetFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn ClearFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, status: VDS_DRIVE_STATUS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsDrive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDrive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pdriveprop)).into())
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryExtents(this, ::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into())
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ClearFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&status)).into())
        }
        IVdsDrive_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ClearFlags: ClearFlags::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsDrive2_Impl: ::windows_core::BaseImpl {
    fn GetProperties2(this: &Self::This, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsDrive2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsDrive2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsDrive2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties2(this, ::core::mem::transmute_copy(&pdriveprop2)).into())
        }
        IVdsDrive2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetProperties2: GetProperties2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsHbaPort_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, phbaportprop: *mut VDS_HBAPORT_PROP) -> ::windows_core::Result<()>;
    fn SetAllPathStatuses(this: &Self::This, status: VDS_PATH_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsHbaPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHbaPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHbaPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHbaPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phbaportprop: *mut VDS_HBAPORT_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&phbaportprop)).into())
        }
        unsafe extern "system" fn SetAllPathStatuses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHbaPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_PATH_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllPathStatuses(this, ::core::mem::transmute_copy(&status)).into())
        }
        IVdsHbaPort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetAllPathStatuses: SetAllPathStatuses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsHwProvider_Impl: ::windows_core::BaseImpl {
    fn QuerySubSystems(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn Reenumerate(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsHwProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHwProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QuerySubSystems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySubSystems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reenumerate(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        IVdsHwProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QuerySubSystems: QuerySubSystems::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderPrivate_Impl: ::windows_core::BaseImpl {
    fn QueryIfCreatedLun(this: &Self::This, pwszdevicepath: &::windows_core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsHwProviderPrivate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderPrivate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHwProviderPrivate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryIfCreatedLun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderPrivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows_core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryIfCreatedLun(this, ::core::mem::transmute(&pwszdevicepath), ::core::mem::transmute_copy(&pvdsluninformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plunid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsHwProviderPrivate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryIfCreatedLun: QueryIfCreatedLun::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsHwProviderPrivateMpio_Impl: ::windows_core::BaseImpl {
    fn SetAllPathStatusesFromHbaPort(this: &Self::This, hbaportprop: &VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsHwProviderPrivateMpio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHwProviderPrivateMpio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllPathStatusesFromHbaPort(this, ::core::mem::transmute(&hbaportprop), ::core::mem::transmute_copy(&status)).into())
        }
        IVdsHwProviderPrivateMpio_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllPathStatusesFromHbaPort: SetAllPathStatusesFromHbaPort::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderStoragePools_Impl: ::windows_core::BaseImpl {
    fn QueryStoragePools(this: &Self::This, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreateLunInStoragePool(this: &Self::This, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: &::windows_core::GUID, pwszunmaskinglist: &::windows_core::PCWSTR, phints2: *const VDS_HINTS2) -> ::windows_core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSizeInStoragePool(this: &Self::This, r#type: VDS_LUN_TYPE, storagepoolid: &::windows_core::GUID, phints2: *const VDS_HINTS2) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsHwProviderStoragePools {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHwProviderStoragePools {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryStoragePools<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryStoragePools(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ullremainingfreespace), ::core::mem::transmute_copy(&ppoolattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLunInStoragePool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows_core::GUID, pwszunmaskinglist: ::windows_core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLunInStoragePool(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute(&storagepoolid), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows_core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaxLunCreateSizeInStoragePool(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&storagepoolid), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxlunsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsHwProviderStoragePools_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryStoragePools: QueryStoragePools::<Identity, Impl, OFFSET>,
            CreateLunInStoragePool: CreateLunInStoragePool::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSizeInStoragePool: QueryMaxLunCreateSizeInStoragePool::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsHwProviderType_Impl: ::windows_core::BaseImpl {
    fn GetProviderType(this: &Self::This) -> ::windows_core::Result<VDS_HWPROVIDER_TYPE>;
}
impl ::windows_core::Iids for IVdsHwProviderType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHwProviderType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsHwProviderType_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProviderType: GetProviderType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsHwProviderType2_Impl: ::windows_core::BaseImpl {
    fn GetProviderType2(this: &Self::This) -> ::windows_core::Result<VDS_HWPROVIDER_TYPE>;
}
impl ::windows_core::Iids for IVdsHwProviderType2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderType2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsHwProviderType2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProviderType2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsHwProviderType2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderType2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsHwProviderType2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProviderType2: GetProviderType2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsIscsiInitiatorAdapter_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> ::windows_core::Result<()>;
    fn QueryInitiatorPortals(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn LoginToTarget(this: &Self::This, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: &::windows_core::GUID, targetportalid: &::windows_core::GUID, initiatorportalid: &::windows_core::GUID, ulloginflags: u32, bheaderdigest: super::super::Foundation::BOOL, bdatadigest: super::super::Foundation::BOOL, authtype: VDS_ISCSI_AUTH_TYPE) -> ::windows_core::Result<IVdsAsync>;
    fn LogoutFromTarget(this: &Self::This, targetid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsIscsiInitiatorAdapter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorAdapter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsIscsiInitiatorAdapter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pinitiatoradapterprop)).into())
        }
        unsafe extern "system" fn QueryInitiatorPortals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryInitiatorPortals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoginToTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: ::windows_core::GUID, targetportalid: ::windows_core::GUID, initiatorportalid: ::windows_core::GUID, ulloginflags: u32, bheaderdigest: super::super::Foundation::BOOL, bdatadigest: super::super::Foundation::BOOL, authtype: VDS_ISCSI_AUTH_TYPE, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoginToTarget(this, ::core::mem::transmute_copy(&logintype), ::core::mem::transmute(&targetid), ::core::mem::transmute(&targetportalid), ::core::mem::transmute(&initiatorportalid), ::core::mem::transmute_copy(&ulloginflags), ::core::mem::transmute_copy(&bheaderdigest), ::core::mem::transmute_copy(&bdatadigest), ::core::mem::transmute_copy(&authtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LogoutFromTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LogoutFromTarget(this, ::core::mem::transmute(&targetid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsIscsiInitiatorAdapter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            QueryInitiatorPortals: QueryInitiatorPortals::<Identity, Impl, OFFSET>,
            LoginToTarget: LoginToTarget::<Identity, Impl, OFFSET>,
            LogoutFromTarget: LogoutFromTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsIscsiInitiatorPortal_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> ::windows_core::Result<()>;
    fn GetInitiatorAdapter(this: &Self::This) -> ::windows_core::Result<IVdsIscsiInitiatorAdapter>;
    fn SetIpsecTunnelAddress(this: &Self::This, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::Result<()>;
    fn GetIpsecSecurity(this: &Self::This, targetportalid: &::windows_core::GUID) -> ::windows_core::Result<u64>;
    fn SetIpsecSecurity(this: &Self::This, targetportalid: &::windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsIscsiInitiatorPortal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorPortal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsIscsiInitiatorPortal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pinitiatorportalprop)).into())
        }
        unsafe extern "system" fn GetInitiatorAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinitiatoradapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInitiatorAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinitiatoradapter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecTunnelAddress(this, ::core::mem::transmute_copy(&ptunneladdress), ::core::mem::transmute_copy(&pdestinationaddress)).into())
        }
        unsafe extern "system" fn GetIpsecSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetportalid: ::windows_core::GUID, pullsecurityflags: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIpsecSecurity(this, ::core::mem::transmute(&targetportalid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullsecurityflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpsecSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiInitiatorPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetportalid: ::windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecSecurity(this, ::core::mem::transmute(&targetportalid), ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into())
        }
        IVdsIscsiInitiatorPortal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetInitiatorAdapter: GetInitiatorAdapter::<Identity, Impl, OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Identity, Impl, OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Identity, Impl, OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsIscsiPortal_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows_core::Result<()>;
    fn GetSubSystem(this: &Self::This) -> ::windows_core::Result<IVdsSubSystem>;
    fn QueryAssociatedPortalGroups(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn SetStatus(this: &Self::This, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows_core::Result<()>;
    fn SetIpsecTunnelAddress(this: &Self::This, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::Result<()>;
    fn GetIpsecSecurity(this: &Self::This, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows_core::Result<u64>;
    fn SetIpsecSecurity(this: &Self::This, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsIscsiPortal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsIscsiPortal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pportalprop)).into())
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAssociatedPortalGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecTunnelAddress(this, ::core::mem::transmute_copy(&ptunneladdress), ::core::mem::transmute_copy(&pdestinationaddress)).into())
        }
        unsafe extern "system" fn GetIpsecSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIpsecSecurity(this, ::core::mem::transmute_copy(&pinitiatorportaladdress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullsecurityflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpsecSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecSecurity(this, ::core::mem::transmute_copy(&pinitiatorportaladdress), ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into())
        }
        IVdsIscsiPortal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryAssociatedPortalGroups: QueryAssociatedPortalGroups::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Identity, Impl, OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Identity, Impl, OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsIscsiPortalGroup_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows_core::Result<()>;
    fn GetTarget(this: &Self::This) -> ::windows_core::Result<IVdsIscsiTarget>;
    fn QueryAssociatedPortals(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn AddPortal(this: &Self::This, portalid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn RemovePortal(this: &Self::This, portalid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<IVdsAsync>;
}
impl ::windows_core::Iids for IVdsIscsiPortalGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsIscsiPortalGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pportalgroupprop)).into())
        }
        unsafe extern "system" fn GetTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAssociatedPortals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAssociatedPortals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPortal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portalid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddPortal(this, ::core::mem::transmute(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePortal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portalid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemovePortal(this, ::core::mem::transmute(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsIscsiPortalGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetTarget: GetTarget::<Identity, Impl, OFFSET>,
            QueryAssociatedPortals: QueryAssociatedPortals::<Identity, Impl, OFFSET>,
            AddPortal: AddPortal::<Identity, Impl, OFFSET>,
            RemovePortal: RemovePortal::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsIscsiPortalLocal_Impl: ::windows_core::BaseImpl {
    fn SetIpsecSecurityLocal(this: &Self::This, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsIscsiPortalLocal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalLocal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsIscsiPortalLocal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIpsecSecurityLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiPortalLocal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecSecurityLocal(this, ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into())
        }
        IVdsIscsiPortalLocal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIpsecSecurityLocal: SetIpsecSecurityLocal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsIscsiTarget_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows_core::Result<()>;
    fn GetSubSystem(this: &Self::This) -> ::windows_core::Result<IVdsSubSystem>;
    fn QueryPortalGroups(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryAssociatedLuns(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreatePortalGroup(this: &Self::This) -> ::windows_core::Result<IVdsAsync>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<IVdsAsync>;
    fn SetFriendlyName(this: &Self::This, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetSharedSecret(this: &Self::This, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RememberInitiatorSharedSecret(this: &Self::This, pwszinitiatorname: &::windows_core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows_core::Result<()>;
    fn GetConnectedInitiators(this: &Self::This, pppwszinitiatorlist: *mut *mut ::windows_core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsIscsiTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsIscsiTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&ptargetprop)).into())
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryPortalGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryPortalGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAssociatedLuns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePortalGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePortalGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFriendlyName(this, ::core::mem::transmute(&pwszfriendlyname)).into())
        }
        unsafe extern "system" fn SetSharedSecret<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSharedSecret(this, ::core::mem::transmute_copy(&ptargetsharedsecret), ::core::mem::transmute(&pwszinitiatorname)).into())
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinitiatorname: ::windows_core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RememberInitiatorSharedSecret(this, ::core::mem::transmute(&pwszinitiatorname), ::core::mem::transmute_copy(&pinitiatorsharedsecret)).into())
        }
        unsafe extern "system" fn GetConnectedInitiators<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut ::windows_core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectedInitiators(this, ::core::mem::transmute_copy(&pppwszinitiatorlist), ::core::mem::transmute_copy(&plnumberofinitiators)).into())
        }
        IVdsIscsiTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryPortalGroups: QueryPortalGroups::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            CreatePortalGroup: CreatePortalGroup::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
            SetSharedSecret: SetSharedSecret::<Identity, Impl, OFFSET>,
            RememberInitiatorSharedSecret: RememberInitiatorSharedSecret::<Identity, Impl, OFFSET>,
            GetConnectedInitiators: GetConnectedInitiators::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, plunprop: *mut VDS_LUN_PROP) -> ::windows_core::Result<()>;
    fn GetSubSystem(this: &Self::This) -> ::windows_core::Result<IVdsSubSystem>;
    fn GetIdentificationData(this: &Self::This, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows_core::Result<()>;
    fn QueryActiveControllers(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn Extend(this: &Self::This, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32) -> ::windows_core::Result<IVdsAsync>;
    fn Shrink(this: &Self::This, ullnumberofbytestoremove: u64) -> ::windows_core::Result<IVdsAsync>;
    fn QueryPlexes(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn AddPlex(this: &Self::This, lunid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn RemovePlex(this: &Self::This, plexid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn Recover(this: &Self::This) -> ::windows_core::Result<IVdsAsync>;
    fn SetMask(this: &Self::This, pwszunmaskinglist: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn AssociateControllers(this: &Self::This, pactivecontrolleridarray: *const ::windows_core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows_core::GUID, lnumberofinactivecontrollers: i32) -> ::windows_core::Result<()>;
    fn QueryHints(this: &Self::This, phints: *mut VDS_HINTS) -> ::windows_core::Result<()>;
    fn ApplyHints(this: &Self::This, phints: *const VDS_HINTS) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, status: VDS_LUN_STATUS) -> ::windows_core::Result<()>;
    fn QueryMaxLunExtendSize(this: &Self::This, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsLun {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLun {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&plunprop)).into())
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIdentificationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdentificationData(this, ::core::mem::transmute_copy(&pluninfo)).into())
        }
        unsafe extern "system" fn QueryActiveControllers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryActiveControllers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extend(this, ::core::mem::transmute_copy(&ullnumberofbytestoadd), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shrink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shrink(this, ::core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryPlexes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryPlexes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPlex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lunid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddPlex(this, ::core::mem::transmute(&lunid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePlex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plexid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemovePlex(this, ::core::mem::transmute(&plexid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Recover<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recover(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszunmaskinglist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMask(this, ::core::mem::transmute(&pwszunmaskinglist)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn AssociateControllers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows_core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows_core::GUID, lnumberofinactivecontrollers: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateControllers(this, ::core::mem::transmute_copy(&pactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofactivecontrollers), ::core::mem::transmute_copy(&pinactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollers)).into())
        }
        unsafe extern "system" fn QueryHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryHints(this, ::core::mem::transmute_copy(&phints)).into())
        }
        unsafe extern "system" fn ApplyHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyHints(this, ::core::mem::transmute_copy(&phints)).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaxLunExtendSize(this, ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxbytestobeadded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsLun_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            GetIdentificationData: GetIdentificationData::<Identity, Impl, OFFSET>,
            QueryActiveControllers: QueryActiveControllers::<Identity, Impl, OFFSET>,
            Extend: Extend::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
            QueryPlexes: QueryPlexes::<Identity, Impl, OFFSET>,
            AddPlex: AddPlex::<Identity, Impl, OFFSET>,
            RemovePlex: RemovePlex::<Identity, Impl, OFFSET>,
            Recover: Recover::<Identity, Impl, OFFSET>,
            SetMask: SetMask::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            AssociateControllers: AssociateControllers::<Identity, Impl, OFFSET>,
            QueryHints: QueryHints::<Identity, Impl, OFFSET>,
            ApplyHints: ApplyHints::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            QueryMaxLunExtendSize: QueryMaxLunExtendSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun2_Impl: ::windows_core::BaseImpl {
    fn QueryHints2(this: &Self::This, phints2: *mut VDS_HINTS2) -> ::windows_core::Result<()>;
    fn ApplyHints2(this: &Self::This, phints2: *const VDS_HINTS2) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsLun2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLun2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryHints2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryHints2(this, ::core::mem::transmute_copy(&phints2)).into())
        }
        unsafe extern "system" fn ApplyHints2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLun2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyHints2(this, ::core::mem::transmute_copy(&phints2)).into())
        }
        IVdsLun2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryHints2: QueryHints2::<Identity, Impl, OFFSET>,
            ApplyHints2: ApplyHints2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsLunControllerPorts_Impl: ::windows_core::BaseImpl {
    fn AssociateControllerPorts(this: &Self::This, pactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows_core::Result<()>;
    fn QueryActiveControllerPorts(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::Iids for IVdsLunControllerPorts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunControllerPorts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLunControllerPorts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssociateControllerPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunControllerPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateControllerPorts(this, ::core::mem::transmute_copy(&pactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofactivecontrollerports), ::core::mem::transmute_copy(&pinactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollerports)).into())
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunControllerPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryActiveControllerPorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsLunControllerPorts_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssociateControllerPorts: AssociateControllerPorts::<Identity, Impl, OFFSET>,
            QueryActiveControllerPorts: QueryActiveControllerPorts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsLunIscsi_Impl: ::windows_core::BaseImpl {
    fn AssociateTargets(this: &Self::This, ptargetidarray: *const ::windows_core::GUID, lnumberoftargets: i32) -> ::windows_core::Result<()>;
    fn QueryAssociatedTargets(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::Iids for IVdsLunIscsi {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunIscsi_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLunIscsi {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssociateTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows_core::GUID, lnumberoftargets: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateTargets(this, ::core::mem::transmute_copy(&ptargetidarray), ::core::mem::transmute_copy(&lnumberoftargets)).into())
        }
        unsafe extern "system" fn QueryAssociatedTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAssociatedTargets(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsLunIscsi_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssociateTargets: AssociateTargets::<Identity, Impl, OFFSET>,
            QueryAssociatedTargets: QueryAssociatedTargets::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunMpio_Impl: ::windows_core::BaseImpl {
    fn GetPathInfo(this: &Self::This, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows_core::Result<()>;
    fn GetLoadBalancePolicy(this: &Self::This, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows_core::Result<()>;
    fn SetLoadBalancePolicy(this: &Self::This, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows_core::Result<()>;
    fn GetSupportedLbPolicies(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsLunMpio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLunMpio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPathInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPathInfo(this, ::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into())
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLoadBalancePolicy(this, ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into())
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoadBalancePolicy(this, ::core::mem::transmute_copy(&policy), ::core::mem::transmute_copy(&ppaths), ::core::mem::transmute_copy(&lnumberofpaths)).into())
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedLbPolicies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullbflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsLunMpio_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPathInfo: GetPathInfo::<Identity, Impl, OFFSET>,
            GetLoadBalancePolicy: GetLoadBalancePolicy::<Identity, Impl, OFFSET>,
            SetLoadBalancePolicy: SetLoadBalancePolicy::<Identity, Impl, OFFSET>,
            GetSupportedLbPolicies: GetSupportedLbPolicies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsLunNaming_Impl: ::windows_core::BaseImpl {
    fn SetFriendlyName(this: &Self::This, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsLunNaming {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunNaming_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLunNaming {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunNaming_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFriendlyName(this, ::core::mem::transmute(&pwszfriendlyname)).into())
        }
        IVdsLunNaming_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsLunNumber_Impl: ::windows_core::BaseImpl {
    fn GetLunNumber(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IVdsLunNumber {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunNumber_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLunNumber {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLunNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunNumber_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLunNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullunnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsLunNumber_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetLunNumber: GetLunNumber::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunPlex_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows_core::Result<()>;
    fn GetLun(this: &Self::This) -> ::windows_core::Result<IVdsLun>;
    fn QueryExtents(this: &Self::This, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn QueryHints(this: &Self::This, phints: *mut VDS_HINTS) -> ::windows_core::Result<()>;
    fn ApplyHints(this: &Self::This, phints: *const VDS_HINTS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsLunPlex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsLunPlex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pplexprop)).into())
        }
        unsafe extern "system" fn GetLun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplun: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLun(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryExtents(this, ::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into())
        }
        unsafe extern "system" fn QueryHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryHints(this, ::core::mem::transmute_copy(&phints)).into())
        }
        unsafe extern "system" fn ApplyHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyHints(this, ::core::mem::transmute_copy(&phints)).into())
        }
        IVdsLunPlex_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetLun: GetLun::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            QueryHints: QueryHints::<Identity, Impl, OFFSET>,
            ApplyHints: ApplyHints::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsMaintenance_Impl: ::windows_core::BaseImpl {
    fn StartMaintenance(this: &Self::This, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::Result<()>;
    fn StopMaintenance(this: &Self::This, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::Result<()>;
    fn PulseMaintenance(this: &Self::This, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsMaintenance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsMaintenance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartMaintenance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartMaintenance(this, ::core::mem::transmute_copy(&operation)).into())
        }
        unsafe extern "system" fn StopMaintenance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopMaintenance(this, ::core::mem::transmute_copy(&operation)).into())
        }
        unsafe extern "system" fn PulseMaintenance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PulseMaintenance(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&ulcount)).into())
        }
        IVdsMaintenance_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartMaintenance: StartMaintenance::<Identity, Impl, OFFSET>,
            StopMaintenance: StopMaintenance::<Identity, Impl, OFFSET>,
            PulseMaintenance: PulseMaintenance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Vhd\"`"]
#[cfg(feature = "Win32_Storage_Vhd")]
pub trait IVdsOpenVDisk_Impl: ::windows_core::BaseImpl {
    fn Attach(this: &Self::This, pstringsecuritydescriptor: &::windows_core::PCWSTR, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32) -> ::windows_core::Result<IVdsAsync>;
    fn Detach(this: &Self::This, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows_core::Result<()>;
    fn DetachAndDelete(this: &Self::This, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows_core::Result<()>;
    fn Compact(this: &Self::This, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32) -> ::windows_core::Result<IVdsAsync>;
    fn Merge(this: &Self::This, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32) -> ::windows_core::Result<IVdsAsync>;
    fn Expand(this: &Self::This, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::windows_core::Iids for IVdsOpenVDisk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsOpenVDisk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstringsecuritydescriptor: ::windows_core::PCWSTR, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attach(this, ::core::mem::transmute(&pstringsecuritydescriptor), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&providerspecificflags), ::core::mem::transmute_copy(&timeoutinms)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Detach(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&providerspecificflags)).into())
        }
        unsafe extern "system" fn DetachAndDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetachAndDelete(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&providerspecificflags)).into())
        }
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compact(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Merge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Merge(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&mergedepth)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsOpenVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Expand(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&newsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsOpenVDisk_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            DetachAndDelete: DetachAndDelete::<Identity, Impl, OFFSET>,
            Compact: Compact::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsPack_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, ppackprop: *mut VDS_PACK_PROP) -> ::windows_core::Result<()>;
    fn GetProvider(this: &Self::This) -> ::windows_core::Result<IVdsProvider>;
    fn QueryVolumes(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryDisks(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreateVolume(this: &Self::This, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32) -> ::windows_core::Result<IVdsAsync>;
    fn AddDisk(this: &Self::This, diskid: &::windows_core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn MigrateDisks(this: &Self::This, pdiskarray: *const ::windows_core::GUID, lnumberofdisks: i32, targetpack: &::windows_core::GUID, bforce: super::super::Foundation::BOOL, bqueryonly: super::super::Foundation::BOOL, presults: *mut ::windows_core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ReplaceDisk(this: &Self::This, olddiskid: &::windows_core::GUID, newdiskid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn RemoveMissingDisk(this: &Self::This, diskid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Recover(this: &Self::This) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsPack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsPack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppackprop: *mut VDS_PACK_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&ppackprop)).into())
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryVolumes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDisks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVolume(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pinputdiskarray), ::core::mem::transmute_copy(&lnumberofdisks), ::core::mem::transmute_copy(&ulstripesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, diskid: ::windows_core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDisk(this, ::core::mem::transmute(&diskid), ::core::mem::transmute_copy(&partitionstyle), ::core::mem::transmute_copy(&bashotspare)).into())
        }
        unsafe extern "system" fn MigrateDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiskarray: *const ::windows_core::GUID, lnumberofdisks: i32, targetpack: ::windows_core::GUID, bforce: super::super::Foundation::BOOL, bqueryonly: super::super::Foundation::BOOL, presults: *mut ::windows_core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MigrateDisks(this, ::core::mem::transmute_copy(&pdiskarray), ::core::mem::transmute_copy(&lnumberofdisks), ::core::mem::transmute(&targetpack), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bqueryonly), ::core::mem::transmute_copy(&presults), ::core::mem::transmute_copy(&pbrebootneeded)).into())
        }
        unsafe extern "system" fn ReplaceDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olddiskid: ::windows_core::GUID, newdiskid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplaceDisk(this, ::core::mem::transmute(&olddiskid), ::core::mem::transmute(&newdiskid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveMissingDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, diskid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMissingDisk(this, ::core::mem::transmute(&diskid)).into())
        }
        unsafe extern "system" fn Recover<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recover(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsPack_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            QueryVolumes: QueryVolumes::<Identity, Impl, OFFSET>,
            QueryDisks: QueryDisks::<Identity, Impl, OFFSET>,
            CreateVolume: CreateVolume::<Identity, Impl, OFFSET>,
            AddDisk: AddDisk::<Identity, Impl, OFFSET>,
            MigrateDisks: MigrateDisks::<Identity, Impl, OFFSET>,
            ReplaceDisk: ReplaceDisk::<Identity, Impl, OFFSET>,
            RemoveMissingDisk: RemoveMissingDisk::<Identity, Impl, OFFSET>,
            Recover: Recover::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsPack2_Impl: ::windows_core::BaseImpl {
    fn CreateVolume2(this: &Self::This, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ulalign: u32) -> ::windows_core::Result<IVdsAsync>;
}
impl ::windows_core::Iids for IVdsPack2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsPack2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVolume2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsPack2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ulalign: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVolume2(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pinputdiskarray), ::core::mem::transmute_copy(&lnumberofdisks), ::core::mem::transmute_copy(&ulstripesize), ::core::mem::transmute_copy(&ulalign)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsPack2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateVolume2: CreateVolume2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsProvider_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pproviderprop)).into())
        }
        IVdsProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetProperties: GetProperties::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProviderPrivate_Impl: ::windows_core::BaseImpl {
    fn GetObject(this: &Self::This, objectid: &::windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnLoad(this: &Self::This, pwszmachinename: &::windows_core::PCWSTR, pcallbackobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnUnload(this: &Self::This, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsProviderPrivate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsProviderPrivate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: ::windows_core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this, ::core::mem::transmute(&objectid), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows_core::PCWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLoad(this, ::core::mem::transmute(&pwszmachinename), ::windows_core::from_raw_borrowed(&pcallbackobject)).into())
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUnload(this, ::core::mem::transmute_copy(&bforceunload)).into())
        }
        IVdsProviderPrivate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsProviderSupport_Impl: ::windows_core::BaseImpl {
    fn GetVersionSupport(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IVdsProviderSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProviderSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsProviderSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVersionSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsProviderSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersionSupport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ulversionsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsProviderSupport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVersionSupport: GetVersionSupport::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsRemovable_Impl: ::windows_core::BaseImpl {
    fn QueryMedia(this: &Self::This) -> ::windows_core::Result<()>;
    fn Eject(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsRemovable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsRemovable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsRemovable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsRemovable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryMedia(this).into())
        }
        unsafe extern "system" fn Eject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsRemovable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Eject(this).into())
        }
        IVdsRemovable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryMedia: QueryMedia::<Identity, Impl, OFFSET>,
            Eject: Eject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsService_Impl: ::windows_core::BaseImpl {
    fn IsServiceReady(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForServiceReady(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProperties(this: &Self::This) -> ::windows_core::Result<VDS_SERVICE_PROP>;
    fn QueryProviders(this: &Self::This, masks: u32) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryMaskedDisks(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryUnallocatedDisks(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn GetObject(this: &Self::This, objectid: &::windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryDriveLetters(this: &Self::This, wcfirstletter: u16, count: u32, pdriveletterproparray: *mut VDS_DRIVE_LETTER_PROP) -> ::windows_core::Result<()>;
    fn QueryFileSystemTypes(this: &Self::This, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> ::windows_core::Result<()>;
    fn Reenumerate(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn CleanupObsoleteMountPoints(this: &Self::This) -> ::windows_core::Result<()>;
    fn Advise(this: &Self::This, psink: ::core::option::Option<&IVdsAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn Reboot(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn ClearFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsServiceReady<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsServiceReady(this).into())
        }
        unsafe extern "system" fn WaitForServiceReady<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForServiceReady(this).into())
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserviceprop: *mut VDS_SERVICE_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pserviceprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, masks: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryProviders(this, ::core::mem::transmute_copy(&masks)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryMaskedDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaskedDisks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryUnallocatedDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryUnallocatedDisks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: ::windows_core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this, ::core::mem::transmute(&objectid), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryDriveLetters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wcfirstletter: u16, count: u32, pdriveletterproparray: *mut VDS_DRIVE_LETTER_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryDriveLetters(this, ::core::mem::transmute_copy(&wcfirstletter), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pdriveletterproparray)).into())
        }
        unsafe extern "system" fn QueryFileSystemTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryFileSystemTypes(this, ::core::mem::transmute_copy(&ppfilesystemtypeprops), ::core::mem::transmute_copy(&plnumberoffilesystems)).into())
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reenumerate(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn CleanupObsoleteMountPoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CleanupObsoleteMountPoints(this).into())
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn Reboot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reboot(this).into())
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ClearFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        IVdsService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsServiceReady: IsServiceReady::<Identity, Impl, OFFSET>,
            WaitForServiceReady: WaitForServiceReady::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            QueryProviders: QueryProviders::<Identity, Impl, OFFSET>,
            QueryMaskedDisks: QueryMaskedDisks::<Identity, Impl, OFFSET>,
            QueryUnallocatedDisks: QueryUnallocatedDisks::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            QueryDriveLetters: QueryDriveLetters::<Identity, Impl, OFFSET>,
            QueryFileSystemTypes: QueryFileSystemTypes::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            CleanupObsoleteMountPoints: CleanupObsoleteMountPoints::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            Reboot: Reboot::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ClearFlags: ClearFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsServiceHba_Impl: ::windows_core::BaseImpl {
    fn QueryHbaPorts(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::Iids for IVdsServiceHba {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceHba_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceHba {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryHbaPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceHba_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHbaPorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsServiceHba_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, QueryHbaPorts: QueryHbaPorts::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsServiceInitialization_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pwszmachinename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsServiceInitialization {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceInitialization_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceInitialization {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceInitialization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pwszmachinename)).into())
        }
        IVdsServiceInitialization_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsServiceIscsi_Impl: ::windows_core::BaseImpl {
    fn GetInitiatorName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn QueryInitiatorAdapters(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn SetIpsecGroupPresharedKey(this: &Self::This, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
    fn SetAllIpsecTunnelAddresses(this: &Self::This, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::Result<()>;
    fn SetAllIpsecSecurity(this: &Self::This, targetportalid: &::windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
    fn SetInitiatorSharedSecret(this: &Self::This, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RememberTargetSharedSecret(this: &Self::This, targetid: &::windows_core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsServiceIscsi {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceIscsi {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInitiatorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwsziscsiname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInitiatorName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwsziscsiname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInitiatorAdapters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryInitiatorAdapters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecGroupPresharedKey(this, ::core::mem::transmute_copy(&pipseckey)).into())
        }
        unsafe extern "system" fn SetAllIpsecTunnelAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllIpsecTunnelAddresses(this, ::core::mem::transmute_copy(&ptunneladdress), ::core::mem::transmute_copy(&pdestinationaddress)).into())
        }
        unsafe extern "system" fn SetAllIpsecSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetportalid: ::windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllIpsecSecurity(this, ::core::mem::transmute(&targetportalid), ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into())
        }
        unsafe extern "system" fn SetInitiatorSharedSecret<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitiatorSharedSecret(this, ::core::mem::transmute_copy(&pinitiatorsharedsecret), ::core::mem::transmute(&targetid)).into())
        }
        unsafe extern "system" fn RememberTargetSharedSecret<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: ::windows_core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RememberTargetSharedSecret(this, ::core::mem::transmute(&targetid), ::core::mem::transmute_copy(&ptargetsharedsecret)).into())
        }
        IVdsServiceIscsi_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInitiatorName: GetInitiatorName::<Identity, Impl, OFFSET>,
            QueryInitiatorAdapters: QueryInitiatorAdapters::<Identity, Impl, OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Identity, Impl, OFFSET>,
            SetAllIpsecTunnelAddresses: SetAllIpsecTunnelAddresses::<Identity, Impl, OFFSET>,
            SetAllIpsecSecurity: SetAllIpsecSecurity::<Identity, Impl, OFFSET>,
            SetInitiatorSharedSecret: SetInitiatorSharedSecret::<Identity, Impl, OFFSET>,
            RememberTargetSharedSecret: RememberTargetSharedSecret::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsServiceLoader_Impl: ::windows_core::BaseImpl {
    fn LoadService(this: &Self::This, pwszmachinename: &::windows_core::PCWSTR) -> ::windows_core::Result<IVdsService>;
}
impl ::windows_core::Iids for IVdsServiceLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadService(this, ::core::mem::transmute(&pwszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsServiceLoader_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LoadService: LoadService::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsServiceSAN_Impl: ::windows_core::BaseImpl {
    fn GetSANPolicy(this: &Self::This) -> ::windows_core::Result<VDS_SAN_POLICY>;
    fn SetSANPolicy(this: &Self::This, sanpolicy: VDS_SAN_POLICY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsServiceSAN {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceSAN_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceSAN {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSANPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceSAN_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psanpolicy: *mut VDS_SAN_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSANPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psanpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSANPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceSAN_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sanpolicy: VDS_SAN_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSANPolicy(this, ::core::mem::transmute_copy(&sanpolicy)).into())
        }
        IVdsServiceSAN_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSANPolicy: GetSANPolicy::<Identity, Impl, OFFSET>,
            SetSANPolicy: SetSANPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsServiceSw_Impl: ::windows_core::BaseImpl {
    fn GetDiskObject(this: &Self::This, pwszdeviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IVdsServiceSw {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceSw_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceSw {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDiskObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceSw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdeviceid: ::windows_core::PCWSTR, ppdiskunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDiskObject(this, ::core::mem::transmute(&pwszdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdiskunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsServiceSw_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDiskObject: GetDiskObject::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsServiceUninstallDisk_Impl: ::windows_core::BaseImpl {
    fn GetDiskIdFromLunInfo(this: &Self::This, pluninfo: *const VDS_LUN_INFORMATION) -> ::windows_core::Result<::windows_core::GUID>;
    fn UninstallDisks(this: &Self::This, pdiskidarray: *const ::windows_core::GUID, ulcount: u32, bforce: super::super::Foundation::BOOLEAN, pbreboot: *mut u8, presults: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsServiceUninstallDisk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceUninstallDisk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsServiceUninstallDisk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDiskIdFromLunInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceUninstallDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pluninfo: *const VDS_LUN_INFORMATION, pdiskid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDiskIdFromLunInfo(this, ::core::mem::transmute_copy(&pluninfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdiskid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsServiceUninstallDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiskidarray: *const ::windows_core::GUID, ulcount: u32, bforce: super::super::Foundation::BOOLEAN, pbreboot: *mut u8, presults: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UninstallDisks(this, ::core::mem::transmute_copy(&pdiskidarray), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&pbreboot), ::core::mem::transmute_copy(&presults)).into())
        }
        IVdsServiceUninstallDisk_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDiskIdFromLunInfo: GetDiskIdFromLunInfo::<Identity, Impl, OFFSET>,
            UninstallDisks: UninstallDisks::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsStoragePool_Impl: ::windows_core::BaseImpl {
    fn GetProvider(this: &Self::This) -> ::windows_core::Result<IVdsProvider>;
    fn GetProperties(this: &Self::This, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows_core::Result<()>;
    fn GetAttributes(this: &Self::This, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows_core::Result<()>;
    fn QueryDriveExtents(this: &Self::This, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn QueryAllocatedLuns(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryAllocatedStoragePools(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsStoragePool {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsStoragePool {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pstoragepoolprop)).into())
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributes(this, ::core::mem::transmute_copy(&pstoragepoolattributes)).into())
        }
        unsafe extern "system" fn QueryDriveExtents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryDriveExtents(this, ::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into())
        }
        unsafe extern "system" fn QueryAllocatedLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAllocatedLuns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAllocatedStoragePools(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsStoragePool_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            QueryDriveExtents: QueryDriveExtents::<Identity, Impl, OFFSET>,
            QueryAllocatedLuns: QueryAllocatedLuns::<Identity, Impl, OFFSET>,
            QueryAllocatedStoragePools: QueryAllocatedStoragePools::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows_core::Result<()>;
    fn GetProvider(this: &Self::This) -> ::windows_core::Result<IVdsProvider>;
    fn QueryControllers(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryLuns(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryDrives(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn GetDrive(this: &Self::This, sbusnumber: i16, sslotnumber: i16) -> ::windows_core::Result<IVdsDrive>;
    fn Reenumerate(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetControllerStatus(this: &Self::This, ponlinecontrolleridarray: *const ::windows_core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows_core::GUID, lnumberofofflinecontrollers: i32) -> ::windows_core::Result<()>;
    fn CreateLun(this: &Self::This, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &::windows_core::PCWSTR, phints: *const VDS_HINTS) -> ::windows_core::Result<IVdsAsync>;
    fn ReplaceDrive(this: &Self::This, drivetobereplaced: &::windows_core::GUID, replacementdrive: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, status: VDS_SUB_SYSTEM_STATUS) -> ::windows_core::Result<()>;
    fn QueryMaxLunCreateSize(this: &Self::This, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsSubSystem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSubSystem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&psubsystemprop)).into())
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryControllers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryControllers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryLuns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryDrives<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDrives(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDrive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDrive(this, ::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdrive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reenumerate(this).into())
        }
        unsafe extern "system" fn SetControllerStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows_core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows_core::GUID, lnumberofofflinecontrollers: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControllerStatus(this, ::core::mem::transmute_copy(&ponlinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofonlinecontrollers), ::core::mem::transmute_copy(&pofflinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofofflinecontrollers)).into())
        }
        unsafe extern "system" fn CreateLun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows_core::PCWSTR, phints: *const VDS_HINTS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLun(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReplaceDrive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows_core::GUID, replacementdrive: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceDrive(this, ::core::mem::transmute(&drivetobereplaced), ::core::mem::transmute(&replacementdrive)).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaxLunCreateSize(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxlunsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsSubSystem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            QueryControllers: QueryControllers::<Identity, Impl, OFFSET>,
            QueryLuns: QueryLuns::<Identity, Impl, OFFSET>,
            QueryDrives: QueryDrives::<Identity, Impl, OFFSET>,
            GetDrive: GetDrive::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            SetControllerStatus: SetControllerStatus::<Identity, Impl, OFFSET>,
            CreateLun: CreateLun::<Identity, Impl, OFFSET>,
            ReplaceDrive: ReplaceDrive::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSize: QueryMaxLunCreateSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem2_Impl: ::windows_core::BaseImpl {
    fn GetProperties2(this: &Self::This, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows_core::Result<()>;
    fn GetDrive2(this: &Self::This, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows_core::Result<IVdsDrive>;
    fn CreateLun2(this: &Self::This, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &::windows_core::PCWSTR, phints2: *const VDS_HINTS2) -> ::windows_core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSize2(this: &Self::This, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsSubSystem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSubSystem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties2(this, ::core::mem::transmute_copy(&psubsystemprop2)).into())
        }
        unsafe extern "system" fn GetDrive2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDrive2(this, ::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber), ::core::mem::transmute_copy(&ulenclosurenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdrive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLun2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows_core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLun2(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaxLunCreateSize2(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxlunsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsSubSystem2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties2: GetProperties2::<Identity, Impl, OFFSET>,
            GetDrive2: GetDrive2::<Identity, Impl, OFFSET>,
            CreateLun2: CreateLun2::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSize2: QueryMaxLunCreateSize2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsSubSystemImportTarget_Impl: ::windows_core::BaseImpl {
    fn GetImportTarget(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetImportTarget(this: &Self::This, pwsziscsiname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsSubSystemImportTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemImportTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSubSystemImportTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImportTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemImportTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwsziscsiname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImportTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwsziscsiname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImportTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemImportTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsziscsiname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImportTarget(this, ::core::mem::transmute(&pwsziscsiname)).into())
        }
        IVdsSubSystemImportTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImportTarget: GetImportTarget::<Identity, Impl, OFFSET>,
            SetImportTarget: SetImportTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsSubSystemInterconnect_Impl: ::windows_core::BaseImpl {
    fn GetSupportedInterconnects(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IVdsSubSystemInterconnect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSubSystemInterconnect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedInterconnects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedInterconnects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulsupportedinterconnectsflag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsSubSystemInterconnect_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedInterconnects: GetSupportedInterconnects::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsSubSystemIscsi_Impl: ::windows_core::BaseImpl {
    fn QueryTargets(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryPortals(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreateTarget(this: &Self::This, pwsziscsiname: &::windows_core::PCWSTR, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<IVdsAsync>;
    fn SetIpsecGroupPresharedKey(this: &Self::This, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsSubSystemIscsi {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSubSystemIscsi {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryTargets(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryPortals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryPortals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsziscsiname: ::windows_core::PCWSTR, pwszfriendlyname: ::windows_core::PCWSTR, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTarget(this, ::core::mem::transmute(&pwsziscsiname), ::core::mem::transmute(&pwszfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpsecGroupPresharedKey(this, ::core::mem::transmute_copy(&pipseckey)).into())
        }
        IVdsSubSystemIscsi_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryTargets: QueryTargets::<Identity, Impl, OFFSET>,
            QueryPortals: QueryPortals::<Identity, Impl, OFFSET>,
            CreateTarget: CreateTarget::<Identity, Impl, OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsSubSystemNaming_Impl: ::windows_core::BaseImpl {
    fn SetFriendlyName(this: &Self::This, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsSubSystemNaming {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemNaming_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSubSystemNaming {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSubSystemNaming_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFriendlyName(this, ::core::mem::transmute(&pwszfriendlyname)).into())
        }
        IVdsSubSystemNaming_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsSwProvider_Impl: ::windows_core::BaseImpl {
    fn QueryPacks(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreatePack(this: &Self::This) -> ::windows_core::Result<IVdsPack>;
}
impl ::windows_core::Iids for IVdsSwProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSwProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsSwProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryPacks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSwProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryPacks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsSwProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppack: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsSwProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryPacks: QueryPacks::<Identity, Impl, OFFSET>,
            CreatePack: CreatePack::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Vhd\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
pub trait IVdsVDisk_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32) -> ::windows_core::Result<IVdsOpenVDisk>;
    fn GetProperties(this: &Self::This, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> ::windows_core::Result<()>;
    fn GetHostVolume(this: &Self::This) -> ::windows_core::Result<IVdsVolume>;
    fn GetDeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::windows_core::Iids for IVdsVDisk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVDisk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVDisk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32, ppopenvdisk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this, ::core::mem::transmute_copy(&accessmask), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&readwritedepth)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppopenvdisk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pdiskproperties)).into())
        }
        unsafe extern "system" fn GetHostVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvolume: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHostVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevicename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsVDisk_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetHostVolume: GetHostVolume::<Identity, Impl, OFFSET>,
            GetDeviceName: GetDeviceName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Vhd\"`"]
#[cfg(feature = "Win32_Storage_Vhd")]
pub trait IVdsVdProvider_Impl: ::windows_core::BaseImpl {
    fn QueryVDisks(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreateVDisk(this: &Self::This, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: &::windows_core::PCWSTR, pstringsecuritydescriptor: &::windows_core::PCWSTR, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: *mut ::core::option::Option<IVdsAsync>) -> ::windows_core::Result<()>;
    fn AddVDisk(this: &Self::This, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: &::windows_core::PCWSTR, ppvdisk: *mut ::core::option::Option<IVdsVDisk>) -> ::windows_core::Result<()>;
    fn GetDiskFromVDisk(this: &Self::This, pvdisk: ::core::option::Option<&IVdsVDisk>) -> ::windows_core::Result<IVdsDisk>;
    fn GetVDiskFromDisk(this: &Self::This, pdisk: ::core::option::Option<&IVdsDisk>) -> ::windows_core::Result<IVdsVDisk>;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::windows_core::Iids for IVdsVdProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVdProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVdProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryVDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVdProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryVDisks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVdProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: ::windows_core::PCWSTR, pstringsecuritydescriptor: ::windows_core::PCWSTR, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVDisk(this, ::core::mem::transmute_copy(&virtualdevicetype), ::core::mem::transmute(&ppath), ::core::mem::transmute(&pstringsecuritydescriptor), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&providerspecificflags), ::core::mem::transmute_copy(&reserved), ::core::mem::transmute_copy(&pcreatediskparameters), ::core::mem::transmute_copy(&ppasync)).into())
        }
        unsafe extern "system" fn AddVDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVdProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: ::windows_core::PCWSTR, ppvdisk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddVDisk(this, ::core::mem::transmute_copy(&virtualdevicetype), ::core::mem::transmute(&ppath), ::core::mem::transmute_copy(&ppvdisk)).into())
        }
        unsafe extern "system" fn GetDiskFromVDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVdProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdisk: *mut ::core::ffi::c_void, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDiskFromVDisk(this, ::windows_core::from_raw_borrowed(&pvdisk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVDiskFromDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVdProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisk: *mut ::core::ffi::c_void, ppvdisk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVDiskFromDisk(this, ::windows_core::from_raw_borrowed(&pdisk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvdisk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsVdProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryVDisks: QueryVDisks::<Identity, Impl, OFFSET>,
            CreateVDisk: CreateVDisk::<Identity, Impl, OFFSET>,
            AddVDisk: AddVDisk::<Identity, Impl, OFFSET>,
            GetDiskFromVDisk: GetDiskFromVDisk::<Identity, Impl, OFFSET>,
            GetVDiskFromDisk: GetVDiskFromDisk::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsVolume_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pvolumeproperties: *mut VDS_VOLUME_PROP) -> ::windows_core::Result<()>;
    fn GetPack(this: &Self::This) -> ::windows_core::Result<IVdsPack>;
    fn QueryPlexes(this: &Self::This) -> ::windows_core::Result<IEnumVdsObject>;
    fn Extend(this: &Self::This, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32) -> ::windows_core::Result<IVdsAsync>;
    fn Shrink(this: &Self::This, ullnumberofbytestoremove: u64) -> ::windows_core::Result<IVdsAsync>;
    fn AddPlex(this: &Self::This, volumeid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn BreakPlex(this: &Self::This, plexid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn RemovePlex(this: &Self::This, plexid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn Delete(this: &Self::This, bforce: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetFlags(this: &Self::This, ulflags: u32, brevertonclose: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ClearFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsVolume {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolume {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvolumeproperties: *mut VDS_VOLUME_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pvolumeproperties)).into())
        }
        unsafe extern "system" fn GetPack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppack: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryPlexes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryPlexes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extend(this, ::core::mem::transmute_copy(&pinputdiskarray), ::core::mem::transmute_copy(&lnumberofdisks)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shrink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shrink(this, ::core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPlex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volumeid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddPlex(this, ::core::mem::transmute(&volumeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BreakPlex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plexid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BreakPlex(this, ::core::mem::transmute(&plexid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePlex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plexid: ::windows_core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemovePlex(this, ::core::mem::transmute(&plexid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bforce: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&bforce)).into())
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, brevertonclose: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&brevertonclose)).into())
        }
        unsafe extern "system" fn ClearFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        IVdsVolume_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPack: GetPack::<Identity, Impl, OFFSET>,
            QueryPlexes: QueryPlexes::<Identity, Impl, OFFSET>,
            Extend: Extend::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
            AddPlex: AddPlex::<Identity, Impl, OFFSET>,
            BreakPlex: BreakPlex::<Identity, Impl, OFFSET>,
            RemovePlex: RemovePlex::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ClearFlags: ClearFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsVolume2_Impl: ::windows_core::BaseImpl {
    fn GetProperties2(this: &Self::This, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsVolume2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolume2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolume2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties2(this, ::core::mem::transmute_copy(&pvolumeproperties)).into())
        }
        IVdsVolume2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetProperties2: GetProperties2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsVolumeMF_Impl: ::windows_core::BaseImpl {
    fn GetFileSystemProperties(this: &Self::This, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows_core::Result<()>;
    fn Format(this: &Self::This, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: &::windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> ::windows_core::Result<IVdsAsync>;
    fn AddAccessPath(this: &Self::This, pwszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn QueryAccessPaths(this: &Self::This, pwszpatharray: *mut *mut ::windows_core::PWSTR, plnumberofaccesspaths: *mut i32) -> ::windows_core::Result<()>;
    fn QueryReparsePoints(this: &Self::This, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> ::windows_core::Result<()>;
    fn DeleteAccessPath(this: &Self::This, pwszpath: &::windows_core::PCWSTR, bforce: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Mount(this: &Self::This) -> ::windows_core::Result<()>;
    fn Dismount(this: &Self::This, bforce: super::super::Foundation::BOOL, bpermanent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetFileSystemFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn ClearFileSystemFlags(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsVolumeMF {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolumeMF {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFileSystemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileSystemProperties(this, ::core::mem::transmute_copy(&pfilesystemprop)).into())
        }
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: ::windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Format(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwszlabel), ::core::mem::transmute_copy(&dwunitallocationsize), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bquickformat), ::core::mem::transmute_copy(&benablecompression)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddAccessPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAccessPath(this, ::core::mem::transmute(&pwszpath)).into())
        }
        unsafe extern "system" fn QueryAccessPaths<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpatharray: *mut *mut ::windows_core::PWSTR, plnumberofaccesspaths: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryAccessPaths(this, ::core::mem::transmute_copy(&pwszpatharray), ::core::mem::transmute_copy(&plnumberofaccesspaths)).into())
        }
        unsafe extern "system" fn QueryReparsePoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryReparsePoints(this, ::core::mem::transmute_copy(&ppreparsepointprops), ::core::mem::transmute_copy(&plnumberofreparsepointprops)).into())
        }
        unsafe extern "system" fn DeleteAccessPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpath: ::windows_core::PCWSTR, bforce: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAccessPath(this, ::core::mem::transmute(&pwszpath), ::core::mem::transmute_copy(&bforce)).into())
        }
        unsafe extern "system" fn Mount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Mount(this).into())
        }
        unsafe extern "system" fn Dismount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bforce: super::super::Foundation::BOOL, bpermanent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dismount(this, ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bpermanent)).into())
        }
        unsafe extern "system" fn SetFileSystemFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileSystemFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ClearFileSystemFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFileSystemFlags(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        IVdsVolumeMF_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFileSystemProperties: GetFileSystemProperties::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            AddAccessPath: AddAccessPath::<Identity, Impl, OFFSET>,
            QueryAccessPaths: QueryAccessPaths::<Identity, Impl, OFFSET>,
            QueryReparsePoints: QueryReparsePoints::<Identity, Impl, OFFSET>,
            DeleteAccessPath: DeleteAccessPath::<Identity, Impl, OFFSET>,
            Mount: Mount::<Identity, Impl, OFFSET>,
            Dismount: Dismount::<Identity, Impl, OFFSET>,
            SetFileSystemFlags: SetFileSystemFlags::<Identity, Impl, OFFSET>,
            ClearFileSystemFlags: ClearFileSystemFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsVolumeMF2_Impl: ::windows_core::BaseImpl {
    fn GetFileSystemTypeName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn QueryFileSystemFormatSupport(this: &Self::This, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows_core::Result<()>;
    fn FormatEx(this: &Self::This, pwszfilesystemtypename: &::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &::windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> ::windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVdsVolumeMF2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolumeMF2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFileSystemTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszfilesystemtypename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSystemTypeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszfilesystemtypename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryFileSystemFormatSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryFileSystemFormatSupport(this, ::core::mem::transmute_copy(&ppfilesystemsupportprops), ::core::mem::transmute_copy(&plnumberoffilesystems)).into())
        }
        unsafe extern "system" fn FormatEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilesystemtypename: ::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatEx(this, ::core::mem::transmute(&pwszfilesystemtypename), ::core::mem::transmute_copy(&usfilesystemrevision), ::core::mem::transmute_copy(&uldesiredunitallocationsize), ::core::mem::transmute(&pwszlabel), ::core::mem::transmute_copy(&bforce), ::core::mem::transmute_copy(&bquickformat), ::core::mem::transmute_copy(&benablecompression)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsVolumeMF2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFileSystemTypeName: GetFileSystemTypeName::<Identity, Impl, OFFSET>,
            QueryFileSystemFormatSupport: QueryFileSystemFormatSupport::<Identity, Impl, OFFSET>,
            FormatEx: FormatEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsVolumeMF3_Impl: ::windows_core::BaseImpl {
    fn QueryVolumeGuidPathnames(this: &Self::This, pwszpatharray: *mut *mut ::windows_core::PWSTR, pulnumberofpaths: *mut u32) -> ::windows_core::Result<()>;
    fn FormatEx2(this: &Self::This, pwszfilesystemtypename: &::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &::windows_core::PCWSTR, options: u32) -> ::windows_core::Result<IVdsAsync>;
    fn OfflineVolume(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsVolumeMF3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolumeMF3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryVolumeGuidPathnames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpatharray: *mut *mut ::windows_core::PWSTR, pulnumberofpaths: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryVolumeGuidPathnames(this, ::core::mem::transmute_copy(&pwszpatharray), ::core::mem::transmute_copy(&pulnumberofpaths)).into())
        }
        unsafe extern "system" fn FormatEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilesystemtypename: ::windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows_core::PCWSTR, options: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatEx2(this, ::core::mem::transmute(&pwszfilesystemtypename), ::core::mem::transmute_copy(&usfilesystemrevision), ::core::mem::transmute_copy(&uldesiredunitallocationsize), ::core::mem::transmute(&pwszlabel), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OfflineVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeMF3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OfflineVolume(this).into())
        }
        IVdsVolumeMF3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryVolumeGuidPathnames: QueryVolumeGuidPathnames::<Identity, Impl, OFFSET>,
            FormatEx2: FormatEx2::<Identity, Impl, OFFSET>,
            OfflineVolume: OfflineVolume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsVolumeOnline_Impl: ::windows_core::BaseImpl {
    fn Online(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVdsVolumeOnline {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeOnline_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolumeOnline {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Online<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeOnline_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Online(this).into())
        }
        IVdsVolumeOnline_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Online: Online::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsVolumePlex_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> ::windows_core::Result<()>;
    fn GetVolume(this: &Self::This) -> ::windows_core::Result<IVdsVolume>;
    fn QueryExtents(this: &Self::This, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn Repair(this: &Self::This, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32) -> ::windows_core::Result<IVdsAsync>;
}
impl ::windows_core::Iids for IVdsVolumePlex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumePlex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolumePlex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumePlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&pplexproperties)).into())
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumePlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvolume: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumePlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryExtents(this, ::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into())
        }
        unsafe extern "system" fn Repair<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumePlex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Repair(this, ::core::mem::transmute_copy(&pinputdiskarray), ::core::mem::transmute_copy(&lnumberofdisks)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsVolumePlex_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            Repair: Repair::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVdsVolumeShrink_Impl: ::windows_core::BaseImpl {
    fn QueryMaxReclaimableBytes(this: &Self::This) -> ::windows_core::Result<u64>;
    fn Shrink(this: &Self::This, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64) -> ::windows_core::Result<IVdsAsync>;
}
impl ::windows_core::Iids for IVdsVolumeShrink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeShrink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVdsVolumeShrink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryMaxReclaimableBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeShrink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullmaxnumberofreclaimablebytes: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaxReclaimableBytes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxnumberofreclaimablebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shrink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVdsVolumeShrink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shrink(this, ::core::mem::transmute_copy(&ulldesirednumberofreclaimablebytes), ::core::mem::transmute_copy(&ullminnumberofreclaimablebytes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVdsVolumeShrink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryMaxReclaimableBytes: QueryMaxReclaimableBytes::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
