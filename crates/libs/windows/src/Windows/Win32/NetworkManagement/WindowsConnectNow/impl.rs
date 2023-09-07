pub trait IWCNConnectNotify_Impl: ::windows_core::BaseImpl {
    fn ConnectSucceeded(this: &Self::This) -> ::windows_core::Result<()>;
    fn ConnectFailed(this: &Self::This, hrfailure: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWCNConnectNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWCNConnectNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectSucceeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectSucceeded(this).into())
        }
        unsafe extern "system" fn ConnectFailed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrfailure: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectFailed(this, ::core::mem::transmute_copy(&hrfailure)).into())
        }
        IWCNConnectNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectSucceeded: ConnectSucceeded::<Identity, Impl, OFFSET>,
            ConnectFailed: ConnectFailed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWCNDevice_Impl: ::windows_core::BaseImpl {
    fn SetPassword(this: &Self::This, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows_core::Result<()>;
    fn Connect(this: &Self::This, pnotify: ::core::option::Option<&IWCNConnectNotify>) -> ::windows_core::Result<()>;
    fn GetAttribute(this: &Self::This, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows_core::Result<()>;
    fn GetIntegerAttribute(this: &Self::This, attributetype: WCN_ATTRIBUTE_TYPE) -> ::windows_core::Result<u32>;
    fn GetStringAttribute(this: &Self::This, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetNetworkProfile(this: &Self::This, cchmaxstringlength: u32, wszprofile: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetNetworkProfile(this: &Self::This, pszprofilexml: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetVendorExtension(this: &Self::This, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows_core::Result<()>;
    fn SetVendorExtension(this: &Self::This, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows_core::Result<()>;
    fn Unadvise(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetNFCPasswordParams(this: &Self::This, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWCNDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWCNDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword)).into())
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::windows_core::from_raw_borrowed(&pnotify)).into())
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttribute(this, ::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into())
        }
        unsafe extern "system" fn GetIntegerAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIntegerAttribute(this, ::core::mem::transmute_copy(&attributetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinteger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringAttribute(this, ::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&cchmaxstring), ::core::mem::transmute_copy(&wszstring)).into())
        }
        unsafe extern "system" fn GetNetworkProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchmaxstringlength: u32, wszprofile: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNetworkProfile(this, ::core::mem::transmute_copy(&cchmaxstringlength), ::core::mem::transmute_copy(&wszprofile)).into())
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszprofilexml: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkProfile(this, ::core::mem::transmute(&pszprofilexml)).into())
        }
        unsafe extern "system" fn GetVendorExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVendorExtension(this, ::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into())
        }
        unsafe extern "system" fn SetVendorExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVendorExtension(this, ::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer)).into())
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this).into())
        }
        unsafe extern "system" fn SetNFCPasswordParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNFCPasswordParams(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwoobpasswordid), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword), ::core::mem::transmute_copy(&dwremotepublickeyhashlength), ::core::mem::transmute_copy(&pbremotepublickeyhash), ::core::mem::transmute_copy(&dwdhkeybloblength), ::core::mem::transmute_copy(&pbdhkeyblob)).into())
        }
        IWCNDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetIntegerAttribute: GetIntegerAttribute::<Identity, Impl, OFFSET>,
            GetStringAttribute: GetStringAttribute::<Identity, Impl, OFFSET>,
            GetNetworkProfile: GetNetworkProfile::<Identity, Impl, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, Impl, OFFSET>,
            GetVendorExtension: GetVendorExtension::<Identity, Impl, OFFSET>,
            SetVendorExtension: SetVendorExtension::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            SetNFCPasswordParams: SetNFCPasswordParams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
