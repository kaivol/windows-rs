#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IItemEnumerator_Impl: ::windows_core::BaseImpl {
    fn Current(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IItemEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IItemEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Current<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Current(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemvalid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IItemEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Current: Current::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISettingsContext_Impl: ::windows_core::BaseImpl {
    fn Serialize(this: &Self::This, pstream: ::core::option::Option<&super::Com::IStream>, ptarget: ::core::option::Option<&ITargetInfo>) -> ::windows_core::Result<()>;
    fn Deserialize(this: &Self::This, pstream: ::core::option::Option<&super::Com::IStream>, ptarget: ::core::option::Option<&ITargetInfo>, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows_core::Result<()>;
    fn SetUserData(this: &Self::This, puserdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetUserData(this: &Self::This) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
    fn GetNamespaces(this: &Self::This) -> ::windows_core::Result<IItemEnumerator>;
    fn GetStoredSettings(this: &Self::This, pidentity: ::core::option::Option<&ISettingsIdentity>, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows_core::Result<()>;
    fn RevertSetting(this: &Self::This, pidentity: ::core::option::Option<&ISettingsIdentity>, pwzsetting: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISettingsContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISettingsContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&ptarget)).into())
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deserialize(this, ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&ptarget), ::core::mem::transmute_copy(&pppresults), ::core::mem::transmute_copy(&pcresultcount)).into())
        }
        unsafe extern "system" fn SetUserData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserData(this, ::core::mem::transmute_copy(&puserdata)).into())
        }
        unsafe extern "system" fn GetUserData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puserdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamespaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespaceids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStoredSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, ppaddedsettings: *mut *mut ::core::ffi::c_void, ppmodifiedsettings: *mut *mut ::core::ffi::c_void, ppdeletedsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStoredSettings(this, ::windows_core::from_raw_borrowed(&pidentity), ::core::mem::transmute_copy(&ppaddedsettings), ::core::mem::transmute_copy(&ppmodifiedsettings), ::core::mem::transmute_copy(&ppdeletedsettings)).into())
        }
        unsafe extern "system" fn RevertSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, pwzsetting: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevertSetting(this, ::windows_core::from_raw_borrowed(&pidentity), ::core::mem::transmute(&pwzsetting)).into())
        }
        ISettingsContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
            SetUserData: SetUserData::<Identity, Impl, OFFSET>,
            GetUserData: GetUserData::<Identity, Impl, OFFSET>,
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetStoredSettings: GetStoredSettings::<Identity, Impl, OFFSET>,
            RevertSetting: RevertSetting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISettingsEngine_Impl: ::windows_core::BaseImpl {
    fn GetNamespaces(this: &Self::This, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<IItemEnumerator>;
    fn GetNamespace(this: &Self::This, settingsid: ::core::option::Option<&ISettingsIdentity>, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<ISettingsNamespace>;
    fn GetErrorDescription(this: &Self::This, hresult: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateSettingsIdentity(this: &Self::This) -> ::windows_core::Result<ISettingsIdentity>;
    fn GetStoreStatus(this: &Self::This, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<WcmUserStatus>;
    fn LoadStore(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
    fn UnloadStore(this: &Self::This, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RegisterNamespace(this: &Self::This, settingsid: ::core::option::Option<&ISettingsIdentity>, stream: ::core::option::Option<&super::Com::IStream>, pushsettings: super::super::Foundation::BOOL) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn UnregisterNamespace(this: &Self::This, settingsid: ::core::option::Option<&ISettingsIdentity>, removesettings: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CreateTargetInfo(this: &Self::This) -> ::windows_core::Result<ITargetInfo>;
    fn GetTargetInfo(this: &Self::This) -> ::windows_core::Result<ITargetInfo>;
    fn SetTargetInfo(this: &Self::This, target: ::core::option::Option<&ITargetInfo>) -> ::windows_core::Result<()>;
    fn CreateSettingsContext(this: &Self::This, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<ISettingsContext>;
    fn SetSettingsContext(this: &Self::This, settingscontext: ::core::option::Option<&ISettingsContext>) -> ::windows_core::Result<()>;
    fn ApplySettingsContext(this: &Self::This, settingscontext: ::core::option::Option<&ISettingsContext>, pppwzidentities: *mut *mut ::windows_core::PWSTR, pcidentities: *mut usize) -> ::windows_core::Result<()>;
    fn GetSettingsContext(this: &Self::This) -> ::windows_core::Result<ISettingsContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISettingsEngine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISettingsEngine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamespaces(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamespace(this, ::windows_core::from_raw_borrowed(&settingsid), ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorDescription(this, ::core::mem::transmute_copy(&hresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSettingsIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSettingsIdentity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStoreStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStoreStatus(this, ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadStore(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn UnloadStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnloadStore(this, ::core::mem::transmute_copy(&reserved)).into())
        }
        unsafe extern "system" fn RegisterNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterNamespace(this, ::windows_core::from_raw_borrowed(&settingsid), ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&pushsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterNamespace(this, ::windows_core::from_raw_borrowed(&settingsid), ::core::mem::transmute_copy(&removesettings)).into())
        }
        unsafe extern "system" fn CreateTargetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTargetInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(target, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(target, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetInfo(this, ::windows_core::from_raw_borrowed(&target)).into())
        }
        unsafe extern "system" fn CreateSettingsContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSettingsContext(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingscontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSettingsContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSettingsContext(this, ::windows_core::from_raw_borrowed(&settingscontext)).into())
        }
        unsafe extern "system" fn ApplySettingsContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void, pppwzidentities: *mut *mut ::windows_core::PWSTR, pcidentities: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplySettingsContext(this, ::windows_core::from_raw_borrowed(&settingscontext), ::core::mem::transmute_copy(&pppwzidentities), ::core::mem::transmute_copy(&pcidentities)).into())
        }
        unsafe extern "system" fn GetSettingsContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettingsContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingscontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISettingsEngine_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            CreateSettingsIdentity: CreateSettingsIdentity::<Identity, Impl, OFFSET>,
            GetStoreStatus: GetStoreStatus::<Identity, Impl, OFFSET>,
            LoadStore: LoadStore::<Identity, Impl, OFFSET>,
            UnloadStore: UnloadStore::<Identity, Impl, OFFSET>,
            RegisterNamespace: RegisterNamespace::<Identity, Impl, OFFSET>,
            UnregisterNamespace: UnregisterNamespace::<Identity, Impl, OFFSET>,
            CreateTargetInfo: CreateTargetInfo::<Identity, Impl, OFFSET>,
            GetTargetInfo: GetTargetInfo::<Identity, Impl, OFFSET>,
            SetTargetInfo: SetTargetInfo::<Identity, Impl, OFFSET>,
            CreateSettingsContext: CreateSettingsContext::<Identity, Impl, OFFSET>,
            SetSettingsContext: SetSettingsContext::<Identity, Impl, OFFSET>,
            ApplySettingsContext: ApplySettingsContext::<Identity, Impl, OFFSET>,
            GetSettingsContext: GetSettingsContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISettingsIdentity_Impl: ::windows_core::BaseImpl {
    fn GetAttribute(this: &Self::This, reserved: *const ::core::ffi::c_void, name: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAttribute(this: &Self::This, reserved: *const ::core::ffi::c_void, name: &::windows_core::PCWSTR, value: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFlags(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISettingsIdentity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISettingsIdentity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttribute(this, ::core::mem::transmute_copy(&reserved), ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttribute(this, ::core::mem::transmute_copy(&reserved), ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        ISettingsIdentity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            SetAttribute: SetAttribute::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISettingsItem_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetValue(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, value: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetSettingType(this: &Self::This) -> ::windows_core::Result<WcmSettingType>;
    fn GetDataType(this: &Self::This) -> ::windows_core::Result<WcmDataType>;
    fn GetValueRaw(this: &Self::This, data: *mut *mut u8, datasize: *mut u32) -> ::windows_core::Result<()>;
    fn SetValueRaw(this: &Self::This, datatype: i32, data: *const u8, datasize: u32) -> ::windows_core::Result<()>;
    fn HasChild(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Children(this: &Self::This) -> ::windows_core::Result<IItemEnumerator>;
    fn GetChild(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<ISettingsItem>;
    fn GetSettingByPath(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<ISettingsItem>;
    fn CreateSettingByPath(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<ISettingsItem>;
    fn RemoveSettingByPath(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetListKeyInformation(this: &Self::This, keyname: *mut ::windows_core::BSTR, datatype: *mut WcmDataType) -> ::windows_core::Result<()>;
    fn CreateListElement(this: &Self::This, keydata: *const super::Variant::VARIANT) -> ::windows_core::Result<ISettingsItem>;
    fn RemoveListElement(this: &Self::This, elementname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Attributes(this: &Self::This) -> ::windows_core::Result<IItemEnumerator>;
    fn GetAttribute(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRestrictionFacets(this: &Self::This) -> ::windows_core::Result<WcmRestrictionFacets>;
    fn GetRestriction(this: &Self::This, restrictionfacet: WcmRestrictionFacets) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetKeyValue(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISettingsItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISettingsItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetSettingType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettingType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValueRaw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValueRaw(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn SetValueRaw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueRaw(this, ::core::mem::transmute_copy(&datatype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn HasChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasChild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemhaschild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, child: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChild(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(child, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSettingByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettingByPath(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSettingByPath(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSettingByPath(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn GetListKeyInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, datatype: *mut WcmDataType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetListKeyInformation(this, ::core::mem::transmute_copy(&keyname), ::core::mem::transmute_copy(&datatype)).into())
        }
        unsafe extern "system" fn CreateListElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keydata: *const super::Variant::VARIANT, child: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateListElement(this, ::core::mem::transmute_copy(&keydata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(child, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveListElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elementname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveListElement(this, ::core::mem::transmute(&elementname)).into())
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttribute(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRestrictionFacets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictionFacets(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictionfacets, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRestriction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestriction(this, ::core::mem::transmute_copy(&restrictionfacet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(facetdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKeyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeyValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISettingsItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetSettingType: GetSettingType::<Identity, Impl, OFFSET>,
            GetDataType: GetDataType::<Identity, Impl, OFFSET>,
            GetValueRaw: GetValueRaw::<Identity, Impl, OFFSET>,
            SetValueRaw: SetValueRaw::<Identity, Impl, OFFSET>,
            HasChild: HasChild::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, Impl, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, Impl, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, Impl, OFFSET>,
            GetListKeyInformation: GetListKeyInformation::<Identity, Impl, OFFSET>,
            CreateListElement: CreateListElement::<Identity, Impl, OFFSET>,
            RemoveListElement: RemoveListElement::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetRestrictionFacets: GetRestrictionFacets::<Identity, Impl, OFFSET>,
            GetRestriction: GetRestriction::<Identity, Impl, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISettingsNamespace_Impl: ::windows_core::BaseImpl {
    fn GetIdentity(this: &Self::This) -> ::windows_core::Result<ISettingsIdentity>;
    fn Settings(this: &Self::This) -> ::windows_core::Result<IItemEnumerator>;
    fn Save(this: &Self::This, pushsettings: super::super::Foundation::BOOL) -> ::windows_core::Result<ISettingsResult>;
    fn GetSettingByPath(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<ISettingsItem>;
    fn CreateSettingByPath(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<ISettingsItem>;
    fn RemoveSettingByPath(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAttribute(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISettingsNamespace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISettingsNamespace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIdentity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Settings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Save(this, ::core::mem::transmute_copy(&pushsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSettingByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettingByPath(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSettingByPath(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSettingByPath(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttribute(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISettingsNamespace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdentity: GetIdentity::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, Impl, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, Impl, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISettingsResult_Impl: ::windows_core::BaseImpl {
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetErrorCode(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetContextDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLine(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetColumn(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSource(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ISettingsResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISettingsResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrout: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hrout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContextDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContextDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwcolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISettingsResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, Impl, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, Impl, OFFSET>,
            GetLine: GetLine::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITargetInfo_Impl: ::windows_core::BaseImpl {
    fn GetTargetMode(this: &Self::This) -> ::windows_core::Result<WcmTargetMode>;
    fn SetTargetMode(this: &Self::This, targetmode: WcmTargetMode) -> ::windows_core::Result<()>;
    fn GetTemporaryStoreLocation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTemporaryStoreLocation(this: &Self::This, temporarystorelocation: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetTargetID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTargetID(this: &Self::This, targetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetTargetProcessorArchitecture(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTargetProcessorArchitecture(this: &Self::This, processorarchitecture: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, offline: super::super::Foundation::BOOL, property: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProperty(this: &Self::This, offline: super::super::Foundation::BOOL, property: &::windows_core::PCWSTR, value: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IItemEnumerator>;
    fn ExpandTarget(this: &Self::This, offline: super::super::Foundation::BOOL, location: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExpandTargetPath(this: &Self::This, offline: super::super::Foundation::BOOL, location: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModulePath(this: &Self::This, module: &::windows_core::PCWSTR, path: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LoadModule(this: &Self::This, module: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
    fn SetWow64Context(this: &Self::This, installermodule: &::windows_core::PCWSTR, wow64context: *const u8) -> ::windows_core::Result<()>;
    fn TranslateWow64(this: &Self::This, clientarchitecture: &::windows_core::PCWSTR, value: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSchemaHiveLocation(this: &Self::This, pwzhivedir: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSchemaHiveLocation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSchemaHiveMountName(this: &Self::This, pwzmountname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSchemaHiveMountName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITargetInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTargetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetMode(this, ::core::mem::transmute_copy(&targetmode)).into())
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTemporaryStoreLocation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(temporarystorelocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, temporarystorelocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTemporaryStoreLocation(this, ::core::mem::transmute(&temporarystorelocation)).into())
        }
        unsafe extern "system" fn GetTargetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetID(this, ::core::mem::transmute(&targetid)).into())
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processorarchitecture: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetProcessorArchitecture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(processorarchitecture, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processorarchitecture: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetProcessorArchitecture(this, ::core::mem::transmute(&processorarchitecture)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows_core::PCWSTR, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&offline), ::core::mem::transmute(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&offline), ::core::mem::transmute(&property), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpandTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows_core::PCWSTR, expandedlocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpandTarget(this, ::core::mem::transmute_copy(&offline), ::core::mem::transmute(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expandedlocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpandTargetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows_core::PCWSTR, expandedlocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpandTargetPath(this, ::core::mem::transmute_copy(&offline), ::core::mem::transmute(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expandedlocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetModulePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, module: ::windows_core::PCWSTR, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModulePath(this, ::core::mem::transmute(&module), ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn LoadModule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, module: ::windows_core::PCWSTR, modulehandle: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadModule(this, ::core::mem::transmute(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modulehandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWow64Context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installermodule: ::windows_core::PCWSTR, wow64context: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWow64Context(this, ::core::mem::transmute(&installermodule), ::core::mem::transmute_copy(&wow64context)).into())
        }
        unsafe extern "system" fn TranslateWow64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientarchitecture: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR, translatedvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TranslateWow64(this, ::core::mem::transmute(&clientarchitecture), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(translatedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzhivedir: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSchemaHiveLocation(this, ::core::mem::transmute(&pwzhivedir)).into())
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phivelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSchemaHiveLocation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phivelocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzmountname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSchemaHiveMountName(this, ::core::mem::transmute(&pwzmountname)).into())
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmountname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSchemaHiveMountName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmountname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITargetInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTargetMode: GetTargetMode::<Identity, Impl, OFFSET>,
            SetTargetMode: SetTargetMode::<Identity, Impl, OFFSET>,
            GetTemporaryStoreLocation: GetTemporaryStoreLocation::<Identity, Impl, OFFSET>,
            SetTemporaryStoreLocation: SetTemporaryStoreLocation::<Identity, Impl, OFFSET>,
            GetTargetID: GetTargetID::<Identity, Impl, OFFSET>,
            SetTargetID: SetTargetID::<Identity, Impl, OFFSET>,
            GetTargetProcessorArchitecture: GetTargetProcessorArchitecture::<Identity, Impl, OFFSET>,
            SetTargetProcessorArchitecture: SetTargetProcessorArchitecture::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
            ExpandTarget: ExpandTarget::<Identity, Impl, OFFSET>,
            ExpandTargetPath: ExpandTargetPath::<Identity, Impl, OFFSET>,
            SetModulePath: SetModulePath::<Identity, Impl, OFFSET>,
            LoadModule: LoadModule::<Identity, Impl, OFFSET>,
            SetWow64Context: SetWow64Context::<Identity, Impl, OFFSET>,
            TranslateWow64: TranslateWow64::<Identity, Impl, OFFSET>,
            SetSchemaHiveLocation: SetSchemaHiveLocation::<Identity, Impl, OFFSET>,
            GetSchemaHiveLocation: GetSchemaHiveLocation::<Identity, Impl, OFFSET>,
            SetSchemaHiveMountName: SetSchemaHiveMountName::<Identity, Impl, OFFSET>,
            GetSchemaHiveMountName: GetSchemaHiveMountName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
