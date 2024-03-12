#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INSNetSourceCreator_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateNetSource(this: &Self::This, pszstreamname: &::windows_core::PCWSTR, pmonitor: ::core::option::Option<&::windows_core::IUnknown>, pdata: *const u8, pusercontext: ::core::option::Option<&::windows_core::IUnknown>, pcallback: ::core::option::Option<&::windows_core::IUnknown>, qwcontext: u64) -> ::windows_core::Result<()>;
    fn GetNetSourceProperties(this: &Self::This, pszstreamname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetNetSourceSharedNamespace(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetNetSourceAdminInterface(this: &Self::This, pszstreamname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetNumProtocolsSupported(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetProtocolName(this: &Self::This, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u16) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INSNetSourceCreator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INSNetSourceCreator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        unsafe extern "system" fn CreateNetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstreamname: ::windows_core::PCWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNetSource(this, ::core::mem::transmute(&pszstreamname), ::windows_core::from_raw_borrowed(&pmonitor), ::core::mem::transmute_copy(&pdata), ::windows_core::from_raw_borrowed(&pusercontext), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&qwcontext)).into())
        }
        unsafe extern "system" fn GetNetSourceProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstreamname: ::windows_core::PCWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetSourceProperties(this, ::core::mem::transmute(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertiesnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetSourceSharedNamespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsharednamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstreamname: ::windows_core::PCWSTR, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetSourceAdminInterface(this, ::core::mem::transmute(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumProtocolsSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprotocols, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProtocolName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtocolName(this, ::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        INSNetSourceCreator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateNetSource: CreateNetSource::<Identity, Impl, OFFSET>,
            GetNetSourceProperties: GetNetSourceProperties::<Identity, Impl, OFFSET>,
            GetNetSourceSharedNamespace: GetNetSourceSharedNamespace::<Identity, Impl, OFFSET>,
            GetNetSourceAdminInterface: GetNetSourceAdminInterface::<Identity, Impl, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, Impl, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INSSBuffer_Impl: ::windows_core::BaseImpl {
    fn GetLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetLength(this: &Self::This, dwlength: u32) -> ::windows_core::Result<()>;
    fn GetMaxLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetBuffer(this: &Self::This) -> ::windows_core::Result<*mut u8>;
    fn GetBufferAndLength(this: &Self::This, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INSSBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INSSBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLength(this, ::core::mem::transmute_copy(&dwlength)).into())
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdwbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferAndLength(this, ::core::mem::transmute_copy(&ppdwbuffer), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        INSSBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLength: GetLength::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INSSBuffer2_Impl: ::windows_core::BaseImpl + INSSBuffer_Impl {
    fn GetSampleProperties(this: &Self::This, cbproperties: u32) -> ::windows_core::Result<u8>;
    fn SetSampleProperties(this: &Self::This, cbproperties: u32, pbproperties: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INSSBuffer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INSSBuffer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INSSBuffer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSampleProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSampleProperties(this, ::core::mem::transmute_copy(&cbproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSampleProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSampleProperties(this, ::core::mem::transmute_copy(&cbproperties), ::core::mem::transmute_copy(&pbproperties)).into())
        }
        INSSBuffer2_Vtbl {
            base__: <INSSBuffer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSampleProperties: GetSampleProperties::<Identity, Impl, OFFSET>,
            SetSampleProperties: SetSampleProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INSSBuffer3_Impl: ::windows_core::BaseImpl + INSSBuffer2_Impl {
    fn SetProperty(this: &Self::This, guidbufferproperty: &::windows_core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, guidbufferproperty: &::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INSSBuffer3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INSSBuffer2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INSSBuffer3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows_core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&dwbufferpropertysize)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into())
        }
        INSSBuffer3_Vtbl {
            base__: <INSSBuffer2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INSSBuffer4_Impl: ::windows_core::BaseImpl + INSSBuffer3_Impl {
    fn GetPropertyCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPropertyByIndex(this: &Self::This, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INSSBuffer4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INSSBuffer3);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INSSBuffer4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbufferproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INSSBuffer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyByIndex(this, ::core::mem::transmute_copy(&dwbufferpropertyindex), ::core::mem::transmute_copy(&pguidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into())
        }
        INSSBuffer4_Vtbl {
            base__: <INSSBuffer3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMAddressAccess_Impl: ::windows_core::BaseImpl {
    fn GetAccessEntryCount(this: &Self::This, aetype: WM_AETYPE) -> ::windows_core::Result<u32>;
    fn GetAccessEntry(this: &Self::This, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::Result<WM_ADDRESS_ACCESSENTRY>;
    fn AddAccessEntry(this: &Self::This, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows_core::Result<()>;
    fn RemoveAccessEntry(this: &Self::This, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMAddressAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMAddressAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAccessEntryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccessEntryCount(this, ::core::mem::transmute_copy(&aetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcentries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAccessEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccessEntry(this, ::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddraccessentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddAccessEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAccessEntry(this, ::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&paddraccessentry)).into())
        }
        unsafe extern "system" fn RemoveAccessEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAccessEntry(this, ::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)).into())
        }
        IWMAddressAccess_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAccessEntryCount: GetAccessEntryCount::<Identity, Impl, OFFSET>,
            GetAccessEntry: GetAccessEntry::<Identity, Impl, OFFSET>,
            AddAccessEntry: AddAccessEntry::<Identity, Impl, OFFSET>,
            RemoveAccessEntry: RemoveAccessEntry::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMAddressAccess2_Impl: ::windows_core::BaseImpl + IWMAddressAccess_Impl {
    fn GetAccessEntryEx(this: &Self::This, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::windows_core::BSTR, pbstrmask: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddAccessEntryEx(this: &Self::This, aetype: WM_AETYPE, bstraddress: &::windows_core::BSTR, bstrmask: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMAddressAccess2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMAddressAccess);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMAddressAccess2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAccessEntryEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrmask: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAccessEntryEx(this, ::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum), ::core::mem::transmute_copy(&pbstraddress), ::core::mem::transmute_copy(&pbstrmask)).into())
        }
        unsafe extern "system" fn AddAccessEntryEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAddressAccess2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrmask: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAccessEntryEx(this, ::core::mem::transmute_copy(&aetype), ::core::mem::transmute(&bstraddress), ::core::mem::transmute(&bstrmask)).into())
        }
        IWMAddressAccess2_Vtbl {
            base__: <IWMAddressAccess as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAccessEntryEx: GetAccessEntryEx::<Identity, Impl, OFFSET>,
            AddAccessEntryEx: AddAccessEntryEx::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMAuthorizer_Impl: ::windows_core::BaseImpl {
    fn GetCertCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCert(this: &Self::This, dwindex: u32) -> ::windows_core::Result<*mut u8>;
    fn GetSharedData(this: &Self::This, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows_core::Result<*mut u8>;
}
impl ::windows_core::Iids for IWMAuthorizer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMAuthorizer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCertCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccerts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCert(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbcertdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSharedData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSharedData(this, ::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata), ::core::mem::transmute_copy(&pbcert)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbshareddata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMAuthorizer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCertCount: GetCertCount::<Identity, Impl, OFFSET>,
            GetCert: GetCert::<Identity, Impl, OFFSET>,
            GetSharedData: GetSharedData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMBackupRestoreProps_Impl: ::windows_core::BaseImpl {
    fn GetPropCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetPropByIndex(this: &Self::This, windex: u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn GetPropByName(this: &Self::This, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn SetProp(this: &Self::This, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
    fn RemoveProp(this: &Self::This, pcwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoveAllProps(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMBackupRestoreProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMBackupRestoreProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropByIndex(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn GetPropByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropByName(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn SetProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProp(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        unsafe extern "system" fn RemoveProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveProp(this, ::core::mem::transmute(&pcwszname)).into())
        }
        unsafe extern "system" fn RemoveAllProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllProps(this).into())
        }
        IWMBackupRestoreProps_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropCount: GetPropCount::<Identity, Impl, OFFSET>,
            GetPropByIndex: GetPropByIndex::<Identity, Impl, OFFSET>,
            GetPropByName: GetPropByName::<Identity, Impl, OFFSET>,
            SetProp: SetProp::<Identity, Impl, OFFSET>,
            RemoveProp: RemoveProp::<Identity, Impl, OFFSET>,
            RemoveAllProps: RemoveAllProps::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMBandwidthSharing_Impl: ::windows_core::BaseImpl + IWMStreamList_Impl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetType(this: &Self::This, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetBandwidth(this: &Self::This, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows_core::Result<()>;
    fn SetBandwidth(this: &Self::This, dwbitrate: u32, msbufferwindow: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMBandwidthSharing {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMStreamList);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMBandwidthSharing {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&guidtype)).into())
        }
        unsafe extern "system" fn GetBandwidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBandwidth(this, ::core::mem::transmute_copy(&pdwbitrate), ::core::mem::transmute_copy(&pmsbufferwindow)).into())
        }
        unsafe extern "system" fn SetBandwidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBandwidth(this, ::core::mem::transmute_copy(&dwbitrate), ::core::mem::transmute_copy(&msbufferwindow)).into())
        }
        IWMBandwidthSharing_Vtbl {
            base__: <IWMStreamList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            GetBandwidth: GetBandwidth::<Identity, Impl, OFFSET>,
            SetBandwidth: SetBandwidth::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMClientConnections_Impl: ::windows_core::BaseImpl {
    fn GetClientCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetClientProperties(this: &Self::This, dwclientnum: u32) -> ::windows_core::Result<WM_CLIENT_PROPERTIES>;
}
impl ::windows_core::Iids for IWMClientConnections {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMClientConnections_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMClientConnections {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClientCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMClientConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcclients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClientProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMClientConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientProperties(this, ::core::mem::transmute_copy(&dwclientnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclientproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMClientConnections_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClientCount: GetClientCount::<Identity, Impl, OFFSET>,
            GetClientProperties: GetClientProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMClientConnections2_Impl: ::windows_core::BaseImpl + IWMClientConnections_Impl {
    fn GetClientInfo(this: &Self::This, dwclientnum: u32, pwsznetworkaddress: ::windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows_core::PWSTR, pcchdnsname: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMClientConnections2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMClientConnections);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMClientConnections2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMClientConnections2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClientInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMClientConnections2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: ::windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows_core::PWSTR, pcchdnsname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClientInfo(this, ::core::mem::transmute_copy(&dwclientnum), ::core::mem::transmute_copy(&pwsznetworkaddress), ::core::mem::transmute_copy(&pcchnetworkaddress), ::core::mem::transmute_copy(&pwszport), ::core::mem::transmute_copy(&pcchport), ::core::mem::transmute_copy(&pwszdnsname), ::core::mem::transmute_copy(&pcchdnsname)).into())
        }
        IWMClientConnections2_Vtbl { base__: <IWMClientConnections as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetClientInfo: GetClientInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMCodecInfo_Impl: ::windows_core::BaseImpl {
    fn GetCodecInfoCount(this: &Self::This, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn GetCodecFormatCount(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32) -> ::windows_core::Result<u32>;
    fn GetCodecFormat(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows_core::Result<IWMStreamConfig>;
}
impl ::windows_core::Iids for IWMCodecInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMCodecInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCodecInfoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, pccodecs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodecInfoCount(this, ::core::mem::transmute_copy(&guidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccodecs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodecFormatCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodecFormatCount(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodecFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodecFormat(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppistreamconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMCodecInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCodecInfoCount: GetCodecInfoCount::<Identity, Impl, OFFSET>,
            GetCodecFormatCount: GetCodecFormatCount::<Identity, Impl, OFFSET>,
            GetCodecFormat: GetCodecFormat::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMCodecInfo2_Impl: ::windows_core::BaseImpl + IWMCodecInfo_Impl {
    fn GetCodecName(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, wszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()>;
    fn GetCodecFormatDesc(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows_core::PWSTR, pcchdesc: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMCodecInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMCodecInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMCodecInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCodecName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, wszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecName(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn GetCodecFormatDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void, wszdesc: ::windows_core::PWSTR, pcchdesc: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecFormatDesc(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute_copy(&ppistreamconfig), ::core::mem::transmute_copy(&wszdesc), ::core::mem::transmute_copy(&pcchdesc)).into())
        }
        IWMCodecInfo2_Vtbl {
            base__: <IWMCodecInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCodecName: GetCodecName::<Identity, Impl, OFFSET>,
            GetCodecFormatDesc: GetCodecFormatDesc::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMCodecInfo3_Impl: ::windows_core::BaseImpl + IWMCodecInfo2_Impl {
    fn GetCodecFormatProp(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetCodecProp(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
    fn SetCodecEnumerationSetting(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::Result<()>;
    fn GetCodecEnumerationSetting(this: &Self::This, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMCodecInfo3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMCodecInfo2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMCodecInfo3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCodecFormatProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecFormatProp(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        unsafe extern "system" fn GetCodecProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecProp(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCodecEnumerationSetting(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into())
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecEnumerationSetting(this, ::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        IWMCodecInfo3_Vtbl {
            base__: <IWMCodecInfo2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCodecFormatProp: GetCodecFormatProp::<Identity, Impl, OFFSET>,
            GetCodecProp: GetCodecProp::<Identity, Impl, OFFSET>,
            SetCodecEnumerationSetting: SetCodecEnumerationSetting::<Identity, Impl, OFFSET>,
            GetCodecEnumerationSetting: GetCodecEnumerationSetting::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMCredentialCallback_Impl: ::windows_core::BaseImpl {
    fn AcquireCredentials(this: &Self::This, pwszrealm: &::windows_core::PCWSTR, pwszsite: &::windows_core::PCWSTR, pwszuser: ::windows_core::PWSTR, cchuser: u32, pwszpassword: ::windows_core::PWSTR, cchpassword: u32, hrstatus: ::windows_core::HRESULT, pdwflags: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMCredentialCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCredentialCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMCredentialCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMCredentialCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrealm: ::windows_core::PCWSTR, pwszsite: ::windows_core::PCWSTR, pwszuser: ::windows_core::PWSTR, cchuser: u32, pwszpassword: ::windows_core::PWSTR, cchpassword: u32, hrstatus: ::windows_core::HRESULT, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireCredentials(this, ::core::mem::transmute(&pwszrealm), ::core::mem::transmute(&pwszsite), ::core::mem::transmute_copy(&pwszuser), ::core::mem::transmute_copy(&cchuser), ::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&cchpassword), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&pdwflags)).into())
        }
        IWMCredentialCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireCredentials: AcquireCredentials::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMDRMEditor_Impl: ::windows_core::BaseImpl {
    fn GetDRMProperty(this: &Self::This, pwstrname: &::windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDRMEditor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMEditor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMEditor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDRMProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrname: ::windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDRMProperty(this, ::core::mem::transmute(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        IWMDRMEditor_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDRMProperty: GetDRMProperty::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMDRMMessageParser_Impl: ::windows_core::BaseImpl {
    fn ParseRegistrationReqMsg(this: &Self::This, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows_core::Result<()>;
    fn ParseLicenseRequestMsg(this: &Self::This, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDRMMessageParser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMMessageParser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMMessageParser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseRegistrationReqMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMMessageParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseRegistrationReqMsg(this, ::core::mem::transmute_copy(&pbregistrationreqmsg), ::core::mem::transmute_copy(&cbregistrationreqmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber)).into())
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMMessageParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseLicenseRequestMsg(this, ::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber), ::core::mem::transmute_copy(&pbstraction)).into())
        }
        IWMDRMMessageParser_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseRegistrationReqMsg: ParseRegistrationReqMsg::<Identity, Impl, OFFSET>,
            ParseLicenseRequestMsg: ParseLicenseRequestMsg::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMDRMReader_Impl: ::windows_core::BaseImpl {
    fn AcquireLicense(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn CancelLicenseAcquisition(this: &Self::This) -> ::windows_core::Result<()>;
    fn Individualize(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn CancelIndividualization(this: &Self::This) -> ::windows_core::Result<()>;
    fn MonitorLicenseAcquisition(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelMonitorLicenseAcquisition(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetDRMProperty(this: &Self::This, pwstrname: &::windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
    fn GetDRMProperty(this: &Self::This, pwstrname: &::windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDRMReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireLicense(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelLicenseAcquisition(this).into())
        }
        unsafe extern "system" fn Individualize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Individualize(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn CancelIndividualization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelIndividualization(this).into())
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MonitorLicenseAcquisition(this).into())
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelMonitorLicenseAcquisition(this).into())
        }
        unsafe extern "system" fn SetDRMProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrname: ::windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDRMProperty(this, ::core::mem::transmute(&pwstrname), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        unsafe extern "system" fn GetDRMProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrname: ::windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDRMProperty(this, ::core::mem::transmute(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        IWMDRMReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireLicense: AcquireLicense::<Identity, Impl, OFFSET>,
            CancelLicenseAcquisition: CancelLicenseAcquisition::<Identity, Impl, OFFSET>,
            Individualize: Individualize::<Identity, Impl, OFFSET>,
            CancelIndividualization: CancelIndividualization::<Identity, Impl, OFFSET>,
            MonitorLicenseAcquisition: MonitorLicenseAcquisition::<Identity, Impl, OFFSET>,
            CancelMonitorLicenseAcquisition: CancelMonitorLicenseAcquisition::<Identity, Impl, OFFSET>,
            SetDRMProperty: SetDRMProperty::<Identity, Impl, OFFSET>,
            GetDRMProperty: GetDRMProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader2_Impl: ::windows_core::BaseImpl + IWMDRMReader_Impl {
    fn SetEvaluateOutputLevelLicenses(this: &Self::This, fevaluate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPlayOutputLevels(this: &Self::This, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::Result<()>;
    fn GetCopyOutputLevels(this: &Self::This, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::Result<()>;
    fn TryNextLicense(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMDRMReader2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDRMReader);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMReader2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEvaluateOutputLevelLicenses(this, ::core::mem::transmute_copy(&fevaluate)).into())
        }
        unsafe extern "system" fn GetPlayOutputLevels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPlayOutputLevels(this, ::core::mem::transmute_copy(&pplayopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into())
        }
        unsafe extern "system" fn GetCopyOutputLevels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCopyOutputLevels(this, ::core::mem::transmute_copy(&pcopyopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into())
        }
        unsafe extern "system" fn TryNextLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TryNextLicense(this).into())
        }
        IWMDRMReader2_Vtbl {
            base__: <IWMDRMReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetEvaluateOutputLevelLicenses: SetEvaluateOutputLevelLicenses::<Identity, Impl, OFFSET>,
            GetPlayOutputLevels: GetPlayOutputLevels::<Identity, Impl, OFFSET>,
            GetCopyOutputLevels: GetCopyOutputLevels::<Identity, Impl, OFFSET>,
            TryNextLicense: TryNextLicense::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader3_Impl: ::windows_core::BaseImpl + IWMDRMReader2_Impl {
    fn GetInclusionList(this: &Self::This, ppguids: *mut *mut ::windows_core::GUID, pcguids: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMDRMReader3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDRMReader2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMReader3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInclusionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMReader3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows_core::GUID, pcguids: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInclusionList(this, ::core::mem::transmute_copy(&ppguids), ::core::mem::transmute_copy(&pcguids)).into())
        }
        IWMDRMReader3_Vtbl { base__: <IWMDRMReader2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetInclusionList: GetInclusionList::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMDRMTranscryptionManager_Impl: ::windows_core::BaseImpl {
    fn CreateTranscryptor(this: &Self::This) -> ::windows_core::Result<IWMDRMTranscryptor>;
}
impl ::windows_core::Iids for IWMDRMTranscryptionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMTranscryptionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTranscryptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptranscryptor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTranscryptor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptranscryptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDRMTranscryptionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTranscryptor: CreateTranscryptor::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMDRMTranscryptor_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, bstrfilename: &::windows_core::BSTR, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, hnstime: u64) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, pbdata: *const u8, pcbdata: *const u32) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDRMTranscryptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMTranscryptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&pplicenseresponsemsg), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&hnstime)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&pcbdata)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IWMDRMTranscryptor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptor2_Impl: ::windows_core::BaseImpl + IWMDRMTranscryptor_Impl {
    fn SeekEx(this: &Self::This, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ZeroAdjustTimestamps(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSeekStartTime(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetDuration(this: &Self::This) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMDRMTranscryptor2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDRMTranscryptor);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMTranscryptor2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SeekEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SeekEx(this, ::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&flrate), ::core::mem::transmute_copy(&fincludefileheader)).into())
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ZeroAdjustTimestamps(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn GetSeekStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSeekStartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnstime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDRMTranscryptor2_Vtbl {
            base__: <IWMDRMTranscryptor as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SeekEx: SeekEx::<Identity, Impl, OFFSET>,
            ZeroAdjustTimestamps: ZeroAdjustTimestamps::<Identity, Impl, OFFSET>,
            GetSeekStartTime: GetSeekStartTime::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMDRMWriter_Impl: ::windows_core::BaseImpl {
    fn GenerateKeySeed(this: &Self::This, pwszkeyseed: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()>;
    fn GenerateKeyID(this: &Self::This, pwszkeyid: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()>;
    fn GenerateSigningKeyPair(this: &Self::This, pwszprivkey: ::windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_core::Result<()>;
    fn SetDRMAttribute(this: &Self::This, wstreamnum: u16, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDRMWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GenerateKeySeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszkeyseed: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateKeySeed(this, ::core::mem::transmute_copy(&pwszkeyseed), ::core::mem::transmute_copy(&pcwchlength)).into())
        }
        unsafe extern "system" fn GenerateKeyID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszkeyid: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateKeyID(this, ::core::mem::transmute_copy(&pwszkeyid), ::core::mem::transmute_copy(&pcwchlength)).into())
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprivkey: ::windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateSigningKeyPair(this, ::core::mem::transmute_copy(&pwszprivkey), ::core::mem::transmute_copy(&pcwchprivkeylength), ::core::mem::transmute_copy(&pwszpubkey), ::core::mem::transmute_copy(&pcwchpubkeylength)).into())
        }
        unsafe extern "system" fn SetDRMAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDRMAttribute(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        IWMDRMWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GenerateKeySeed: GenerateKeySeed::<Identity, Impl, OFFSET>,
            GenerateKeyID: GenerateKeyID::<Identity, Impl, OFFSET>,
            GenerateSigningKeyPair: GenerateSigningKeyPair::<Identity, Impl, OFFSET>,
            SetDRMAttribute: SetDRMAttribute::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter2_Impl: ::windows_core::BaseImpl + IWMDRMWriter_Impl {
    fn SetWMDRMNetEncryption(this: &Self::This, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMDRMWriter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDRMWriter);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMWriter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetWMDRMNetEncryption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWMDRMNetEncryption(this, ::core::mem::transmute_copy(&fsamplesencrypted), ::core::mem::transmute_copy(&pbkeyid), ::core::mem::transmute_copy(&cbkeyid)).into())
        }
        IWMDRMWriter2_Vtbl {
            base__: <IWMDRMWriter as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter3_Impl: ::windows_core::BaseImpl + IWMDRMWriter2_Impl {
    fn SetProtectStreamSamples(this: &Self::This, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMDRMWriter3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDRMWriter2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDRMWriter3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDRMWriter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtectStreamSamples(this, ::core::mem::transmute_copy(&pimportinitstruct)).into())
        }
        IWMDRMWriter3_Vtbl {
            base__: <IWMDRMWriter2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProtectStreamSamples: SetProtectStreamSamples::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMDeviceRegistration_Impl: ::windows_core::BaseImpl {
    fn RegisterDevice(this: &Self::This, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows_core::Result<IWMRegisteredDevice>;
    fn UnregisterDevice(this: &Self::This, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows_core::Result<()>;
    fn GetRegistrationStats(this: &Self::This, dwregistertype: u32) -> ::windows_core::Result<u32>;
    fn GetFirstRegisteredDevice(this: &Self::This, dwregistertype: u32) -> ::windows_core::Result<IWMRegisteredDevice>;
    fn GetNextRegisteredDevice(this: &Self::This) -> ::windows_core::Result<IWMRegisteredDevice>;
    fn GetRegisteredDeviceByID(this: &Self::This, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows_core::Result<IWMRegisteredDevice>;
}
impl ::windows_core::Iids for IWMDeviceRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDeviceRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterDevice(this, ::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDevice(this, ::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute(&serialnumber)).into())
        }
        unsafe extern "system" fn GetRegistrationStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegistrationStats(this, ::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcregistereddevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstRegisteredDevice(this, ::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextRegisteredDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegisteredDeviceByID(this, ::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDeviceRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterDevice: RegisterDevice::<Identity, Impl, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, Impl, OFFSET>,
            GetRegistrationStats: GetRegistrationStats::<Identity, Impl, OFFSET>,
            GetFirstRegisteredDevice: GetFirstRegisteredDevice::<Identity, Impl, OFFSET>,
            GetNextRegisteredDevice: GetNextRegisteredDevice::<Identity, Impl, OFFSET>,
            GetRegisteredDeviceByID: GetRegisteredDeviceByID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMGetSecureChannel_Impl: ::windows_core::BaseImpl {
    fn GetPeerSecureChannelInterface(this: &Self::This) -> ::windows_core::Result<IWMSecureChannel>;
}
impl ::windows_core::Iids for IWMGetSecureChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMGetSecureChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMGetSecureChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMGetSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppeer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPeerSecureChannelInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppeer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMGetSecureChannel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPeerSecureChannelInterface: GetPeerSecureChannelInterface::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMHeaderInfo_Impl: ::windows_core::BaseImpl {
    fn GetAttributeCount(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<u16>;
    fn GetAttributeByIndex(this: &Self::This, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn GetAttributeByName(this: &Self::This, pwstreamnum: *mut u16, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn SetAttribute(this: &Self::This, wstreamnum: u16, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
    fn GetMarkerCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetMarker(this: &Self::This, windex: u16, pwszmarkername: ::windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_core::Result<()>;
    fn AddMarker(this: &Self::This, pwszmarkername: &::windows_core::PCWSTR, cnsmarkertime: u64) -> ::windows_core::Result<()>;
    fn RemoveMarker(this: &Self::This, windex: u16) -> ::windows_core::Result<()>;
    fn GetScriptCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetScript(this: &Self::This, windex: u16, pwsztype: ::windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_core::Result<()>;
    fn AddScript(this: &Self::This, pwsztype: &::windows_core::PCWSTR, pwszcommand: &::windows_core::PCWSTR, cnsscripttime: u64) -> ::windows_core::Result<()>;
    fn RemoveScript(this: &Self::This, windex: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMHeaderInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMHeaderInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeCount(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeByIndex(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn GetAttributeByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeByName(this, ::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn SetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttribute(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        unsafe extern "system" fn GetMarkerCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMarkerCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcmarkers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: ::windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMarker(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszmarkername), ::core::mem::transmute_copy(&pcchmarkernamelen), ::core::mem::transmute_copy(&pcnsmarkertime)).into())
        }
        unsafe extern "system" fn AddMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmarkername: ::windows_core::PCWSTR, cnsmarkertime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMarker(this, ::core::mem::transmute(&pwszmarkername), ::core::mem::transmute_copy(&cnsmarkertime)).into())
        }
        unsafe extern "system" fn RemoveMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMarker(this, ::core::mem::transmute_copy(&windex)).into())
        }
        unsafe extern "system" fn GetScriptCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScriptCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcscripts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: ::windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScript(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwsztype), ::core::mem::transmute_copy(&pcchtypelen), ::core::mem::transmute_copy(&pwszcommand), ::core::mem::transmute_copy(&pcchcommandlen), ::core::mem::transmute_copy(&pcnsscripttime)).into())
        }
        unsafe extern "system" fn AddScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztype: ::windows_core::PCWSTR, pwszcommand: ::windows_core::PCWSTR, cnsscripttime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddScript(this, ::core::mem::transmute(&pwsztype), ::core::mem::transmute(&pwszcommand), ::core::mem::transmute_copy(&cnsscripttime)).into())
        }
        unsafe extern "system" fn RemoveScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveScript(this, ::core::mem::transmute_copy(&windex)).into())
        }
        IWMHeaderInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttributeCount: GetAttributeCount::<Identity, Impl, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, Impl, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, Impl, OFFSET>,
            SetAttribute: SetAttribute::<Identity, Impl, OFFSET>,
            GetMarkerCount: GetMarkerCount::<Identity, Impl, OFFSET>,
            GetMarker: GetMarker::<Identity, Impl, OFFSET>,
            AddMarker: AddMarker::<Identity, Impl, OFFSET>,
            RemoveMarker: RemoveMarker::<Identity, Impl, OFFSET>,
            GetScriptCount: GetScriptCount::<Identity, Impl, OFFSET>,
            GetScript: GetScript::<Identity, Impl, OFFSET>,
            AddScript: AddScript::<Identity, Impl, OFFSET>,
            RemoveScript: RemoveScript::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMHeaderInfo2_Impl: ::windows_core::BaseImpl + IWMHeaderInfo_Impl {
    fn GetCodecInfoCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCodecInfo(this: &Self::This, windex: u32, pcchname: *mut u16, pwszname: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMHeaderInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMHeaderInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMHeaderInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCodecInfoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodecInfoCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccodecinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodecInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecInfo(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcodectype), ::core::mem::transmute_copy(&pcbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into())
        }
        IWMHeaderInfo2_Vtbl {
            base__: <IWMHeaderInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCodecInfoCount: GetCodecInfoCount::<Identity, Impl, OFFSET>,
            GetCodecInfo: GetCodecInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMHeaderInfo3_Impl: ::windows_core::BaseImpl + IWMHeaderInfo2_Impl {
    fn GetAttributeCountEx(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<u16>;
    fn GetAttributeIndices(this: &Self::This, wstreamnum: u16, pwszname: &::windows_core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows_core::Result<()>;
    fn GetAttributeByIndexEx(this: &Self::This, wstreamnum: u16, windex: u16, pwszname: ::windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows_core::Result<()>;
    fn ModifyAttribute(this: &Self::This, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_core::Result<()>;
    fn AddAttribute(this: &Self::This, wstreamnum: u16, pszname: &::windows_core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_core::Result<()>;
    fn DeleteAttribute(this: &Self::This, wstreamnum: u16, windex: u16) -> ::windows_core::Result<()>;
    fn AddCodecInfo(this: &Self::This, pwszname: &::windows_core::PCWSTR, pwszdescription: &::windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMHeaderInfo3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMHeaderInfo2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMHeaderInfo3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttributeCountEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeCountEx(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: ::windows_core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeIndices(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pwindices), ::core::mem::transmute_copy(&pwcount)).into())
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: ::windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeByIndexEx(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwdatalength)).into())
        }
        unsafe extern "system" fn ModifyAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyAttribute(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into())
        }
        unsafe extern "system" fn AddAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows_core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAttribute(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pwindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into())
        }
        unsafe extern "system" fn DeleteAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttribute(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex)).into())
        }
        unsafe extern "system" fn AddCodecInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwszdescription: ::windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddCodecInfo(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwszdescription), ::core::mem::transmute_copy(&codectype), ::core::mem::transmute_copy(&cbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into())
        }
        IWMHeaderInfo3_Vtbl {
            base__: <IWMHeaderInfo2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttributeCountEx: GetAttributeCountEx::<Identity, Impl, OFFSET>,
            GetAttributeIndices: GetAttributeIndices::<Identity, Impl, OFFSET>,
            GetAttributeByIndexEx: GetAttributeByIndexEx::<Identity, Impl, OFFSET>,
            ModifyAttribute: ModifyAttribute::<Identity, Impl, OFFSET>,
            AddAttribute: AddAttribute::<Identity, Impl, OFFSET>,
            DeleteAttribute: DeleteAttribute::<Identity, Impl, OFFSET>,
            AddCodecInfo: AddCodecInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMIStreamProps_Impl: ::windows_core::BaseImpl {
    fn GetProperty(this: &Self::This, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMIStreamProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIStreamProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMIStreamProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIStreamProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        IWMIStreamProps_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetProperty: GetProperty::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMImageInfo_Impl: ::windows_core::BaseImpl {
    fn GetImageCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetImage(this: &Self::This, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMImageInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMImageInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMImageInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMImageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcimages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMImageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImage(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchmimetype), ::core::mem::transmute_copy(&pwszmimetype), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pimagetype), ::core::mem::transmute_copy(&pcbimagedata), ::core::mem::transmute_copy(&pbimagedata)).into())
        }
        IWMImageInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImageCount: GetImageCount::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMIndexer_Impl: ::windows_core::BaseImpl {
    fn StartIndexing(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMIndexer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIndexer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMIndexer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartIndexing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIndexer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartIndexing(this, ::core::mem::transmute(&pwszurl), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIndexer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IWMIndexer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartIndexing: StartIndexing::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMIndexer2_Impl: ::windows_core::BaseImpl + IWMIndexer_Impl {
    fn Configure(this: &Self::This, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMIndexer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMIndexer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIndexer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMIndexer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Configure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIndexer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Configure(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&nindexertype), ::core::mem::transmute_copy(&pvinterval), ::core::mem::transmute_copy(&pvindextype)).into())
        }
        IWMIndexer2_Vtbl { base__: <IWMIndexer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Configure: Configure::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMInputMediaProps_Impl: ::windows_core::BaseImpl + IWMMediaProps_Impl {
    fn GetConnectionName(this: &Self::This, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()>;
    fn GetGroupName(this: &Self::This, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMInputMediaProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMMediaProps);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMInputMediaProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMInputMediaProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMInputMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectionName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn GetGroupName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMInputMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGroupName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        IWMInputMediaProps_Vtbl {
            base__: <IWMMediaProps as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
            GetGroupName: GetGroupName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMLanguageList_Impl: ::windows_core::BaseImpl {
    fn GetLanguageCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetLanguageDetails(this: &Self::This, windex: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()>;
    fn AddLanguageByRFC1766String(this: &Self::This, pwszlanguagestring: &::windows_core::PCWSTR) -> ::windows_core::Result<u16>;
}
impl ::windows_core::Iids for IWMLanguageList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMLanguageList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLanguageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguageCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLanguageDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLanguageDetails(this, ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into())
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows_core::PCWSTR, pwindex: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddLanguageByRFC1766String(this, ::core::mem::transmute(&pwszlanguagestring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMLanguageList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLanguageCount: GetLanguageCount::<Identity, Impl, OFFSET>,
            GetLanguageDetails: GetLanguageDetails::<Identity, Impl, OFFSET>,
            AddLanguageByRFC1766String: AddLanguageByRFC1766String::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMLicenseBackup_Impl: ::windows_core::BaseImpl {
    fn BackupLicenses(this: &Self::This, dwflags: u32, pcallback: ::core::option::Option<&IWMStatusCallback>) -> ::windows_core::Result<()>;
    fn CancelLicenseBackup(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMLicenseBackup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseBackup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMLicenseBackup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BackupLicenses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackupLicenses(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn CancelLicenseBackup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelLicenseBackup(this).into())
        }
        IWMLicenseBackup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BackupLicenses: BackupLicenses::<Identity, Impl, OFFSET>,
            CancelLicenseBackup: CancelLicenseBackup::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMLicenseRestore_Impl: ::windows_core::BaseImpl {
    fn RestoreLicenses(this: &Self::This, dwflags: u32, pcallback: ::core::option::Option<&IWMStatusCallback>) -> ::windows_core::Result<()>;
    fn CancelLicenseRestore(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMLicenseRestore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseRestore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMLicenseRestore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RestoreLicenses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseRestore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreLicenses(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn CancelLicenseRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseRestore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelLicenseRestore(this).into())
        }
        IWMLicenseRestore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RestoreLicenses: RestoreLicenses::<Identity, Impl, OFFSET>,
            CancelLicenseRestore: CancelLicenseRestore::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMLicenseRevocationAgent_Impl: ::windows_core::BaseImpl {
    fn GetLRBChallenge(this: &Self::This, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows_core::Result<()>;
    fn ProcessLRB(this: &Self::This, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMLicenseRevocationAgent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMLicenseRevocationAgent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLRBChallenge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLRBChallenge(this, ::core::mem::transmute_copy(&pmachineid), ::core::mem::transmute_copy(&dwmachineidlength), ::core::mem::transmute_copy(&pchallenge), ::core::mem::transmute_copy(&dwchallengelength), ::core::mem::transmute_copy(&pchallengeoutput), ::core::mem::transmute_copy(&pdwchallengeoutputlength)).into())
        }
        unsafe extern "system" fn ProcessLRB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessLRB(this, ::core::mem::transmute_copy(&psignedlrb), ::core::mem::transmute_copy(&dwsignedlrblength), ::core::mem::transmute_copy(&psignedack), ::core::mem::transmute_copy(&pdwsignedacklength)).into())
        }
        IWMLicenseRevocationAgent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLRBChallenge: GetLRBChallenge::<Identity, Impl, OFFSET>,
            ProcessLRB: ProcessLRB::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMediaProps_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetMediaType(this: &Self::This, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::Result<()>;
    fn SetMediaType(this: &Self::This, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMMediaProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMMediaProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMediaType(this, ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pcbtype)).into())
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMediaType(this, ::core::mem::transmute_copy(&ptype)).into())
        }
        IWMMediaProps_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetMediaType: GetMediaType::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMMetadataEditor_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pwszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMMetadataEditor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMMetadataEditor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pwszfilename)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        IWMMetadataEditor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMMetadataEditor2_Impl: ::windows_core::BaseImpl + IWMMetadataEditor_Impl {
    fn OpenEx(this: &Self::This, pwszfilename: &::windows_core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMMetadataEditor2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMMetadataEditor);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMetadataEditor2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMMetadataEditor2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMetadataEditor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenEx(this, ::core::mem::transmute(&pwszfilename), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&dwsharemode)).into())
        }
        IWMMetadataEditor2_Vtbl { base__: <IWMMetadataEditor as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OpenEx: OpenEx::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMMutualExclusion_Impl: ::windows_core::BaseImpl + IWMStreamList_Impl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetType(this: &Self::This, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMMutualExclusion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMStreamList);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMMutualExclusion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&guidtype)).into())
        }
        IWMMutualExclusion_Vtbl {
            base__: <IWMStreamList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMMutualExclusion2_Impl: ::windows_core::BaseImpl + IWMMutualExclusion_Impl {
    fn GetName(this: &Self::This, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRecordCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn AddRecord(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveRecord(this: &Self::This, wrecordnumber: u16) -> ::windows_core::Result<()>;
    fn GetRecordName(this: &Self::This, wrecordnumber: u16, pwszrecordname: ::windows_core::PWSTR, pcchrecordname: *mut u16) -> ::windows_core::Result<()>;
    fn SetRecordName(this: &Self::This, wrecordnumber: u16, pwszrecordname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStreamsForRecord(this: &Self::This, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()>;
    fn AddStreamForRecord(this: &Self::This, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::Result<()>;
    fn RemoveStreamForRecord(this: &Self::This, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMMutualExclusion2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMMutualExclusion);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMMutualExclusion2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecordCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwrecordcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRecord(this).into())
        }
        unsafe extern "system" fn RemoveRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveRecord(this, ::core::mem::transmute_copy(&wrecordnumber)).into())
        }
        unsafe extern "system" fn GetRecordName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows_core::PWSTR, pcchrecordname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRecordName(this, ::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwszrecordname), ::core::mem::transmute_copy(&pcchrecordname)).into())
        }
        unsafe extern "system" fn SetRecordName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecordName(this, ::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute(&pwszrecordname)).into())
        }
        unsafe extern "system" fn GetStreamsForRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamsForRecord(this, ::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into())
        }
        unsafe extern "system" fn AddStreamForRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStreamForRecord(this, ::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into())
        }
        unsafe extern "system" fn RemoveStreamForRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStreamForRecord(this, ::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into())
        }
        IWMMutualExclusion2_Vtbl {
            base__: <IWMMutualExclusion as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
            AddRecord: AddRecord::<Identity, Impl, OFFSET>,
            RemoveRecord: RemoveRecord::<Identity, Impl, OFFSET>,
            GetRecordName: GetRecordName::<Identity, Impl, OFFSET>,
            SetRecordName: SetRecordName::<Identity, Impl, OFFSET>,
            GetStreamsForRecord: GetStreamsForRecord::<Identity, Impl, OFFSET>,
            AddStreamForRecord: AddStreamForRecord::<Identity, Impl, OFFSET>,
            RemoveStreamForRecord: RemoveStreamForRecord::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMOutputMediaProps_Impl: ::windows_core::BaseImpl + IWMMediaProps_Impl {
    fn GetStreamGroupName(this: &Self::This, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()>;
    fn GetConnectionName(this: &Self::This, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMOutputMediaProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMMediaProps);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMOutputMediaProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMOutputMediaProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStreamGroupName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMOutputMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamGroupName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn GetConnectionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMOutputMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectionName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        IWMOutputMediaProps_Vtbl {
            base__: <IWMMediaProps as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStreamGroupName: GetStreamGroupName::<Identity, Impl, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPacketSize_Impl: ::windows_core::BaseImpl {
    fn GetMaxPacketSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxPacketSize(this: &Self::This, dwmaxpacketsize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPacketSize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPacketSize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPacketSize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxPacketSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPacketSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxPacketSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxpacketsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxPacketSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPacketSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxPacketSize(this, ::core::mem::transmute_copy(&dwmaxpacketsize)).into())
        }
        IWMPacketSize_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxPacketSize: GetMaxPacketSize::<Identity, Impl, OFFSET>,
            SetMaxPacketSize: SetMaxPacketSize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPacketSize2_Impl: ::windows_core::BaseImpl + IWMPacketSize_Impl {
    fn GetMinPacketSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMinPacketSize(this: &Self::This, dwminpacketsize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPacketSize2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPacketSize);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPacketSize2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPacketSize2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMinPacketSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPacketSize2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinPacketSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwminpacketsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinPacketSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPacketSize2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinPacketSize(this, ::core::mem::transmute_copy(&dwminpacketsize)).into())
        }
        IWMPacketSize2_Vtbl {
            base__: <IWMPacketSize as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMinPacketSize: GetMinPacketSize::<Identity, Impl, OFFSET>,
            SetMinPacketSize: SetMinPacketSize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPlayerHook_Impl: ::windows_core::BaseImpl {
    fn PreDecode(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPlayerHook {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPlayerHook_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPlayerHook {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PreDecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPlayerHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreDecode(this).into())
        }
        IWMPlayerHook_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, PreDecode: PreDecode::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMPlayerTimestampHook_Impl: ::windows_core::BaseImpl {
    fn MapTimestamp(this: &Self::This, rtin: i64) -> ::windows_core::Result<i64>;
}
impl ::windows_core::Iids for IWMPlayerTimestampHook {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPlayerTimestampHook_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPlayerTimestampHook {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MapTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPlayerTimestampHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapTimestamp(this, ::core::mem::transmute_copy(&rtin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prtout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPlayerTimestampHook_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, MapTimestamp: MapTimestamp::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMProfile_Impl: ::windows_core::BaseImpl {
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<WMT_VERSION>;
    fn GetName(this: &Self::This, pwszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDescription(this: &Self::This, pwszdescription: ::windows_core::PWSTR, pcchdescription: *mut u32) -> ::windows_core::Result<()>;
    fn SetDescription(this: &Self::This, pwszdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStreamCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetStream(this: &Self::This, dwstreamindex: u32) -> ::windows_core::Result<IWMStreamConfig>;
    fn GetStreamByNumber(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<IWMStreamConfig>;
    fn RemoveStream(this: &Self::This, pconfig: ::core::option::Option<&IWMStreamConfig>) -> ::windows_core::Result<()>;
    fn RemoveStreamByNumber(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<()>;
    fn AddStream(this: &Self::This, pconfig: ::core::option::Option<&IWMStreamConfig>) -> ::windows_core::Result<()>;
    fn ReconfigStream(this: &Self::This, pconfig: ::core::option::Option<&IWMStreamConfig>) -> ::windows_core::Result<()>;
    fn CreateNewStream(this: &Self::This, guidstreamtype: *const ::windows_core::GUID) -> ::windows_core::Result<IWMStreamConfig>;
    fn GetMutualExclusionCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMutualExclusion(this: &Self::This, dwmeindex: u32) -> ::windows_core::Result<IWMMutualExclusion>;
    fn RemoveMutualExclusion(this: &Self::This, pme: ::core::option::Option<&IWMMutualExclusion>) -> ::windows_core::Result<()>;
    fn AddMutualExclusion(this: &Self::This, pme: ::core::option::Option<&IWMMutualExclusion>) -> ::windows_core::Result<()>;
    fn CreateNewMutualExclusion(this: &Self::This) -> ::windows_core::Result<IWMMutualExclusion>;
}
impl ::windows_core::Iids for IWMProfile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProfile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdescription: ::windows_core::PWSTR, pcchdescription: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescription(this, ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcchdescription)).into())
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&pwszdescription)).into())
        }
        unsafe extern "system" fn GetStreamCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcstreams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this, ::core::mem::transmute_copy(&dwstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreamByNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamByNumber(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStream(this, ::windows_core::from_raw_borrowed(&pconfig)).into())
        }
        unsafe extern "system" fn RemoveStreamByNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStreamByNumber(this, ::core::mem::transmute_copy(&wstreamnum)).into())
        }
        unsafe extern "system" fn AddStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStream(this, ::windows_core::from_raw_borrowed(&pconfig)).into())
        }
        unsafe extern "system" fn ReconfigStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReconfigStream(this, ::windows_core::from_raw_borrowed(&pconfig)).into())
        }
        unsafe extern "system" fn CreateNewStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows_core::GUID, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNewStream(this, ::core::mem::transmute_copy(&guidstreamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMutualExclusionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMutualExclusionCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcme, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMutualExclusion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMutualExclusion(this, ::core::mem::transmute_copy(&dwmeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppme, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveMutualExclusion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pme: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMutualExclusion(this, ::windows_core::from_raw_borrowed(&pme)).into())
        }
        unsafe extern "system" fn AddMutualExclusion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pme: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMutualExclusion(this, ::windows_core::from_raw_borrowed(&pme)).into())
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppme: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNewMutualExclusion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppme, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMProfile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetStreamByNumber: GetStreamByNumber::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            RemoveStreamByNumber: RemoveStreamByNumber::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            ReconfigStream: ReconfigStream::<Identity, Impl, OFFSET>,
            CreateNewStream: CreateNewStream::<Identity, Impl, OFFSET>,
            GetMutualExclusionCount: GetMutualExclusionCount::<Identity, Impl, OFFSET>,
            GetMutualExclusion: GetMutualExclusion::<Identity, Impl, OFFSET>,
            RemoveMutualExclusion: RemoveMutualExclusion::<Identity, Impl, OFFSET>,
            AddMutualExclusion: AddMutualExclusion::<Identity, Impl, OFFSET>,
            CreateNewMutualExclusion: CreateNewMutualExclusion::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMProfile2_Impl: ::windows_core::BaseImpl + IWMProfile_Impl {
    fn GetProfileID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IWMProfile2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMProfile);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProfile2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProfileID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProfileID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMProfile2_Vtbl { base__: <IWMProfile as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetProfileID: GetProfileID::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMProfile3_Impl: ::windows_core::BaseImpl + IWMProfile2_Impl {
    fn GetStorageFormat(this: &Self::This) -> ::windows_core::Result<WMT_STORAGE_FORMAT>;
    fn SetStorageFormat(this: &Self::This, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows_core::Result<()>;
    fn GetBandwidthSharingCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetBandwidthSharing(this: &Self::This, dwbsindex: u32) -> ::windows_core::Result<IWMBandwidthSharing>;
    fn RemoveBandwidthSharing(this: &Self::This, pbs: ::core::option::Option<&IWMBandwidthSharing>) -> ::windows_core::Result<()>;
    fn AddBandwidthSharing(this: &Self::This, pbs: ::core::option::Option<&IWMBandwidthSharing>) -> ::windows_core::Result<()>;
    fn CreateNewBandwidthSharing(this: &Self::This) -> ::windows_core::Result<IWMBandwidthSharing>;
    fn GetStreamPrioritization(this: &Self::This) -> ::windows_core::Result<IWMStreamPrioritization>;
    fn SetStreamPrioritization(this: &Self::This, psp: ::core::option::Option<&IWMStreamPrioritization>) -> ::windows_core::Result<()>;
    fn RemoveStreamPrioritization(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateNewStreamPrioritization(this: &Self::This) -> ::windows_core::Result<IWMStreamPrioritization>;
    fn GetExpectedPacketCount(this: &Self::This, msduration: u64) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IWMProfile3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMProfile2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProfile3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStorageFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorageFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnstorageformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStorageFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStorageFormat(this, ::core::mem::transmute_copy(&nstorageformat)).into())
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBandwidthSharingCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBandwidthSharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBandwidthSharing(this, ::core::mem::transmute_copy(&dwbsindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveBandwidthSharing(this, ::windows_core::from_raw_borrowed(&pbs)).into())
        }
        unsafe extern "system" fn AddBandwidthSharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddBandwidthSharing(this, ::windows_core::from_raw_borrowed(&pbs)).into())
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNewBandwidthSharing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreamPrioritization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamPrioritization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStreamPrioritization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamPrioritization(this, ::windows_core::from_raw_borrowed(&psp)).into())
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStreamPrioritization(this).into())
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNewStreamPrioritization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExpectedPacketCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExpectedPacketCount(this, ::core::mem::transmute_copy(&msduration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcpackets, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMProfile3_Vtbl {
            base__: <IWMProfile2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStorageFormat: GetStorageFormat::<Identity, Impl, OFFSET>,
            SetStorageFormat: SetStorageFormat::<Identity, Impl, OFFSET>,
            GetBandwidthSharingCount: GetBandwidthSharingCount::<Identity, Impl, OFFSET>,
            GetBandwidthSharing: GetBandwidthSharing::<Identity, Impl, OFFSET>,
            RemoveBandwidthSharing: RemoveBandwidthSharing::<Identity, Impl, OFFSET>,
            AddBandwidthSharing: AddBandwidthSharing::<Identity, Impl, OFFSET>,
            CreateNewBandwidthSharing: CreateNewBandwidthSharing::<Identity, Impl, OFFSET>,
            GetStreamPrioritization: GetStreamPrioritization::<Identity, Impl, OFFSET>,
            SetStreamPrioritization: SetStreamPrioritization::<Identity, Impl, OFFSET>,
            RemoveStreamPrioritization: RemoveStreamPrioritization::<Identity, Impl, OFFSET>,
            CreateNewStreamPrioritization: CreateNewStreamPrioritization::<Identity, Impl, OFFSET>,
            GetExpectedPacketCount: GetExpectedPacketCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMProfileManager_Impl: ::windows_core::BaseImpl {
    fn CreateEmptyProfile(this: &Self::This, dwversion: WMT_VERSION) -> ::windows_core::Result<IWMProfile>;
    fn LoadProfileByID(this: &Self::This, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<IWMProfile>;
    fn LoadProfileByData(this: &Self::This, pwszprofile: &::windows_core::PCWSTR) -> ::windows_core::Result<IWMProfile>;
    fn SaveProfile(this: &Self::This, piwmprofile: ::core::option::Option<&IWMProfile>, pwszprofile: &::windows_core::PCWSTR, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetSystemProfileCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn LoadSystemProfile(this: &Self::This, dwprofileindex: u32) -> ::windows_core::Result<IWMProfile>;
}
impl ::windows_core::Iids for IWMProfileManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProfileManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEmptyProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEmptyProfile(this, ::core::mem::transmute_copy(&dwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadProfileByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows_core::GUID, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadProfileByID(this, ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadProfileByData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprofile: ::windows_core::PCWSTR, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadProfileByData(this, ::core::mem::transmute(&pwszprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmprofile: *mut ::core::ffi::c_void, pwszprofile: ::windows_core::PCWSTR, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveProfile(this, ::windows_core::from_raw_borrowed(&piwmprofile), ::core::mem::transmute(&pwszprofile), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn GetSystemProfileCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemProfileCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprofiles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadSystemProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadSystemProfile(this, ::core::mem::transmute_copy(&dwprofileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMProfileManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEmptyProfile: CreateEmptyProfile::<Identity, Impl, OFFSET>,
            LoadProfileByID: LoadProfileByID::<Identity, Impl, OFFSET>,
            LoadProfileByData: LoadProfileByData::<Identity, Impl, OFFSET>,
            SaveProfile: SaveProfile::<Identity, Impl, OFFSET>,
            GetSystemProfileCount: GetSystemProfileCount::<Identity, Impl, OFFSET>,
            LoadSystemProfile: LoadSystemProfile::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMProfileManager2_Impl: ::windows_core::BaseImpl + IWMProfileManager_Impl {
    fn GetSystemProfileVersion(this: &Self::This, pdwversion: *mut WMT_VERSION) -> ::windows_core::Result<()>;
    fn SetSystemProfileVersion(this: &Self::This, dwversion: WMT_VERSION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMProfileManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMProfileManager);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProfileManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSystemProfileVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSystemProfileVersion(this, ::core::mem::transmute_copy(&pdwversion)).into())
        }
        unsafe extern "system" fn SetSystemProfileVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSystemProfileVersion(this, ::core::mem::transmute_copy(&dwversion)).into())
        }
        IWMProfileManager2_Vtbl {
            base__: <IWMProfileManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSystemProfileVersion: GetSystemProfileVersion::<Identity, Impl, OFFSET>,
            SetSystemProfileVersion: SetSystemProfileVersion::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMProfileManagerLanguage_Impl: ::windows_core::BaseImpl {
    fn GetUserLanguageID(this: &Self::This, wlangid: *mut u16) -> ::windows_core::Result<()>;
    fn SetUserLanguageID(this: &Self::This, wlangid: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMProfileManagerLanguage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProfileManagerLanguage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUserLanguageID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserLanguageID(this, ::core::mem::transmute_copy(&wlangid)).into())
        }
        unsafe extern "system" fn SetUserLanguageID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserLanguageID(this, ::core::mem::transmute_copy(&wlangid)).into())
        }
        IWMProfileManagerLanguage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUserLanguageID: GetUserLanguageID::<Identity, Impl, OFFSET>,
            SetUserLanguageID: SetUserLanguageID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPropertyVault_Impl: ::windows_core::BaseImpl {
    fn GetPropertyCount(this: &Self::This, pdwcount: *const u32) -> ::windows_core::Result<()>;
    fn GetPropertyByName(this: &Self::This, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, pszname: &::windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::Result<()>;
    fn GetPropertyByIndex(this: &Self::This, dwindex: u32, pszname: ::windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
    fn CopyPropertiesFrom(this: &Self::This, piwmpropertyvault: ::core::option::Option<&IWMPropertyVault>) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPropertyVault {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPropertyVault {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyCount(this, ::core::mem::transmute_copy(&pdwcount)).into())
        }
        unsafe extern "system" fn GetPropertyByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyByName(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into())
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: ::windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyByIndex(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pdwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        unsafe extern "system" fn CopyPropertiesFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpropertyvault: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyPropertiesFrom(this, ::windows_core::from_raw_borrowed(&piwmpropertyvault)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IWMPropertyVault_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyByName: GetPropertyByName::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, Impl, OFFSET>,
            CopyPropertiesFrom: CopyPropertiesFrom::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMProximityDetection_Impl: ::windows_core::BaseImpl {
    fn StartDetection(this: &Self::This, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMProximityDetection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProximityDetection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMProximityDetection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartDetection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMProximityDetection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartDetection(this, ::core::mem::transmute_copy(&pbregistrationmsg), ::core::mem::transmute_copy(&cbregistrationmsg), ::core::mem::transmute_copy(&pblocaladdress), ::core::mem::transmute_copy(&cblocaladdress), ::core::mem::transmute_copy(&dwextraportsallowed), ::core::mem::transmute_copy(&ppregistrationresponsemsg), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMProximityDetection_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, StartDetection: StartDetection::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMReader_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pcallback: ::core::option::Option<&IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetOutputCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOutputProps(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(this: &Self::This, dwoutputnum: u32, poutput: ::core::option::Option<&IWMOutputMediaProps>) -> ::windows_core::Result<()>;
    fn GetOutputFormatCount(this: &Self::This, dwoutputnumber: u32) -> ::windows_core::Result<u32>;
    fn GetOutputFormat(this: &Self::This, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows_core::Result<IWMOutputMediaProps>;
    fn Start(this: &Self::This, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pwszurl), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoutputs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputProps(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputProps(this, ::core::mem::transmute_copy(&dwoutputnum), ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputFormatCount(this, ::core::mem::transmute_copy(&dwoutputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputFormat(this, ::core::mem::transmute_copy(&dwoutputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        IWMReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, Impl, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, Impl, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, Impl, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAccelerator_Impl: ::windows_core::BaseImpl {
    fn GetCodecInterface(this: &Self::This, dwoutputnum: u32, riid: *const ::windows_core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMReaderAccelerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAccelerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAccelerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCodecInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAccelerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows_core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodecInterface(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcodecinterface)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAccelerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&psubtype)).into())
        }
        IWMReaderAccelerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCodecInterface: GetCodecInterface::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAdvanced_Impl: ::windows_core::BaseImpl {
    fn SetUserProvidedClock(this: &Self::This, fuserclock: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetUserProvidedClock(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DeliverTime(this: &Self::This, cnstime: u64) -> ::windows_core::Result<()>;
    fn SetManualStreamSelection(this: &Self::This, fselection: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetManualStreamSelection(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetStreamsSelected(this: &Self::This, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()>;
    fn GetStreamSelected(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION>;
    fn SetReceiveSelectionCallbacks(this: &Self::This, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReceiveSelectionCallbacks(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetReceiveStreamSamples(this: &Self::This, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReceiveStreamSamples(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForOutput(this: &Self::This, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAllocateForOutput(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForStream(this: &Self::This, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAllocateForStream(this: &Self::This, dwsreamnum: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetStatistics(this: &Self::This, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()>;
    fn SetClientInfo(this: &Self::This, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()>;
    fn GetMaxOutputSampleSize(this: &Self::This, dwoutput: u32) -> ::windows_core::Result<u32>;
    fn GetMaxStreamSampleSize(this: &Self::This, wstream: u16) -> ::windows_core::Result<u32>;
    fn NotifyLateDelivery(this: &Self::This, cnslateness: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMReaderAdvanced {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAdvanced {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetUserProvidedClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserProvidedClock(this, ::core::mem::transmute_copy(&fuserclock)).into())
        }
        unsafe extern "system" fn GetUserProvidedClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserProvidedClock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfuserclock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeliverTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeliverTime(this, ::core::mem::transmute_copy(&cnstime)).into())
        }
        unsafe extern "system" fn SetManualStreamSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManualStreamSelection(this, ::core::mem::transmute_copy(&fselection)).into())
        }
        unsafe extern "system" fn GetManualStreamSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetManualStreamSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfselection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamsSelected(this, ::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into())
        }
        unsafe extern "system" fn GetStreamSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamSelected(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceiveSelectionCallbacks(this, ::core::mem::transmute_copy(&fgetcallbacks)).into())
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReceiveSelectionCallbacks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfgetcallbacks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceiveStreamSamples(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivestreamsamples)).into())
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReceiveStreamSamples(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfreceivestreamsamples, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocateForOutput(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&fallocate)).into())
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocateForOutput(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallocate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocateForStream(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fallocate)).into())
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocateForStream(this, ::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallocate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatistics(this, ::core::mem::transmute_copy(&pstatistics)).into())
        }
        unsafe extern "system" fn SetClientInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientInfo(this, ::core::mem::transmute_copy(&pclientinfo)).into())
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxOutputSampleSize(this, ::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxStreamSampleSize(this, ::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyLateDelivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyLateDelivery(this, ::core::mem::transmute_copy(&cnslateness)).into())
        }
        IWMReaderAdvanced_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetUserProvidedClock: SetUserProvidedClock::<Identity, Impl, OFFSET>,
            GetUserProvidedClock: GetUserProvidedClock::<Identity, Impl, OFFSET>,
            DeliverTime: DeliverTime::<Identity, Impl, OFFSET>,
            SetManualStreamSelection: SetManualStreamSelection::<Identity, Impl, OFFSET>,
            GetManualStreamSelection: GetManualStreamSelection::<Identity, Impl, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, Impl, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, Impl, OFFSET>,
            SetReceiveSelectionCallbacks: SetReceiveSelectionCallbacks::<Identity, Impl, OFFSET>,
            GetReceiveSelectionCallbacks: GetReceiveSelectionCallbacks::<Identity, Impl, OFFSET>,
            SetReceiveStreamSamples: SetReceiveStreamSamples::<Identity, Impl, OFFSET>,
            GetReceiveStreamSamples: GetReceiveStreamSamples::<Identity, Impl, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, Impl, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, Impl, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, Impl, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            SetClientInfo: SetClientInfo::<Identity, Impl, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, Impl, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, Impl, OFFSET>,
            NotifyLateDelivery: NotifyLateDelivery::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced2_Impl: ::windows_core::BaseImpl + IWMReaderAdvanced_Impl {
    fn SetPlayMode(this: &Self::This, mode: WMT_PLAY_MODE) -> ::windows_core::Result<()>;
    fn GetPlayMode(this: &Self::This) -> ::windows_core::Result<WMT_PLAY_MODE>;
    fn GetBufferProgress(this: &Self::This, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::Result<()>;
    fn GetDownloadProgress(this: &Self::This, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::Result<()>;
    fn GetSaveAsProgress(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SaveFileAs(this: &Self::This, pwszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetProtocolName(this: &Self::This, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::Result<()>;
    fn StartAtMarker(this: &Self::This, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetOutputSetting(this: &Self::This, dwoutputnum: u32, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn SetOutputSetting(this: &Self::This, dwoutputnum: u32, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
    fn Preroll(this: &Self::This, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::Result<()>;
    fn SetLogClientID(this: &Self::This, flogclientid: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLogClientID(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn StopBuffering(this: &Self::This) -> ::windows_core::Result<()>;
    fn OpenStream(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, pcallback: ::core::option::Option<&IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMReaderAdvanced2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMReaderAdvanced);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAdvanced2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPlayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn GetPlayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPlayMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBufferProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferProgress(this, ::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pcnsbuffering)).into())
        }
        unsafe extern "system" fn GetDownloadProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDownloadProgress(this, ::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pqwbytesdownloaded), ::core::mem::transmute_copy(&pcnsdownload)).into())
        }
        unsafe extern "system" fn GetSaveAsProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSaveAsProgress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpercent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveFileAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveFileAs(this, ::core::mem::transmute(&pwszfilename)).into())
        }
        unsafe extern "system" fn GetProtocolName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtocolName(this, ::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pcchprotocol)).into())
        }
        unsafe extern "system" fn StartAtMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartAtMarker(this, ::core::mem::transmute_copy(&wmarkerindex), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn GetOutputSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputSetting(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn SetOutputSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputSetting(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        unsafe extern "system" fn Preroll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Preroll(this, ::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate)).into())
        }
        unsafe extern "system" fn SetLogClientID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogClientID(this, ::core::mem::transmute_copy(&flogclientid)).into())
        }
        unsafe extern "system" fn GetLogClientID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLogClientID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflogclientid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StopBuffering<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopBuffering(this).into())
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenStream(this, ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMReaderAdvanced2_Vtbl {
            base__: <IWMReaderAdvanced as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPlayMode: SetPlayMode::<Identity, Impl, OFFSET>,
            GetPlayMode: GetPlayMode::<Identity, Impl, OFFSET>,
            GetBufferProgress: GetBufferProgress::<Identity, Impl, OFFSET>,
            GetDownloadProgress: GetDownloadProgress::<Identity, Impl, OFFSET>,
            GetSaveAsProgress: GetSaveAsProgress::<Identity, Impl, OFFSET>,
            SaveFileAs: SaveFileAs::<Identity, Impl, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, Impl, OFFSET>,
            StartAtMarker: StartAtMarker::<Identity, Impl, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, Impl, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, Impl, OFFSET>,
            Preroll: Preroll::<Identity, Impl, OFFSET>,
            SetLogClientID: SetLogClientID::<Identity, Impl, OFFSET>,
            GetLogClientID: GetLogClientID::<Identity, Impl, OFFSET>,
            StopBuffering: StopBuffering::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced3_Impl: ::windows_core::BaseImpl + IWMReaderAdvanced2_Impl {
    fn StopNetStreaming(this: &Self::This) -> ::windows_core::Result<()>;
    fn StartAtPosition(this: &Self::This, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMReaderAdvanced3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMReaderAdvanced2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAdvanced3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StopNetStreaming<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopNetStreaming(this).into())
        }
        unsafe extern "system" fn StartAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartAtPosition(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pvoffsetstart), ::core::mem::transmute_copy(&pvduration), ::core::mem::transmute_copy(&dwoffsetformat), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMReaderAdvanced3_Vtbl {
            base__: <IWMReaderAdvanced2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StopNetStreaming: StopNetStreaming::<Identity, Impl, OFFSET>,
            StartAtPosition: StartAtPosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced4_Impl: ::windows_core::BaseImpl + IWMReaderAdvanced3_Impl {
    fn GetLanguageCount(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<u16>;
    fn GetLanguage(this: &Self::This, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()>;
    fn GetMaxSpeedFactor(this: &Self::This) -> ::windows_core::Result<f64>;
    fn IsUsingFastCache(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn AddLogParam(this: &Self::This, wsznamespace: &::windows_core::PCWSTR, wszname: &::windows_core::PCWSTR, wszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SendLogParams(this: &Self::This) -> ::windows_core::Result<()>;
    fn CanSaveFileAs(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CancelSaveFileAs(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetURL(this: &Self::This, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMReaderAdvanced4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMReaderAdvanced3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAdvanced4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLanguageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguageCount(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwlanguagecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLanguage(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into())
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxSpeedFactor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdblfactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUsingFastCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUsingFastCache(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfusingfastcache, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddLogParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznamespace: ::windows_core::PCWSTR, wszname: ::windows_core::PCWSTR, wszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLogParam(this, ::core::mem::transmute(&wsznamespace), ::core::mem::transmute(&wszname), ::core::mem::transmute(&wszvalue)).into())
        }
        unsafe extern "system" fn SendLogParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendLogParams(this).into())
        }
        unsafe extern "system" fn CanSaveFileAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanSaveFileAs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcansave, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CancelSaveFileAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelSaveFileAs(this).into())
        }
        unsafe extern "system" fn GetURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetURL(this, ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into())
        }
        IWMReaderAdvanced4_Vtbl {
            base__: <IWMReaderAdvanced3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLanguageCount: GetLanguageCount::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            GetMaxSpeedFactor: GetMaxSpeedFactor::<Identity, Impl, OFFSET>,
            IsUsingFastCache: IsUsingFastCache::<Identity, Impl, OFFSET>,
            AddLogParam: AddLogParam::<Identity, Impl, OFFSET>,
            SendLogParams: SendLogParams::<Identity, Impl, OFFSET>,
            CanSaveFileAs: CanSaveFileAs::<Identity, Impl, OFFSET>,
            CancelSaveFileAs: CancelSaveFileAs::<Identity, Impl, OFFSET>,
            GetURL: GetURL::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced5_Impl: ::windows_core::BaseImpl + IWMReaderAdvanced4_Impl {
    fn SetPlayerHook(this: &Self::This, dwoutputnum: u32, phook: ::core::option::Option<&IWMPlayerHook>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMReaderAdvanced5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMReaderAdvanced4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAdvanced5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPlayerHook<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayerHook(this, ::core::mem::transmute_copy(&dwoutputnum), ::windows_core::from_raw_borrowed(&phook)).into())
        }
        IWMReaderAdvanced5_Vtbl { base__: <IWMReaderAdvanced4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetPlayerHook: SetPlayerHook::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced6_Impl: ::windows_core::BaseImpl + IWMReaderAdvanced5_Impl {
    fn SetProtectStreamSamples(this: &Self::This, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMReaderAdvanced6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMReaderAdvanced5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAdvanced6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAdvanced6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtectStreamSamples(this, ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&dwcertificatetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbinitializationvector), ::core::mem::transmute_copy(&pcbinitializationvector)).into())
        }
        IWMReaderAdvanced6_Vtbl {
            base__: <IWMReaderAdvanced5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProtectStreamSamples: SetProtectStreamSamples::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMReaderAllocatorEx_Impl: ::windows_core::BaseImpl {
    fn AllocateForStreamEx(this: &Self::This, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn AllocateForOutputEx(this: &Self::This, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReaderAllocatorEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderAllocatorEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllocateForStreamEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateForStreamEx(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn AllocateForOutputEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateForOutputEx(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMReaderAllocatorEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllocateForStreamEx: AllocateForStreamEx::<Identity, Impl, OFFSET>,
            AllocateForOutputEx: AllocateForOutputEx::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMReaderCallback_Impl: ::windows_core::BaseImpl + IWMStatusCallback_Impl {
    fn OnSample(this: &Self::This, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReaderCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMStatusCallback);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSample(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&psample), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMReaderCallback_Vtbl { base__: <IWMStatusCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnSample: OnSample::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderCallbackAdvanced_Impl: ::windows_core::BaseImpl {
    fn OnStreamSample(this: &Self::This, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OnTime(this: &Self::This, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OnStreamSelection(this: &Self::This, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OnOutputPropsChanged(this: &Self::This, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn AllocateForStream(this: &Self::This, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn AllocateForOutput(this: &Self::This, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMReaderCallbackAdvanced {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderCallbackAdvanced {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStreamSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStreamSample(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&psample), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn OnTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTime(this, ::core::mem::transmute_copy(&cnscurrenttime), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn OnStreamSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStreamSelection(this, ::core::mem::transmute_copy(&wstreamcount), ::core::mem::transmute_copy(&pstreamnumbers), ::core::mem::transmute_copy(&pselections), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn OnOutputPropsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutputPropsChanged(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pmediatype), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn AllocateForStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateForStream(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn AllocateForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateForOutput(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMReaderCallbackAdvanced_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStreamSample: OnStreamSample::<Identity, Impl, OFFSET>,
            OnTime: OnTime::<Identity, Impl, OFFSET>,
            OnStreamSelection: OnStreamSelection::<Identity, Impl, OFFSET>,
            OnOutputPropsChanged: OnOutputPropsChanged::<Identity, Impl, OFFSET>,
            AllocateForStream: AllocateForStream::<Identity, Impl, OFFSET>,
            AllocateForOutput: AllocateForOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig_Impl: ::windows_core::BaseImpl {
    fn GetBufferingTime(this: &Self::This) -> ::windows_core::Result<u64>;
    fn SetBufferingTime(this: &Self::This, cnsbufferingtime: u64) -> ::windows_core::Result<()>;
    fn GetUDPPortRanges(this: &Self::This, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows_core::Result<()>;
    fn SetUDPPortRanges(this: &Self::This, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows_core::Result<()>;
    fn GetProxySettings(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR) -> ::windows_core::Result<WMT_PROXY_SETTINGS>;
    fn SetProxySettings(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows_core::Result<()>;
    fn GetProxyHostName(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, pwszhostname: ::windows_core::PWSTR, pcchhostname: *mut u32) -> ::windows_core::Result<()>;
    fn SetProxyHostName(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, pwszhostname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetProxyPort(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn SetProxyPort(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, dwport: u32) -> ::windows_core::Result<()>;
    fn GetProxyExceptionList(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, pwszexceptionlist: ::windows_core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows_core::Result<()>;
    fn SetProxyExceptionList(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, pwszexceptionlist: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetProxyBypassForLocal(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetProxyBypassForLocal(this: &Self::This, pwszprotocol: &::windows_core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetForceRerunAutoProxyDetection(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetForceRerunAutoProxyDetection(this: &Self::This, fforcererundetection: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnableMulticast(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableMulticast(this: &Self::This, fenablemulticast: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnableHTTP(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableHTTP(this: &Self::This, fenablehttp: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnableUDP(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableUDP(this: &Self::This, fenableudp: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnableTCP(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableTCP(this: &Self::This, fenabletcp: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ResetProtocolRollover(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetConnectionBandwidth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetConnectionBandwidth(this: &Self::This, dwconnectionbandwidth: u32) -> ::windows_core::Result<()>;
    fn GetNumProtocolsSupported(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSupportedProtocolName(this: &Self::This, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u32) -> ::windows_core::Result<()>;
    fn AddLoggingUrl(this: &Self::This, pwszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLoggingUrl(this: &Self::This, dwindex: u32, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()>;
    fn GetLoggingUrlCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ResetLoggingUrlList(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMReaderNetworkConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderNetworkConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBufferingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBufferingTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsbufferingtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferingTime(this, ::core::mem::transmute_copy(&cnsbufferingtime)).into())
        }
        unsafe extern "system" fn GetUDPPortRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUDPPortRanges(this, ::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&pcranges)).into())
        }
        unsafe extern "system" fn SetUDPPortRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUDPPortRanges(this, ::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&cranges)).into())
        }
        unsafe extern "system" fn GetProxySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProxySettings(this, ::core::mem::transmute(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproxysetting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxySettings(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&proxysetting)).into())
        }
        unsafe extern "system" fn GetProxyHostName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszhostname: ::windows_core::PWSTR, pcchhostname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProxyHostName(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&pwszhostname), ::core::mem::transmute_copy(&pcchhostname)).into())
        }
        unsafe extern "system" fn SetProxyHostName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszhostname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyHostName(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute(&pwszhostname)).into())
        }
        unsafe extern "system" fn GetProxyPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pdwport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProxyPort(this, ::core::mem::transmute(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxyPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, dwport: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyPort(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&dwport)).into())
        }
        unsafe extern "system" fn GetProxyExceptionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszexceptionlist: ::windows_core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProxyExceptionList(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&pwszexceptionlist), ::core::mem::transmute_copy(&pcchexceptionlist)).into())
        }
        unsafe extern "system" fn SetProxyExceptionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszexceptionlist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyExceptionList(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute(&pwszexceptionlist)).into())
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProxyBypassForLocal(this, ::core::mem::transmute(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfbypassforlocal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyBypassForLocal(this, ::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into())
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetForceRerunAutoProxyDetection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforcererundetection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForceRerunAutoProxyDetection(this, ::core::mem::transmute_copy(&fforcererundetection)).into())
        }
        unsafe extern "system" fn GetEnableMulticast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableMulticast(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablemulticast, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableMulticast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableMulticast(this, ::core::mem::transmute_copy(&fenablemulticast)).into())
        }
        unsafe extern "system" fn GetEnableHTTP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableHTTP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablehttp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableHTTP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableHTTP(this, ::core::mem::transmute_copy(&fenablehttp)).into())
        }
        unsafe extern "system" fn GetEnableUDP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableUDP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenableudp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableUDP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableUDP(this, ::core::mem::transmute_copy(&fenableudp)).into())
        }
        unsafe extern "system" fn GetEnableTCP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableTCP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabletcp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableTCP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableTCP(this, ::core::mem::transmute_copy(&fenabletcp)).into())
        }
        unsafe extern "system" fn ResetProtocolRollover<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetProtocolRollover(this).into())
        }
        unsafe extern "system" fn GetConnectionBandwidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionBandwidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnectionbandwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectionBandwidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectionBandwidth(this, ::core::mem::transmute_copy(&dwconnectionbandwidth)).into())
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumProtocolsSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprotocols, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedProtocolName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSupportedProtocolName(this, ::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into())
        }
        unsafe extern "system" fn AddLoggingUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLoggingUrl(this, ::core::mem::transmute(&pwszurl)).into())
        }
        unsafe extern "system" fn GetLoggingUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLoggingUrl(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into())
        }
        unsafe extern "system" fn GetLoggingUrlCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLoggingUrlCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwurlcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResetLoggingUrlList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetLoggingUrlList(this).into())
        }
        IWMReaderNetworkConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBufferingTime: GetBufferingTime::<Identity, Impl, OFFSET>,
            SetBufferingTime: SetBufferingTime::<Identity, Impl, OFFSET>,
            GetUDPPortRanges: GetUDPPortRanges::<Identity, Impl, OFFSET>,
            SetUDPPortRanges: SetUDPPortRanges::<Identity, Impl, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, Impl, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, Impl, OFFSET>,
            GetProxyHostName: GetProxyHostName::<Identity, Impl, OFFSET>,
            SetProxyHostName: SetProxyHostName::<Identity, Impl, OFFSET>,
            GetProxyPort: GetProxyPort::<Identity, Impl, OFFSET>,
            SetProxyPort: SetProxyPort::<Identity, Impl, OFFSET>,
            GetProxyExceptionList: GetProxyExceptionList::<Identity, Impl, OFFSET>,
            SetProxyExceptionList: SetProxyExceptionList::<Identity, Impl, OFFSET>,
            GetProxyBypassForLocal: GetProxyBypassForLocal::<Identity, Impl, OFFSET>,
            SetProxyBypassForLocal: SetProxyBypassForLocal::<Identity, Impl, OFFSET>,
            GetForceRerunAutoProxyDetection: GetForceRerunAutoProxyDetection::<Identity, Impl, OFFSET>,
            SetForceRerunAutoProxyDetection: SetForceRerunAutoProxyDetection::<Identity, Impl, OFFSET>,
            GetEnableMulticast: GetEnableMulticast::<Identity, Impl, OFFSET>,
            SetEnableMulticast: SetEnableMulticast::<Identity, Impl, OFFSET>,
            GetEnableHTTP: GetEnableHTTP::<Identity, Impl, OFFSET>,
            SetEnableHTTP: SetEnableHTTP::<Identity, Impl, OFFSET>,
            GetEnableUDP: GetEnableUDP::<Identity, Impl, OFFSET>,
            SetEnableUDP: SetEnableUDP::<Identity, Impl, OFFSET>,
            GetEnableTCP: GetEnableTCP::<Identity, Impl, OFFSET>,
            SetEnableTCP: SetEnableTCP::<Identity, Impl, OFFSET>,
            ResetProtocolRollover: ResetProtocolRollover::<Identity, Impl, OFFSET>,
            GetConnectionBandwidth: GetConnectionBandwidth::<Identity, Impl, OFFSET>,
            SetConnectionBandwidth: SetConnectionBandwidth::<Identity, Impl, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, Impl, OFFSET>,
            GetSupportedProtocolName: GetSupportedProtocolName::<Identity, Impl, OFFSET>,
            AddLoggingUrl: AddLoggingUrl::<Identity, Impl, OFFSET>,
            GetLoggingUrl: GetLoggingUrl::<Identity, Impl, OFFSET>,
            GetLoggingUrlCount: GetLoggingUrlCount::<Identity, Impl, OFFSET>,
            ResetLoggingUrlList: ResetLoggingUrlList::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig2_Impl: ::windows_core::BaseImpl + IWMReaderNetworkConfig_Impl {
    fn GetEnableContentCaching(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableContentCaching(this: &Self::This, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnableFastCache(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableFastCache(this: &Self::This, fenablefastcache: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAcceleratedStreamingDuration(this: &Self::This) -> ::windows_core::Result<u64>;
    fn SetAcceleratedStreamingDuration(this: &Self::This, cnsaccelduration: u64) -> ::windows_core::Result<()>;
    fn GetAutoReconnectLimit(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetAutoReconnectLimit(this: &Self::This, dwautoreconnectlimit: u32) -> ::windows_core::Result<()>;
    fn GetEnableResends(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableResends(this: &Self::This, fenableresends: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnableThinning(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableThinning(this: &Self::This, fenablethinning: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMaxNetPacketSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMReaderNetworkConfig2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMReaderNetworkConfig);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderNetworkConfig2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEnableContentCaching<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableContentCaching(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablecontentcaching, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableContentCaching<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableContentCaching(this, ::core::mem::transmute_copy(&fenablecontentcaching)).into())
        }
        unsafe extern "system" fn GetEnableFastCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableFastCache(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablefastcache, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableFastCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableFastCache(this, ::core::mem::transmute_copy(&fenablefastcache)).into())
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAcceleratedStreamingDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsaccelduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAcceleratedStreamingDuration(this, ::core::mem::transmute_copy(&cnsaccelduration)).into())
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutoReconnectLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwautoreconnectlimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoReconnectLimit(this, ::core::mem::transmute_copy(&dwautoreconnectlimit)).into())
        }
        unsafe extern "system" fn GetEnableResends<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableResends(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenableresends, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableResends<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableResends(this, ::core::mem::transmute_copy(&fenableresends)).into())
        }
        unsafe extern "system" fn GetEnableThinning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnableThinning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablethinning, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableThinning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableThinning(this, ::core::mem::transmute_copy(&fenablethinning)).into())
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxNetPacketSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxnetpacketsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMReaderNetworkConfig2_Vtbl {
            base__: <IWMReaderNetworkConfig as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEnableContentCaching: GetEnableContentCaching::<Identity, Impl, OFFSET>,
            SetEnableContentCaching: SetEnableContentCaching::<Identity, Impl, OFFSET>,
            GetEnableFastCache: GetEnableFastCache::<Identity, Impl, OFFSET>,
            SetEnableFastCache: SetEnableFastCache::<Identity, Impl, OFFSET>,
            GetAcceleratedStreamingDuration: GetAcceleratedStreamingDuration::<Identity, Impl, OFFSET>,
            SetAcceleratedStreamingDuration: SetAcceleratedStreamingDuration::<Identity, Impl, OFFSET>,
            GetAutoReconnectLimit: GetAutoReconnectLimit::<Identity, Impl, OFFSET>,
            SetAutoReconnectLimit: SetAutoReconnectLimit::<Identity, Impl, OFFSET>,
            GetEnableResends: GetEnableResends::<Identity, Impl, OFFSET>,
            SetEnableResends: SetEnableResends::<Identity, Impl, OFFSET>,
            GetEnableThinning: GetEnableThinning::<Identity, Impl, OFFSET>,
            SetEnableThinning: SetEnableThinning::<Identity, Impl, OFFSET>,
            GetMaxNetPacketSize: GetMaxNetPacketSize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMReaderPlaylistBurn_Impl: ::windows_core::BaseImpl {
    fn InitPlaylistBurn(this: &Self::This, cfiles: u32, ppwszfilenames: *const ::windows_core::PCWSTR, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetInitResults(this: &Self::This, cfiles: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndPlaylistBurn(this: &Self::This, hrburnresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReaderPlaylistBurn {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderPlaylistBurn {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitPlaylistBurn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const ::windows_core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitPlaylistBurn(this, ::core::mem::transmute_copy(&cfiles), ::core::mem::transmute_copy(&ppwszfilenames), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn GetInitResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInitResults(this, ::core::mem::transmute_copy(&cfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrstati, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn EndPlaylistBurn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrburnresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndPlaylistBurn(this, ::core::mem::transmute_copy(&hrburnresult)).into())
        }
        IWMReaderPlaylistBurn_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitPlaylistBurn: InitPlaylistBurn::<Identity, Impl, OFFSET>,
            GetInitResults: GetInitResults::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EndPlaylistBurn: EndPlaylistBurn::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMReaderStreamClock_Impl: ::windows_core::BaseImpl {
    fn GetTime(this: &Self::This, pcnsnow: *const u64) -> ::windows_core::Result<()>;
    fn SetTimer(this: &Self::This, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows_core::Result<u32>;
    fn KillTimer(this: &Self::This, dwtimerid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReaderStreamClock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderStreamClock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTime(this, ::core::mem::transmute_copy(&pcnsnow)).into())
        }
        unsafe extern "system" fn SetTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetTimer(this, ::core::mem::transmute_copy(&cnswhen), ::core::mem::transmute_copy(&pvparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtimerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KillTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KillTimer(this, ::core::mem::transmute_copy(&dwtimerid)).into())
        }
        IWMReaderStreamClock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            SetTimer: SetTimer::<Identity, Impl, OFFSET>,
            KillTimer: KillTimer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMReaderTimecode_Impl: ::windows_core::BaseImpl {
    fn GetTimecodeRangeCount(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<u16>;
    fn GetTimecodeRangeBounds(this: &Self::This, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReaderTimecode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderTimecode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderTimecode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTimecodeRangeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderTimecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimecodeRangeCount(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwrangecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderTimecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTimecodeRangeBounds(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&wrangenum), ::core::mem::transmute_copy(&pstarttimecode), ::core::mem::transmute_copy(&pendtimecode)).into())
        }
        IWMReaderTimecode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTimecodeRangeCount: GetTimecodeRangeCount::<Identity, Impl, OFFSET>,
            GetTimecodeRangeBounds: GetTimecodeRangeBounds::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMReaderTypeNegotiation_Impl: ::windows_core::BaseImpl {
    fn TryOutputProps(this: &Self::This, dwoutputnum: u32, poutput: ::core::option::Option<&IWMOutputMediaProps>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMReaderTypeNegotiation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderTypeNegotiation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMReaderTypeNegotiation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TryOutputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMReaderTypeNegotiation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TryOutputProps(this, ::core::mem::transmute_copy(&dwoutputnum), ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        IWMReaderTypeNegotiation_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TryOutputProps: TryOutputProps::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMRegisterCallback_Impl: ::windows_core::BaseImpl {
    fn Advise(this: &Self::This, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Unadvise(this: &Self::This, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMRegisterCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisterCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMRegisterCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisterCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Advise(this, ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisterCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMRegisterCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMRegisteredDevice_Impl: ::windows_core::BaseImpl {
    fn GetDeviceSerialNumber(this: &Self::This) -> ::windows_core::Result<DRM_VAL16>;
    fn GetDeviceCertificate(this: &Self::This) -> ::windows_core::Result<INSSBuffer>;
    fn GetDeviceType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAttributeCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAttributeByIndex(this: &Self::This, dwindex: u32, pbstrname: *mut ::windows_core::BSTR, pbstrvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetAttributeByName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAttributeByName(this: &Self::This, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Approve(this: &Self::This, fapprove: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsValid(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsApproved(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsWmdrmCompliant(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOpened(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Open(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMRegisteredDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMRegisteredDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceSerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceSerialNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pserialnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcertificate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcertificate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeByIndex(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrvalue)).into())
        }
        unsafe extern "system" fn GetAttributeByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeByName(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttributeByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributeByName(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn Approve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Approve(this, ::core::mem::transmute_copy(&fapprove)).into())
        }
        unsafe extern "system" fn IsValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsValid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvalid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsApproved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsApproved(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfapproved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsWmdrmCompliant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWmdrmCompliant(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcompliant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOpened<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpened(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfopened, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IWMRegisteredDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceSerialNumber: GetDeviceSerialNumber::<Identity, Impl, OFFSET>,
            GetDeviceCertificate: GetDeviceCertificate::<Identity, Impl, OFFSET>,
            GetDeviceType: GetDeviceType::<Identity, Impl, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, Impl, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, Impl, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, Impl, OFFSET>,
            SetAttributeByName: SetAttributeByName::<Identity, Impl, OFFSET>,
            Approve: Approve::<Identity, Impl, OFFSET>,
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            IsApproved: IsApproved::<Identity, Impl, OFFSET>,
            IsWmdrmCompliant: IsWmdrmCompliant::<Identity, Impl, OFFSET>,
            IsOpened: IsOpened::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMSBufferAllocator_Impl: ::windows_core::BaseImpl {
    fn AllocateBuffer(this: &Self::This, dwmaxbuffersize: u32) -> ::windows_core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(this: &Self::This, dwmaxbuffersize: u32) -> ::windows_core::Result<INSSBuffer>;
}
impl ::windows_core::Iids for IWMSBufferAllocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSBufferAllocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSBufferAllocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSBufferAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocateBuffer(this, ::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSBufferAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocatePageSizeBuffer(this, ::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMSBufferAllocator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            AllocatePageSizeBuffer: AllocatePageSizeBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, psharednamespace: ::core::option::Option<&::windows_core::IUnknown>, pnamespacenode: ::core::option::Option<&::windows_core::IUnknown>, pnetsourcecreator: ::core::option::Option<&INSNetSourceCreator>, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetNetSourceCreator(this: &Self::This) -> ::windows_core::Result<INSNetSourceCreator>;
    fn SetCredentials(this: &Self::This, bstrrealm: &::windows_core::BSTR, bstrname: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetCredentials(this: &Self::This, bstrrealm: &::windows_core::BSTR, pbstrname: *mut ::windows_core::BSTR, pbstrpassword: *mut ::windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DeleteCredentials(this: &Self::This, bstrrealm: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetCredentialFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetCredentialFlags(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindProxyForURL(this: &Self::This, bstrprotocol: &::windows_core::BSTR, bstrhost: &::windows_core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows_core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::Result<()>;
    fn RegisterProxyFailure(this: &Self::This, hrparam: ::windows_core::HRESULT, dwproxycontext: u32) -> ::windows_core::Result<()>;
    fn ShutdownProxyContext(this: &Self::This, dwproxycontext: u32) -> ::windows_core::Result<()>;
    fn IsUsingIE(this: &Self::This, dwproxycontext: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMSInternalAdminNetSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSInternalAdminNetSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: *mut ::core::ffi::c_void, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&psharednamespace), ::windows_core::from_raw_borrowed(&pnamespacenode), ::windows_core::from_raw_borrowed(&pnetsourcecreator), ::core::mem::transmute_copy(&fembeddedinserver)).into())
        }
        unsafe extern "system" fn GetNetSourceCreator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetSourceCreator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetsourcecreator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into())
        }
        unsafe extern "system" fn GetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCredentials(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into())
        }
        unsafe extern "system" fn DeleteCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteCredentials(this, ::core::mem::transmute(&bstrrealm)).into())
        }
        unsafe extern "system" fn GetCredentialFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCredentialFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentialFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentialFlags(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn FindProxyForURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindProxyForURL(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrhost), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into())
        }
        unsafe extern "system" fn RegisterProxyFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrparam: ::windows_core::HRESULT, dwproxycontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterProxyFailure(this, ::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&dwproxycontext)).into())
        }
        unsafe extern "system" fn ShutdownProxyContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownProxyContext(this, ::core::mem::transmute_copy(&dwproxycontext)).into())
        }
        unsafe extern "system" fn IsUsingIE<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUsingIE(this, ::core::mem::transmute_copy(&dwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisusingie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMSInternalAdminNetSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetNetSourceCreator: GetNetSourceCreator::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            GetCredentials: GetCredentials::<Identity, Impl, OFFSET>,
            DeleteCredentials: DeleteCredentials::<Identity, Impl, OFFSET>,
            GetCredentialFlags: GetCredentialFlags::<Identity, Impl, OFFSET>,
            SetCredentialFlags: SetCredentialFlags::<Identity, Impl, OFFSET>,
            FindProxyForURL: FindProxyForURL::<Identity, Impl, OFFSET>,
            RegisterProxyFailure: RegisterProxyFailure::<Identity, Impl, OFFSET>,
            ShutdownProxyContext: ShutdownProxyContext::<Identity, Impl, OFFSET>,
            IsUsingIE: IsUsingIE::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource2_Impl: ::windows_core::BaseImpl {
    fn SetCredentialsEx(this: &Self::This, bstrrealm: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetCredentialsEx(this: &Self::This, bstrrealm: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows_core::BSTR, pbstrpassword: *mut ::windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DeleteCredentialsEx(this: &Self::This, bstrrealm: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, fproxy: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FindProxyForURLEx(this: &Self::This, bstrprotocol: &::windows_core::BSTR, bstrhost: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows_core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMSInternalAdminNetSource2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSInternalAdminNetSource2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCredentialsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentialsEx(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into())
        }
        unsafe extern "system" fn GetCredentialsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCredentialsEx(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into())
        }
        unsafe extern "system" fn DeleteCredentialsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteCredentialsEx(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy)).into())
        }
        unsafe extern "system" fn FindProxyForURLEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindProxyForURLEx(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrhost), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into())
        }
        IWMSInternalAdminNetSource2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCredentialsEx: SetCredentialsEx::<Identity, Impl, OFFSET>,
            GetCredentialsEx: GetCredentialsEx::<Identity, Impl, OFFSET>,
            DeleteCredentialsEx: DeleteCredentialsEx::<Identity, Impl, OFFSET>,
            FindProxyForURLEx: FindProxyForURLEx::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource3_Impl: ::windows_core::BaseImpl + IWMSInternalAdminNetSource2_Impl {
    fn GetNetSourceCreator2(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn FindProxyForURLEx2(this: &Self::This, bstrprotocol: &::windows_core::BSTR, bstrhost: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows_core::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows_core::Result<()>;
    fn RegisterProxyFailure2(this: &Self::This, hrparam: ::windows_core::HRESULT, qwproxycontext: u64) -> ::windows_core::Result<()>;
    fn ShutdownProxyContext2(this: &Self::This, qwproxycontext: u64) -> ::windows_core::Result<()>;
    fn IsUsingIE2(this: &Self::This, qwproxycontext: u64) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCredentialsEx2(this: &Self::This, bstrrealm: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetCredentialsEx2(this: &Self::This, bstrrealm: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows_core::BSTR, pbstrpassword: *mut ::windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMSInternalAdminNetSource3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMSInternalAdminNetSource2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSInternalAdminNetSource3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNetSourceCreator2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetSourceCreator2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetsourcecreator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindProxyForURLEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindProxyForURLEx2(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrhost), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pqwproxycontext)).into())
        }
        unsafe extern "system" fn RegisterProxyFailure2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrparam: ::windows_core::HRESULT, qwproxycontext: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterProxyFailure2(this, ::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&qwproxycontext)).into())
        }
        unsafe extern "system" fn ShutdownProxyContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownProxyContext2(this, ::core::mem::transmute_copy(&qwproxycontext)).into())
        }
        unsafe extern "system" fn IsUsingIE2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUsingIE2(this, ::core::mem::transmute_copy(&qwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisusingie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentialsEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentialsEx2(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood), ::core::mem::transmute_copy(&fcleartextauthentication)).into())
        }
        unsafe extern "system" fn GetCredentialsEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCredentialsEx2(this, ::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&fcleartextauthentication), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into())
        }
        IWMSInternalAdminNetSource3_Vtbl {
            base__: <IWMSInternalAdminNetSource2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNetSourceCreator2: GetNetSourceCreator2::<Identity, Impl, OFFSET>,
            FindProxyForURLEx2: FindProxyForURLEx2::<Identity, Impl, OFFSET>,
            RegisterProxyFailure2: RegisterProxyFailure2::<Identity, Impl, OFFSET>,
            ShutdownProxyContext2: ShutdownProxyContext2::<Identity, Impl, OFFSET>,
            IsUsingIE2: IsUsingIE2::<Identity, Impl, OFFSET>,
            SetCredentialsEx2: SetCredentialsEx2::<Identity, Impl, OFFSET>,
            GetCredentialsEx2: GetCredentialsEx2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSecureChannel_Impl: ::windows_core::BaseImpl + IWMAuthorizer_Impl {
    fn WMSC_AddCertificate(this: &Self::This, pcert: ::core::option::Option<&IWMAuthorizer>) -> ::windows_core::Result<()>;
    fn WMSC_AddSignature(this: &Self::This, pbcertsig: *const u8, cbcertsig: u32) -> ::windows_core::Result<()>;
    fn WMSC_Connect(this: &Self::This, potherside: ::core::option::Option<&IWMSecureChannel>) -> ::windows_core::Result<()>;
    fn WMSC_IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn WMSC_Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn WMSC_GetValidCertificate(this: &Self::This, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows_core::Result<()>;
    fn WMSC_Encrypt(this: &Self::This, pbdata: *const u8, cbdata: u32) -> ::windows_core::Result<()>;
    fn WMSC_Decrypt(this: &Self::This, pbdata: *const u8, cbdata: u32) -> ::windows_core::Result<()>;
    fn WMSC_Lock(this: &Self::This) -> ::windows_core::Result<()>;
    fn WMSC_Unlock(this: &Self::This) -> ::windows_core::Result<()>;
    fn WMSC_SetSharedData(this: &Self::This, dwcertindex: u32, pbshareddata: *const u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMSecureChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMAuthorizer);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSecureChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WMSC_AddCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcert: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_AddCertificate(this, ::windows_core::from_raw_borrowed(&pcert)).into())
        }
        unsafe extern "system" fn WMSC_AddSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_AddSignature(this, ::core::mem::transmute_copy(&pbcertsig), ::core::mem::transmute_copy(&cbcertsig)).into())
        }
        unsafe extern "system" fn WMSC_Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, potherside: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_Connect(this, ::windows_core::from_raw_borrowed(&potherside)).into())
        }
        unsafe extern "system" fn WMSC_IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WMSC_IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WMSC_Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_Disconnect(this).into())
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_GetValidCertificate(this, ::core::mem::transmute_copy(&ppbcertificate), ::core::mem::transmute_copy(&pdwsignature)).into())
        }
        unsafe extern "system" fn WMSC_Encrypt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_Encrypt(this, ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn WMSC_Decrypt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_Decrypt(this, ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn WMSC_Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_Lock(this).into())
        }
        unsafe extern "system" fn WMSC_Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_Unlock(this).into())
        }
        unsafe extern "system" fn WMSC_SetSharedData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMSC_SetSharedData(this, ::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata)).into())
        }
        IWMSecureChannel_Vtbl {
            base__: <IWMAuthorizer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WMSC_AddCertificate: WMSC_AddCertificate::<Identity, Impl, OFFSET>,
            WMSC_AddSignature: WMSC_AddSignature::<Identity, Impl, OFFSET>,
            WMSC_Connect: WMSC_Connect::<Identity, Impl, OFFSET>,
            WMSC_IsConnected: WMSC_IsConnected::<Identity, Impl, OFFSET>,
            WMSC_Disconnect: WMSC_Disconnect::<Identity, Impl, OFFSET>,
            WMSC_GetValidCertificate: WMSC_GetValidCertificate::<Identity, Impl, OFFSET>,
            WMSC_Encrypt: WMSC_Encrypt::<Identity, Impl, OFFSET>,
            WMSC_Decrypt: WMSC_Decrypt::<Identity, Impl, OFFSET>,
            WMSC_Lock: WMSC_Lock::<Identity, Impl, OFFSET>,
            WMSC_Unlock: WMSC_Unlock::<Identity, Impl, OFFSET>,
            WMSC_SetSharedData: WMSC_SetSharedData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMStatusCallback_Impl: ::windows_core::BaseImpl {
    fn OnStatus(this: &Self::This, status: WMT_STATUS, hr: ::windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMStatusCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStatusCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMStatusCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatus(this, ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMStatusCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnStatus: OnStatus::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMStreamConfig_Impl: ::windows_core::BaseImpl {
    fn GetStreamType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetStreamNumber(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetStreamNumber(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<()>;
    fn GetStreamName(this: &Self::This, pwszstreamname: ::windows_core::PWSTR, pcchstreamname: *mut u16) -> ::windows_core::Result<()>;
    fn SetStreamName(this: &Self::This, pwszstreamname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetConnectionName(this: &Self::This, pwszinputname: ::windows_core::PWSTR, pcchinputname: *mut u16) -> ::windows_core::Result<()>;
    fn SetConnectionName(this: &Self::This, pwszinputname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBitrate(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetBitrate(this: &Self::This, pdwbitrate: u32) -> ::windows_core::Result<()>;
    fn GetBufferWindow(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetBufferWindow(this: &Self::This, msbufferwindow: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMStreamConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMStreamConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStreamType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidstreamtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreamNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwstreamnum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStreamNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamNumber(this, ::core::mem::transmute_copy(&wstreamnum)).into())
        }
        unsafe extern "system" fn GetStreamName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows_core::PWSTR, pcchstreamname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamName(this, ::core::mem::transmute_copy(&pwszstreamname), ::core::mem::transmute_copy(&pcchstreamname)).into())
        }
        unsafe extern "system" fn SetStreamName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamName(this, ::core::mem::transmute(&pwszstreamname)).into())
        }
        unsafe extern "system" fn GetConnectionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinputname: ::windows_core::PWSTR, pcchinputname: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectionName(this, ::core::mem::transmute_copy(&pwszinputname), ::core::mem::transmute_copy(&pcchinputname)).into())
        }
        unsafe extern "system" fn SetConnectionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinputname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectionName(this, ::core::mem::transmute(&pwszinputname)).into())
        }
        unsafe extern "system" fn GetBitrate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitrate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwbitrate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBitrate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitrate(this, ::core::mem::transmute_copy(&pdwbitrate)).into())
        }
        unsafe extern "system" fn GetBufferWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBufferWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmsbufferwindow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferWindow(this, ::core::mem::transmute_copy(&msbufferwindow)).into())
        }
        IWMStreamConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStreamType: GetStreamType::<Identity, Impl, OFFSET>,
            GetStreamNumber: GetStreamNumber::<Identity, Impl, OFFSET>,
            SetStreamNumber: SetStreamNumber::<Identity, Impl, OFFSET>,
            GetStreamName: GetStreamName::<Identity, Impl, OFFSET>,
            SetStreamName: SetStreamName::<Identity, Impl, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
            SetConnectionName: SetConnectionName::<Identity, Impl, OFFSET>,
            GetBitrate: GetBitrate::<Identity, Impl, OFFSET>,
            SetBitrate: SetBitrate::<Identity, Impl, OFFSET>,
            GetBufferWindow: GetBufferWindow::<Identity, Impl, OFFSET>,
            SetBufferWindow: SetBufferWindow::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMStreamConfig2_Impl: ::windows_core::BaseImpl + IWMStreamConfig_Impl {
    fn GetTransportType(this: &Self::This) -> ::windows_core::Result<WMT_TRANSPORT_TYPE>;
    fn SetTransportType(this: &Self::This, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows_core::Result<()>;
    fn AddDataUnitExtension(this: &Self::This, guidextensionsystemid: &::windows_core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows_core::Result<()>;
    fn GetDataUnitExtensionCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetDataUnitExtension(this: &Self::This, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows_core::Result<()>;
    fn RemoveAllDataUnitExtensions(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMStreamConfig2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMStreamConfig);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMStreamConfig2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransportType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransportType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pntransporttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransportType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransportType(this, ::core::mem::transmute_copy(&ntransporttype)).into())
        }
        unsafe extern "system" fn AddDataUnitExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows_core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDataUnitExtension(this, ::core::mem::transmute(&guidextensionsystemid), ::core::mem::transmute_copy(&cbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&cbextensionsysteminfo)).into())
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataUnitExtensionCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcdataunitextensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataUnitExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataUnitExtension(this, ::core::mem::transmute_copy(&wdataunitextensionnumber), ::core::mem::transmute_copy(&pguidextensionsystemid), ::core::mem::transmute_copy(&pcbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&pcbextensionsysteminfo)).into())
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllDataUnitExtensions(this).into())
        }
        IWMStreamConfig2_Vtbl {
            base__: <IWMStreamConfig as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransportType: GetTransportType::<Identity, Impl, OFFSET>,
            SetTransportType: SetTransportType::<Identity, Impl, OFFSET>,
            AddDataUnitExtension: AddDataUnitExtension::<Identity, Impl, OFFSET>,
            GetDataUnitExtensionCount: GetDataUnitExtensionCount::<Identity, Impl, OFFSET>,
            GetDataUnitExtension: GetDataUnitExtension::<Identity, Impl, OFFSET>,
            RemoveAllDataUnitExtensions: RemoveAllDataUnitExtensions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMStreamConfig3_Impl: ::windows_core::BaseImpl + IWMStreamConfig2_Impl {
    fn GetLanguage(this: &Self::This, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()>;
    fn SetLanguage(this: &Self::This, pwszlanguagestring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMStreamConfig3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMStreamConfig2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMStreamConfig3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLanguage(this, ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into())
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamConfig3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguage(this, ::core::mem::transmute(&pwszlanguagestring)).into())
        }
        IWMStreamConfig3_Vtbl {
            base__: <IWMStreamConfig2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMStreamList_Impl: ::windows_core::BaseImpl {
    fn GetStreams(this: &Self::This, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()>;
    fn AddStream(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<()>;
    fn RemoveStream(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMStreamList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMStreamList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreams(this, ::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into())
        }
        unsafe extern "system" fn AddStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStream(this, ::core::mem::transmute_copy(&wstreamnum)).into())
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStream(this, ::core::mem::transmute_copy(&wstreamnum)).into())
        }
        IWMStreamList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStreams: GetStreams::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamPrioritization_Impl: ::windows_core::BaseImpl {
    fn GetPriorityRecords(this: &Self::This, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows_core::Result<()>;
    fn SetPriorityRecords(this: &Self::This, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMStreamPrioritization {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamPrioritization_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMStreamPrioritization {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPriorityRecords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamPrioritization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPriorityRecords(this, ::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&pcrecords)).into())
        }
        unsafe extern "system" fn SetPriorityRecords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMStreamPrioritization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriorityRecords(this, ::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&crecords)).into())
        }
        IWMStreamPrioritization_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPriorityRecords: GetPriorityRecords::<Identity, Impl, OFFSET>,
            SetPriorityRecords: SetPriorityRecords::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pwszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetRange(this: &Self::This, cnsstarttime: u64, cnsduration: i64) -> ::windows_core::Result<()>;
    fn SetRangeByFrame(this: &Self::This, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::Result<()>;
    fn GetNextSample(this: &Self::This, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows_core::Result<()>;
    fn SetStreamsSelected(this: &Self::This, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()>;
    fn GetStreamSelected(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION>;
    fn SetReadStreamSamples(this: &Self::This, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReadStreamSamples(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetOutputSetting(this: &Self::This, dwoutputnum: u32, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn SetOutputSetting(this: &Self::This, dwoutputnum: u32, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
    fn GetOutputCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOutputProps(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(this: &Self::This, dwoutputnum: u32, poutput: ::core::option::Option<&IWMOutputMediaProps>) -> ::windows_core::Result<()>;
    fn GetOutputFormatCount(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<u32>;
    fn GetOutputFormat(this: &Self::This, dwoutputnum: u32, dwformatnum: u32) -> ::windows_core::Result<IWMOutputMediaProps>;
    fn GetOutputNumberForStream(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<u32>;
    fn GetStreamNumberForOutput(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<u16>;
    fn GetMaxOutputSampleSize(this: &Self::This, dwoutput: u32) -> ::windows_core::Result<u32>;
    fn GetMaxStreamSampleSize(this: &Self::This, wstream: u16) -> ::windows_core::Result<u32>;
    fn OpenStream(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMSyncReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSyncReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pwszfilename)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn SetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRange(this, ::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration)).into())
        }
        unsafe extern "system" fn SetRangeByFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRangeByFrame(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)).into())
        }
        unsafe extern "system" fn GetNextSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut *mut ::core::ffi::c_void, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextSample(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&ppsample), ::core::mem::transmute_copy(&pcnssampletime), ::core::mem::transmute_copy(&pcnsduration), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwoutputnum), ::core::mem::transmute_copy(&pwstreamnum)).into())
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamsSelected(this, ::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into())
        }
        unsafe extern "system" fn GetStreamSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamSelected(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReadStreamSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReadStreamSamples(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fcompressed)).into())
        }
        unsafe extern "system" fn GetReadStreamSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReadStreamSamples(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcompressed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputSetting(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn SetOutputSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputSetting(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoutputs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputProps(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputProps(this, ::core::mem::transmute_copy(&dwoutputnum), ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputFormatCount(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputFormat(this, ::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&dwformatnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputNumberForStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputNumberForStream(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoutputnum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamNumberForOutput(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwstreamnum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxOutputSampleSize(this, ::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxStreamSampleSize(this, ::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenStream(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        IWMSyncReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
            SetRangeByFrame: SetRangeByFrame::<Identity, Impl, OFFSET>,
            GetNextSample: GetNextSample::<Identity, Impl, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, Impl, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, Impl, OFFSET>,
            SetReadStreamSamples: SetReadStreamSamples::<Identity, Impl, OFFSET>,
            GetReadStreamSamples: GetReadStreamSamples::<Identity, Impl, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, Impl, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, Impl, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, Impl, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, Impl, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, Impl, OFFSET>,
            GetOutputNumberForStream: GetOutputNumberForStream::<Identity, Impl, OFFSET>,
            GetStreamNumberForOutput: GetStreamNumberForOutput::<Identity, Impl, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, Impl, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader2_Impl: ::windows_core::BaseImpl + IWMSyncReader_Impl {
    fn SetRangeByTimecode(this: &Self::This, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows_core::Result<()>;
    fn SetRangeByFrameEx(this: &Self::This, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::Result<u64>;
    fn SetAllocateForOutput(this: &Self::This, dwoutputnum: u32, pallocator: ::core::option::Option<&IWMReaderAllocatorEx>) -> ::windows_core::Result<()>;
    fn GetAllocateForOutput(this: &Self::This, dwoutputnum: u32) -> ::windows_core::Result<IWMReaderAllocatorEx>;
    fn SetAllocateForStream(this: &Self::This, wstreamnum: u16, pallocator: ::core::option::Option<&IWMReaderAllocatorEx>) -> ::windows_core::Result<()>;
    fn GetAllocateForStream(this: &Self::This, dwsreamnum: u16) -> ::windows_core::Result<IWMReaderAllocatorEx>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMSyncReader2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMSyncReader);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMSyncReader2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRangeByTimecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRangeByTimecode(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pend)).into())
        }
        unsafe extern "system" fn SetRangeByFrameEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetRangeByFrameEx(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsstarttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocateForOutput(this, ::core::mem::transmute_copy(&dwoutputnum), ::windows_core::from_raw_borrowed(&pallocator)).into())
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocateForOutput(this, ::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppallocator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocateForStream(this, ::core::mem::transmute_copy(&wstreamnum), ::windows_core::from_raw_borrowed(&pallocator)).into())
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocateForStream(this, ::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppallocator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMSyncReader2_Vtbl {
            base__: <IWMSyncReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRangeByTimecode: SetRangeByTimecode::<Identity, Impl, OFFSET>,
            SetRangeByFrameEx: SetRangeByFrameEx::<Identity, Impl, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, Impl, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, Impl, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, Impl, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMVideoMediaProps_Impl: ::windows_core::BaseImpl + IWMMediaProps_Impl {
    fn GetMaxKeyFrameSpacing(this: &Self::This) -> ::windows_core::Result<i64>;
    fn SetMaxKeyFrameSpacing(this: &Self::This, lltime: i64) -> ::windows_core::Result<()>;
    fn GetQuality(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetQuality(this: &Self::This, dwquality: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMVideoMediaProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMMediaProps);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMVideoMediaProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxKeyFrameSpacing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plltime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxKeyFrameSpacing(this, ::core::mem::transmute_copy(&lltime)).into())
        }
        unsafe extern "system" fn GetQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQuality(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwquality, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuality(this, ::core::mem::transmute_copy(&dwquality)).into())
        }
        IWMVideoMediaProps_Vtbl {
            base__: <IWMMediaProps as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxKeyFrameSpacing: GetMaxKeyFrameSpacing::<Identity, Impl, OFFSET>,
            SetMaxKeyFrameSpacing: SetMaxKeyFrameSpacing::<Identity, Impl, OFFSET>,
            GetQuality: GetQuality::<Identity, Impl, OFFSET>,
            SetQuality: SetQuality::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMWatermarkInfo_Impl: ::windows_core::BaseImpl {
    fn GetWatermarkEntryCount(this: &Self::This, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows_core::Result<u32>;
    fn GetWatermarkEntry(this: &Self::This, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMWatermarkInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWatermarkInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWatermarkInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWatermarkEntryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWatermarkInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWatermarkEntryCount(this, ::core::mem::transmute_copy(&wmettype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWatermarkEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWatermarkInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWatermarkEntry(this, ::core::mem::transmute_copy(&wmettype), ::core::mem::transmute_copy(&dwentrynum), ::core::mem::transmute_copy(&pentry)).into())
        }
        IWMWatermarkInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWatermarkEntryCount: GetWatermarkEntryCount::<Identity, Impl, OFFSET>,
            GetWatermarkEntry: GetWatermarkEntry::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMWriter_Impl: ::windows_core::BaseImpl {
    fn SetProfileByID(this: &Self::This, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetProfile(this: &Self::This, pprofile: ::core::option::Option<&IWMProfile>) -> ::windows_core::Result<()>;
    fn SetOutputFilename(this: &Self::This, pwszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetInputCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetInputProps(this: &Self::This, dwinputnum: u32) -> ::windows_core::Result<IWMInputMediaProps>;
    fn SetInputProps(this: &Self::This, dwinputnum: u32, pinput: ::core::option::Option<&IWMInputMediaProps>) -> ::windows_core::Result<()>;
    fn GetInputFormatCount(this: &Self::This, dwinputnumber: u32) -> ::windows_core::Result<u32>;
    fn GetInputFormat(this: &Self::This, dwinputnumber: u32, dwformatnumber: u32) -> ::windows_core::Result<IWMInputMediaProps>;
    fn BeginWriting(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndWriting(this: &Self::This) -> ::windows_core::Result<()>;
    fn AllocateSample(this: &Self::This, dwsamplesize: u32) -> ::windows_core::Result<INSSBuffer>;
    fn WriteSample(this: &Self::This, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProfileByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProfileByID(this, ::core::mem::transmute_copy(&guidprofile)).into())
        }
        unsafe extern "system" fn SetProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProfile(this, ::windows_core::from_raw_borrowed(&pprofile)).into())
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputFilename(this, ::core::mem::transmute(&pwszfilename)).into())
        }
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcinputs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputProps(this, ::core::mem::transmute_copy(&dwinputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInputProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputProps(this, ::core::mem::transmute_copy(&dwinputnum), ::windows_core::from_raw_borrowed(&pinput)).into())
        }
        unsafe extern "system" fn GetInputFormatCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputFormatCount(this, ::core::mem::transmute_copy(&dwinputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputFormat(this, ::core::mem::transmute_copy(&dwinputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginWriting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginWriting(this).into())
        }
        unsafe extern "system" fn EndWriting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndWriting(this).into())
        }
        unsafe extern "system" fn AllocateSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocateSample(this, ::core::mem::transmute_copy(&dwsamplesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsample, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSample(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&psample)).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        IWMWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProfileByID: SetProfileByID::<Identity, Impl, OFFSET>,
            SetProfile: SetProfile::<Identity, Impl, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, Impl, OFFSET>,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetInputProps: GetInputProps::<Identity, Impl, OFFSET>,
            SetInputProps: SetInputProps::<Identity, Impl, OFFSET>,
            GetInputFormatCount: GetInputFormatCount::<Identity, Impl, OFFSET>,
            GetInputFormat: GetInputFormat::<Identity, Impl, OFFSET>,
            BeginWriting: BeginWriting::<Identity, Impl, OFFSET>,
            EndWriting: EndWriting::<Identity, Impl, OFFSET>,
            AllocateSample: AllocateSample::<Identity, Impl, OFFSET>,
            WriteSample: WriteSample::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced_Impl: ::windows_core::BaseImpl {
    fn GetSinkCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSink(this: &Self::This, dwsinknum: u32) -> ::windows_core::Result<IWMWriterSink>;
    fn AddSink(this: &Self::This, psink: ::core::option::Option<&IWMWriterSink>) -> ::windows_core::Result<()>;
    fn RemoveSink(this: &Self::This, psink: ::core::option::Option<&IWMWriterSink>) -> ::windows_core::Result<()>;
    fn WriteStreamSample(this: &Self::This, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>) -> ::windows_core::Result<()>;
    fn SetLiveSource(this: &Self::This, fislivesource: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsRealTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetWriterTime(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetStatistics(this: &Self::This, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows_core::Result<()>;
    fn SetSyncTolerance(this: &Self::This, mswindow: u32) -> ::windows_core::Result<()>;
    fn GetSyncTolerance(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterAdvanced {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterAdvanced {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSinkCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSinkCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsinks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSink(this, ::core::mem::transmute_copy(&dwsinknum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSink(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn RemoveSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSink(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn WriteStreamSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStreamSample(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&mssamplesendtime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&psample)).into())
        }
        unsafe extern "system" fn SetLiveSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLiveSource(this, ::core::mem::transmute_copy(&fislivesource)).into())
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRealTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrealtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWriterTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWriterTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnscurrenttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatistics(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn SetSyncTolerance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncTolerance(this, ::core::mem::transmute_copy(&mswindow)).into())
        }
        unsafe extern "system" fn GetSyncTolerance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncTolerance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmswindow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMWriterAdvanced_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSinkCount: GetSinkCount::<Identity, Impl, OFFSET>,
            GetSink: GetSink::<Identity, Impl, OFFSET>,
            AddSink: AddSink::<Identity, Impl, OFFSET>,
            RemoveSink: RemoveSink::<Identity, Impl, OFFSET>,
            WriteStreamSample: WriteStreamSample::<Identity, Impl, OFFSET>,
            SetLiveSource: SetLiveSource::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            GetWriterTime: GetWriterTime::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            SetSyncTolerance: SetSyncTolerance::<Identity, Impl, OFFSET>,
            GetSyncTolerance: GetSyncTolerance::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced2_Impl: ::windows_core::BaseImpl + IWMWriterAdvanced_Impl {
    fn GetInputSetting(this: &Self::This, dwinputnum: u32, pszname: &::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()>;
    fn SetInputSetting(this: &Self::This, dwinputnum: u32, pszname: &::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterAdvanced2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterAdvanced);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterAdvanced2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputSetting(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn SetInputSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputSetting(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into())
        }
        IWMWriterAdvanced2_Vtbl {
            base__: <IWMWriterAdvanced as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputSetting: GetInputSetting::<Identity, Impl, OFFSET>,
            SetInputSetting: SetInputSetting::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced3_Impl: ::windows_core::BaseImpl + IWMWriterAdvanced2_Impl {
    fn GetStatisticsEx(this: &Self::This, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows_core::Result<()>;
    fn SetNonBlocking(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterAdvanced3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterAdvanced2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterAdvanced3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatisticsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatisticsEx(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn SetNonBlocking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterAdvanced3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNonBlocking(this).into())
        }
        IWMWriterAdvanced3_Vtbl {
            base__: <IWMWriterAdvanced2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatisticsEx: GetStatisticsEx::<Identity, Impl, OFFSET>,
            SetNonBlocking: SetNonBlocking::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink_Impl: ::windows_core::BaseImpl + IWMWriterSink_Impl {
    fn Open(this: &Self::This, pwszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterFileSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterFileSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pwszfilename)).into())
        }
        IWMWriterFileSink_Vtbl { base__: <IWMWriterSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Open: Open::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink2_Impl: ::windows_core::BaseImpl + IWMWriterFileSink_Impl {
    fn Start(this: &Self::This, cnsstarttime: u64) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This, cnsstoptime: u64) -> ::windows_core::Result<()>;
    fn IsStopped(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFileDuration(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetFileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsClosed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterFileSink2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterFileSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterFileSink2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute_copy(&cnsstarttime)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this, ::core::mem::transmute_copy(&cnsstoptime)).into())
        }
        unsafe extern "system" fn IsStopped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsStopped(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfstopped, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn IsClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsClosed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfclosed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMWriterFileSink2_Vtbl {
            base__: <IWMWriterFileSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            IsStopped: IsStopped::<Identity, Impl, OFFSET>,
            GetFileDuration: GetFileDuration::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsClosed: IsClosed::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink3_Impl: ::windows_core::BaseImpl + IWMWriterFileSink2_Impl {
    fn SetAutoIndexing(this: &Self::This, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAutoIndexing(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetControlStream(this: &Self::This, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMode(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OnDataUnitEx(this: &Self::This, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows_core::Result<()>;
    fn SetUnbufferedIO(this: &Self::This, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetUnbufferedIO(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompleteOperations(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterFileSink3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterFileSink2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterFileSink3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAutoIndexing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoIndexing(this, ::core::mem::transmute_copy(&fdoautoindexing)).into())
        }
        unsafe extern "system" fn GetAutoIndexing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutoIndexing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfautoindexing, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetControlStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControlStream(this, ::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fshouldcontrolstartandstop)).into())
        }
        unsafe extern "system" fn GetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwfilesinkmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnDataUnitEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataUnitEx(this, ::core::mem::transmute_copy(&pfilesinkdataunit)).into())
        }
        unsafe extern "system" fn SetUnbufferedIO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnbufferedIO(this, ::core::mem::transmute_copy(&funbufferedio), ::core::mem::transmute_copy(&frestrictmemusage)).into())
        }
        unsafe extern "system" fn GetUnbufferedIO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnbufferedIO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfunbufferedio, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompleteOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompleteOperations(this).into())
        }
        IWMWriterFileSink3_Vtbl {
            base__: <IWMWriterFileSink2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAutoIndexing: SetAutoIndexing::<Identity, Impl, OFFSET>,
            GetAutoIndexing: GetAutoIndexing::<Identity, Impl, OFFSET>,
            SetControlStream: SetControlStream::<Identity, Impl, OFFSET>,
            GetMode: GetMode::<Identity, Impl, OFFSET>,
            OnDataUnitEx: OnDataUnitEx::<Identity, Impl, OFFSET>,
            SetUnbufferedIO: SetUnbufferedIO::<Identity, Impl, OFFSET>,
            GetUnbufferedIO: GetUnbufferedIO::<Identity, Impl, OFFSET>,
            CompleteOperations: CompleteOperations::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterNetworkSink_Impl: ::windows_core::BaseImpl + IWMWriterSink_Impl {
    fn SetMaximumClients(this: &Self::This, dwmaxclients: u32) -> ::windows_core::Result<()>;
    fn GetMaximumClients(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNetworkProtocol(this: &Self::This, protocol: WMT_NET_PROTOCOL) -> ::windows_core::Result<()>;
    fn GetNetworkProtocol(this: &Self::This) -> ::windows_core::Result<WMT_NET_PROTOCOL>;
    fn GetHostURL(this: &Self::This, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, pdwportnum: *mut u32) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterNetworkSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterNetworkSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMaximumClients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumClients(this, ::core::mem::transmute_copy(&dwmaxclients)).into())
        }
        unsafe extern "system" fn GetMaximumClients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumClients(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxclients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkProtocol(this, ::core::mem::transmute_copy(&protocol)).into())
        }
        unsafe extern "system" fn GetNetworkProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkProtocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprotocol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHostURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHostURL(this, ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute_copy(&pdwportnum)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IWMWriterNetworkSink_Vtbl {
            base__: <IWMWriterSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMaximumClients: SetMaximumClients::<Identity, Impl, OFFSET>,
            GetMaximumClients: GetMaximumClients::<Identity, Impl, OFFSET>,
            SetNetworkProtocol: SetNetworkProtocol::<Identity, Impl, OFFSET>,
            GetNetworkProtocol: GetNetworkProtocol::<Identity, Impl, OFFSET>,
            GetHostURL: GetHostURL::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPostView_Impl: ::windows_core::BaseImpl {
    fn SetPostViewCallback(this: &Self::This, pcallback: ::core::option::Option<&IWMWriterPostViewCallback>, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetReceivePostViewSamples(this: &Self::This, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReceivePostViewSamples(this: &Self::This, wstreamnum: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetPostViewProps(this: &Self::This, wstreamnumber: u16) -> ::windows_core::Result<IWMMediaProps>;
    fn SetPostViewProps(this: &Self::This, wstreamnumber: u16, poutput: ::core::option::Option<&IWMMediaProps>) -> ::windows_core::Result<()>;
    fn GetPostViewFormatCount(this: &Self::This, wstreamnumber: u16) -> ::windows_core::Result<u32>;
    fn GetPostViewFormat(this: &Self::This, wstreamnumber: u16, dwformatnumber: u32) -> ::windows_core::Result<IWMMediaProps>;
    fn SetAllocateForPostView(this: &Self::This, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAllocateForPostView(this: &Self::This, wstreamnumber: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterPostView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterPostView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPostViewCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostViewCallback(this, ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceivePostViewSamples(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivepostviewsamples)).into())
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReceivePostViewSamples(this, ::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfreceivepostviewsamples, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPostViewProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPostViewProps(this, ::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostViewProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostViewProps(this, ::core::mem::transmute_copy(&wstreamnumber), ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn GetPostViewFormatCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPostViewFormatCount(this, ::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPostViewFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPostViewFormat(this, ::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllocateForPostView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocateForPostView(this, ::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fallocate)).into())
        }
        unsafe extern "system" fn GetAllocateForPostView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocateForPostView(this, ::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallocate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMWriterPostView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPostViewCallback: SetPostViewCallback::<Identity, Impl, OFFSET>,
            SetReceivePostViewSamples: SetReceivePostViewSamples::<Identity, Impl, OFFSET>,
            GetReceivePostViewSamples: GetReceivePostViewSamples::<Identity, Impl, OFFSET>,
            GetPostViewProps: GetPostViewProps::<Identity, Impl, OFFSET>,
            SetPostViewProps: SetPostViewProps::<Identity, Impl, OFFSET>,
            GetPostViewFormatCount: GetPostViewFormatCount::<Identity, Impl, OFFSET>,
            GetPostViewFormat: GetPostViewFormat::<Identity, Impl, OFFSET>,
            SetAllocateForPostView: SetAllocateForPostView::<Identity, Impl, OFFSET>,
            GetAllocateForPostView: GetAllocateForPostView::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMWriterPostViewCallback_Impl: ::windows_core::BaseImpl + IWMStatusCallback_Impl {
    fn OnPostViewSample(this: &Self::This, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn AllocateForPostView(this: &Self::This, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMWriterPostViewCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMStatusCallback);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterPostViewCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPostViewSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPostViewSample(this, ::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&psample), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        unsafe extern "system" fn AllocateForPostView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateForPostView(this, ::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into())
        }
        IWMWriterPostViewCallback_Vtbl {
            base__: <IWMStatusCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPostViewSample: OnPostViewSample::<Identity, Impl, OFFSET>,
            AllocateForPostView: AllocateForPostView::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMWriterPreprocess_Impl: ::windows_core::BaseImpl {
    fn GetMaxPreprocessingPasses(this: &Self::This, dwinputnum: u32, dwflags: u32) -> ::windows_core::Result<u32>;
    fn SetNumPreprocessingPasses(this: &Self::This, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows_core::Result<()>;
    fn BeginPreprocessingPass(this: &Self::This, dwinputnum: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn PreprocessSample(this: &Self::This, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>) -> ::windows_core::Result<()>;
    fn EndPreprocessingPass(this: &Self::This, dwinputnum: u32, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMWriterPreprocess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterPreprocess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxPreprocessingPasses(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxnumpasses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumPreprocessingPasses(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumpasses)).into())
        }
        unsafe extern "system" fn BeginPreprocessingPass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPreprocessingPass(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn PreprocessSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreprocessSample(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&psample)).into())
        }
        unsafe extern "system" fn EndPreprocessingPass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndPreprocessingPass(this, ::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IWMWriterPreprocess_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxPreprocessingPasses: GetMaxPreprocessingPasses::<Identity, Impl, OFFSET>,
            SetNumPreprocessingPasses: SetNumPreprocessingPasses::<Identity, Impl, OFFSET>,
            BeginPreprocessingPass: BeginPreprocessingPass::<Identity, Impl, OFFSET>,
            PreprocessSample: PreprocessSample::<Identity, Impl, OFFSET>,
            EndPreprocessingPass: EndPreprocessingPass::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPushSink_Impl: ::windows_core::BaseImpl + IWMWriterSink_Impl {
    fn Connect(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pwsztemplateurl: &::windows_core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndSession(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterPushSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMWriterSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterPushSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pwsztemplateurl: ::windows_core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwsztemplateurl), ::core::mem::transmute_copy(&fautodestroy)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn EndSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSession(this).into())
        }
        IWMWriterPushSink_Vtbl {
            base__: <IWMWriterSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterSink_Impl: ::windows_core::BaseImpl {
    fn OnHeader(this: &Self::This, pheader: ::core::option::Option<&INSSBuffer>) -> ::windows_core::Result<()>;
    fn IsRealTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn AllocateDataUnit(this: &Self::This, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer>;
    fn OnDataUnit(this: &Self::This, pdataunit: ::core::option::Option<&INSSBuffer>) -> ::windows_core::Result<()>;
    fn OnEndWriting(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMWriterSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMWriterSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnHeader(this, ::windows_core::from_raw_borrowed(&pheader)).into())
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRealTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrealtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllocateDataUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocateDataUnit(this, ::core::mem::transmute_copy(&cbdataunit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataunit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnDataUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataunit: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataUnit(this, ::windows_core::from_raw_borrowed(&pdataunit)).into())
        }
        unsafe extern "system" fn OnEndWriting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndWriting(this).into())
        }
        IWMWriterSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnHeader: OnHeader::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            AllocateDataUnit: AllocateDataUnit::<Identity, Impl, OFFSET>,
            OnDataUnit: OnDataUnit::<Identity, Impl, OFFSET>,
            OnEndWriting: OnEndWriting::<Identity, Impl, OFFSET>,
        }
    };
}
