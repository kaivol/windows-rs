#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIAssociatedIdentityProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_AssociateIdentity(this: &Self::This, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Finish_AssociateIdentity(this: &Self::This) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_DisassociateIdentity(this: &Self::This, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_DisassociateIdentity(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_ChangeCredential(this: &Self::This, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_ChangeCredential(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for AsyncIAssociatedIdentityProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIAssociatedIdentityProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_AssociateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_AssociateIdentity(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_AssociateIdentity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_DisassociateIdentity(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into())
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_DisassociateIdentity(this).into())
        }
        unsafe extern "system" fn Begin_ChangeCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_ChangeCredential(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into())
        }
        unsafe extern "system" fn Finish_ChangeCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_ChangeCredential(this).into())
        }
        AsyncIAssociatedIdentityProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_AssociateIdentity: Begin_AssociateIdentity::<Identity, Impl, OFFSET>,
            Finish_AssociateIdentity: Finish_AssociateIdentity::<Identity, Impl, OFFSET>,
            Begin_DisassociateIdentity: Begin_DisassociateIdentity::<Identity, Impl, OFFSET>,
            Finish_DisassociateIdentity: Finish_DisassociateIdentity::<Identity, Impl, OFFSET>,
            Begin_ChangeCredential: Begin_ChangeCredential::<Identity, Impl, OFFSET>,
            Finish_ChangeCredential: Finish_ChangeCredential::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait AsyncIConnectedIdentityProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_ConnectIdentity(this: &Self::This, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::Result<()>;
    fn Finish_ConnectIdentity(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_DisconnectIdentity(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_DisconnectIdentity(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_IsConnected(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn Begin_GetUrl(this: &Self::This, identifier: IDENTITY_URL, context: ::core::option::Option<&super::super::super::super::System::Com::IBindCtx>) -> ::windows_core::Result<()>;
    fn Finish_GetUrl(this: &Self::This, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn Begin_GetAccountState(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_GetAccountState(this: &Self::This) -> ::windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for AsyncIConnectedIdentityProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIConnectedIdentityProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_ConnectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_ConnectIdentity(this, ::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into())
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_ConnectIdentity(this).into())
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_DisconnectIdentity(this).into())
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_DisconnectIdentity(this).into())
        }
        unsafe extern "system" fn Begin_IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_IsConnected(this).into())
        }
        unsafe extern "system" fn Finish_IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_GetUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetUrl(this, ::core::mem::transmute_copy(&identifier), ::windows_core::from_raw_borrowed(&context)).into())
        }
        unsafe extern "system" fn Finish_GetUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_GetUrl(this, ::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into())
        }
        unsafe extern "system" fn Begin_GetAccountState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetAccountState(this).into())
        }
        unsafe extern "system" fn Finish_GetAccountState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_GetAccountState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        AsyncIConnectedIdentityProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_ConnectIdentity: Begin_ConnectIdentity::<Identity, Impl, OFFSET>,
            Finish_ConnectIdentity: Finish_ConnectIdentity::<Identity, Impl, OFFSET>,
            Begin_DisconnectIdentity: Begin_DisconnectIdentity::<Identity, Impl, OFFSET>,
            Finish_DisconnectIdentity: Finish_DisconnectIdentity::<Identity, Impl, OFFSET>,
            Begin_IsConnected: Begin_IsConnected::<Identity, Impl, OFFSET>,
            Finish_IsConnected: Finish_IsConnected::<Identity, Impl, OFFSET>,
            Begin_GetUrl: Begin_GetUrl::<Identity, Impl, OFFSET>,
            Finish_GetUrl: Finish_GetUrl::<Identity, Impl, OFFSET>,
            Begin_GetAccountState: Begin_GetAccountState::<Identity, Impl, OFFSET>,
            Finish_GetAccountState: Finish_GetAccountState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIIdentityAdvise_Impl: ::windows_core::BaseImpl {
    fn Begin_IdentityUpdated(this: &Self::This, dwidentityupdateevents: u32, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_IdentityUpdated(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIIdentityAdvise {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIIdentityAdvise {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_IdentityUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_IdentityUpdated(this, ::core::mem::transmute_copy(&dwidentityupdateevents), ::core::mem::transmute(&lpszuniqueid)).into())
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_IdentityUpdated(this).into())
        }
        AsyncIIdentityAdvise_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_IdentityUpdated: Begin_IdentityUpdated::<Identity, Impl, OFFSET>,
            Finish_IdentityUpdated: Finish_IdentityUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIIdentityAuthentication_Impl: ::windows_core::BaseImpl {
    fn Begin_SetIdentityCredential(this: &Self::This, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::Result<()>;
    fn Finish_SetIdentityCredential(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_ValidateIdentityCredential(this: &Self::This, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Finish_ValidateIdentityCredential(this: &Self::This, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for AsyncIIdentityAuthentication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIIdentityAuthentication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_SetIdentityCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_SetIdentityCredential(this, ::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into())
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_SetIdentityCredential(this).into())
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_ValidateIdentityCredential(this, ::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into())
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_ValidateIdentityCredential(this, ::core::mem::transmute_copy(&ppidentityproperties)).into())
        }
        AsyncIIdentityAuthentication_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_SetIdentityCredential: Begin_SetIdentityCredential::<Identity, Impl, OFFSET>,
            Finish_SetIdentityCredential: Finish_SetIdentityCredential::<Identity, Impl, OFFSET>,
            Begin_ValidateIdentityCredential: Begin_ValidateIdentityCredential::<Identity, Impl, OFFSET>,
            Finish_ValidateIdentityCredential: Finish_ValidateIdentityCredential::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_GetIdentityEnum(this: &Self::This, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_GetIdentityEnum(this: &Self::This) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Create(this: &Self::This, lpszusername: &::windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_Create(this: &Self::This) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Import(this: &Self::This, ppropertystore: ::core::option::Option<&super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Finish_Import(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_Delete(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_FindByUniqueID(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_FindByUniqueID(this: &Self::This) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_GetProviderPropertyStore(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_GetProviderPropertyStore(this: &Self::This) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Advise(this: &Self::This, pidentityadvise: ::core::option::Option<&IIdentityAdvise>, dwidentityupdateevents: u32) -> ::windows_core::Result<()>;
    fn Finish_Advise(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Begin_UnAdvise(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn Finish_UnAdvise(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for AsyncIIdentityProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIIdentityProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_GetIdentityEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetIdentityEnum(this, ::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into())
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_GetIdentityEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszusername: ::windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Create(this, ::core::mem::transmute(&lpszusername), ::core::mem::transmute_copy(&pkeywordstoadd)).into())
        }
        unsafe extern "system" fn Finish_Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_Create(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Import(this, ::windows_core::from_raw_borrowed(&ppropertystore)).into())
        }
        unsafe extern "system" fn Finish_Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Import(this).into())
        }
        unsafe extern "system" fn Begin_Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Delete(this, ::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into())
        }
        unsafe extern "system" fn Finish_Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Delete(this).into())
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_FindByUniqueID(this, ::core::mem::transmute(&lpszuniqueid)).into())
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_FindByUniqueID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetProviderPropertyStore(this).into())
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_GetProviderPropertyStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Advise(this, ::windows_core::from_raw_borrowed(&pidentityadvise), ::core::mem::transmute_copy(&dwidentityupdateevents)).into())
        }
        unsafe extern "system" fn Finish_Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_Advise(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_UnAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_UnAdvise(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn Finish_UnAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_UnAdvise(this).into())
        }
        AsyncIIdentityProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_GetIdentityEnum: Begin_GetIdentityEnum::<Identity, Impl, OFFSET>,
            Finish_GetIdentityEnum: Finish_GetIdentityEnum::<Identity, Impl, OFFSET>,
            Begin_Create: Begin_Create::<Identity, Impl, OFFSET>,
            Finish_Create: Finish_Create::<Identity, Impl, OFFSET>,
            Begin_Import: Begin_Import::<Identity, Impl, OFFSET>,
            Finish_Import: Finish_Import::<Identity, Impl, OFFSET>,
            Begin_Delete: Begin_Delete::<Identity, Impl, OFFSET>,
            Finish_Delete: Finish_Delete::<Identity, Impl, OFFSET>,
            Begin_FindByUniqueID: Begin_FindByUniqueID::<Identity, Impl, OFFSET>,
            Finish_FindByUniqueID: Finish_FindByUniqueID::<Identity, Impl, OFFSET>,
            Begin_GetProviderPropertyStore: Begin_GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Finish_GetProviderPropertyStore: Finish_GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Begin_Advise: Begin_Advise::<Identity, Impl, OFFSET>,
            Finish_Advise: Finish_Advise::<Identity, Impl, OFFSET>,
            Begin_UnAdvise: Begin_UnAdvise::<Identity, Impl, OFFSET>,
            Finish_UnAdvise: Finish_UnAdvise::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityStore_Impl: ::windows_core::BaseImpl {
    fn Begin_GetCount(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Begin_GetAt(this: &Self::This, dwprovider: u32, pprovguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_GetAt(this: &Self::This, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Begin_AddToCache(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_AddToCache(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_ConvertToSid(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8) -> ::windows_core::Result<()>;
    fn Finish_ConvertToSid(this: &Self::This, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::Result<()>;
    fn Begin_EnumerateIdentities(this: &Self::This, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_EnumerateIdentities(this: &Self::This) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for AsyncIIdentityStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIIdentityStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetCount(this).into())
        }
        unsafe extern "system" fn Finish_GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetAt(this, ::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid)).into())
        }
        unsafe extern "system" fn Finish_GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_GetAt(this, ::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into())
        }
        unsafe extern "system" fn Begin_AddToCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_AddToCache(this, ::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into())
        }
        unsafe extern "system" fn Finish_AddToCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_AddToCache(this).into())
        }
        unsafe extern "system" fn Begin_ConvertToSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_ConvertToSid(this, ::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid)).into())
        }
        unsafe extern "system" fn Finish_ConvertToSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_ConvertToSid(this, ::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into())
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_EnumerateIdentities(this, ::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into())
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_EnumerateIdentities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Begin_Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Reset(this).into())
        }
        unsafe extern "system" fn Finish_Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Reset(this).into())
        }
        AsyncIIdentityStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_GetCount: Begin_GetCount::<Identity, Impl, OFFSET>,
            Finish_GetCount: Finish_GetCount::<Identity, Impl, OFFSET>,
            Begin_GetAt: Begin_GetAt::<Identity, Impl, OFFSET>,
            Finish_GetAt: Finish_GetAt::<Identity, Impl, OFFSET>,
            Begin_AddToCache: Begin_AddToCache::<Identity, Impl, OFFSET>,
            Finish_AddToCache: Finish_AddToCache::<Identity, Impl, OFFSET>,
            Begin_ConvertToSid: Begin_ConvertToSid::<Identity, Impl, OFFSET>,
            Finish_ConvertToSid: Finish_ConvertToSid::<Identity, Impl, OFFSET>,
            Begin_EnumerateIdentities: Begin_EnumerateIdentities::<Identity, Impl, OFFSET>,
            Finish_EnumerateIdentities: Finish_EnumerateIdentities::<Identity, Impl, OFFSET>,
            Begin_Reset: Begin_Reset::<Identity, Impl, OFFSET>,
            Finish_Reset: Finish_Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIIdentityStoreEx_Impl: ::windows_core::BaseImpl {
    fn Begin_CreateConnectedIdentity(this: &Self::This, localname: &::windows_core::PCWSTR, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_CreateConnectedIdentity(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_DeleteConnectedIdentity(this: &Self::This, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_DeleteConnectedIdentity(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIIdentityStoreEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIIdentityStoreEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localname: ::windows_core::PCWSTR, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_CreateConnectedIdentity(this, ::core::mem::transmute(&localname), ::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into())
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_CreateConnectedIdentity(this).into())
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_DeleteConnectedIdentity(this, ::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into())
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_DeleteConnectedIdentity(this).into())
        }
        AsyncIIdentityStoreEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_CreateConnectedIdentity: Begin_CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            Finish_CreateConnectedIdentity: Finish_CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            Begin_DeleteConnectedIdentity: Begin_DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
            Finish_DeleteConnectedIdentity: Finish_DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAssociatedIdentityProvider_Impl: ::windows_core::BaseImpl {
    fn AssociateIdentity(this: &Self::This, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn DisassociateIdentity(this: &Self::This, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ChangeCredential(this: &Self::This, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IAssociatedIdentityProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAssociatedIdentityProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssociateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssociateIdentity(this, ::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisassociateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisassociateIdentity(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into())
        }
        unsafe extern "system" fn ChangeCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeCredential(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into())
        }
        IAssociatedIdentityProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssociateIdentity: AssociateIdentity::<Identity, Impl, OFFSET>,
            DisassociateIdentity: DisassociateIdentity::<Identity, Impl, OFFSET>,
            ChangeCredential: ChangeCredential::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IConnectedIdentityProvider_Impl: ::windows_core::BaseImpl {
    fn ConnectIdentity(this: &Self::This, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::Result<()>;
    fn DisconnectIdentity(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn GetUrl(this: &Self::This, identifier: IDENTITY_URL, context: ::core::option::Option<&super::super::super::super::System::Com::IBindCtx>, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetAccountState(this: &Self::This) -> ::windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IConnectedIdentityProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConnectedIdentityProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectIdentity(this, ::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into())
        }
        unsafe extern "system" fn DisconnectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectIdentity(this).into())
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUrl(this, ::core::mem::transmute_copy(&identifier), ::windows_core::from_raw_borrowed(&context), ::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into())
        }
        unsafe extern "system" fn GetAccountState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccountState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConnectedIdentityProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectIdentity: ConnectIdentity::<Identity, Impl, OFFSET>,
            DisconnectIdentity: DisconnectIdentity::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetUrl: GetUrl::<Identity, Impl, OFFSET>,
            GetAccountState: GetAccountState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IIdentityAdvise_Impl: ::windows_core::BaseImpl {
    fn IdentityUpdated(this: &Self::This, dwidentityupdateevents: &IdentityUpdateEvent, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IIdentityAdvise {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityAdvise_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdentityAdvise {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IdentityUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityAdvise_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IdentityUpdated(this, ::core::mem::transmute(&dwidentityupdateevents), ::core::mem::transmute(&lpszuniqueid)).into())
        }
        IIdentityAdvise_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IdentityUpdated: IdentityUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthentication_Impl: ::windows_core::BaseImpl {
    fn SetIdentityCredential(this: &Self::This, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::Result<()>;
    fn ValidateIdentityCredential(this: &Self::This, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IIdentityAuthentication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityAuthentication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdentityAuthentication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIdentityCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityAuthentication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIdentityCredential(this, ::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into())
        }
        unsafe extern "system" fn ValidateIdentityCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityAuthentication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateIdentityCredential(this, ::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into())
        }
        IIdentityAuthentication_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIdentityCredential: SetIdentityCredential::<Identity, Impl, OFFSET>,
            ValidateIdentityCredential: ValidateIdentityCredential::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityProvider_Impl: ::windows_core::BaseImpl {
    fn GetIdentityEnum(this: &Self::This, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Create(this: &Self::This, lpszusername: &::windows_core::PCWSTR, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Import(this: &Self::This, ppropertystore: ::core::option::Option<&super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn FindByUniqueID(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetProviderPropertyStore(this: &Self::This) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Advise(this: &Self::This, pidentityadvise: ::core::option::Option<&IIdentityAdvise>, dwidentityupdateevents: &IdentityUpdateEvent) -> ::windows_core::Result<u32>;
    fn UnAdvise(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IIdentityProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdentityProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdentityEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIdentityEnum(this, ::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszusername: ::windows_core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute(&lpszusername), ::core::mem::transmute_copy(&pppropertystore), ::core::mem::transmute_copy(&pkeywordstoadd)).into())
        }
        unsafe extern "system" fn Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Import(this, ::windows_core::from_raw_borrowed(&ppropertystore)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into())
        }
        unsafe extern "system" fn FindByUniqueID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindByUniqueID(this, ::core::mem::transmute(&lpszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderPropertyStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&pidentityadvise), ::core::mem::transmute(&dwidentityupdateevents)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnAdvise(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        IIdentityProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdentityEnum: GetIdentityEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            FindByUniqueID: FindByUniqueID::<Identity, Impl, OFFSET>,
            GetProviderPropertyStore: GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            UnAdvise: UnAdvise::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityStore_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, dwprovider: u32, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn AddToCache(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ConvertToSid(this: &Self::This, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::Result<()>;
    fn EnumerateIdentities(this: &Self::This, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IIdentityStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdentityStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into())
        }
        unsafe extern "system" fn AddToCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToCache(this, ::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into())
        }
        unsafe extern "system" fn ConvertToSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertToSid(this, ::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into())
        }
        unsafe extern "system" fn EnumerateIdentities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateIdentities(this, ::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IIdentityStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            AddToCache: AddToCache::<Identity, Impl, OFFSET>,
            ConvertToSid: ConvertToSid::<Identity, Impl, OFFSET>,
            EnumerateIdentities: EnumerateIdentities::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IIdentityStoreEx_Impl: ::windows_core::BaseImpl {
    fn CreateConnectedIdentity(this: &Self::This, localname: &::windows_core::PCWSTR, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DeleteConnectedIdentity(this: &Self::This, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IIdentityStoreEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStoreEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdentityStoreEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateConnectedIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localname: ::windows_core::PCWSTR, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateConnectedIdentity(this, ::core::mem::transmute(&localname), ::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into())
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdentityStoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteConnectedIdentity(this, ::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into())
        }
        IIdentityStoreEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateConnectedIdentity: CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            DeleteConnectedIdentity: DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
