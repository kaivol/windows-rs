pub trait IEnumNetCfgBindingInterface_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, ppenum: *const ::core::option::Option<IEnumNetCfgBindingInterface>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumNetCfgBindingInterface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetCfgBindingInterface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        IEnumNetCfgBindingInterface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumNetCfgBindingPath_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, ppenum: *const ::core::option::Option<IEnumNetCfgBindingPath>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumNetCfgBindingPath {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetCfgBindingPath {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        IEnumNetCfgBindingPath_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumNetCfgComponent_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<INetCfgComponent>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, ppenum: *const ::core::option::Option<IEnumNetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumNetCfgComponent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetCfgComponent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        IEnumNetCfgComponent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfg_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Uninitialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Apply(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumComponents(this: &Self::This, pguidclass: *const ::windows_core::GUID, ppenumcomponent: *mut ::core::option::Option<IEnumNetCfgComponent>) -> ::windows_core::Result<()>;
    fn FindComponent(this: &Self::This, pszwinfid: &::windows_core::PCWSTR, pcomponent: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn QueryNetCfgClass(this: &Self::This, pguidclass: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfg {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfg {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&pvreserved)).into())
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this).into())
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Apply(this).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn EnumComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows_core::GUID, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumComponents(this, ::core::mem::transmute_copy(&pguidclass), ::core::mem::transmute_copy(&ppenumcomponent)).into())
        }
        unsafe extern "system" fn FindComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows_core::PCWSTR, pcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindComponent(this, ::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&pcomponent)).into())
        }
        unsafe extern "system" fn QueryNetCfgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryNetCfgClass(this, ::core::mem::transmute_copy(&pguidclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        INetCfg_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EnumComponents: EnumComponents::<Identity, Impl, OFFSET>,
            FindComponent: FindComponent::<Identity, Impl, OFFSET>,
            QueryNetCfgClass: QueryNetCfgClass::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgBindingInterface_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This, ppszwinterfacename: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetUpperComponent(this: &Self::This, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn GetLowerComponent(this: &Self::This, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgBindingInterface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgBindingInterface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&ppszwinterfacename)).into())
        }
        unsafe extern "system" fn GetUpperComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUpperComponent(this, ::core::mem::transmute_copy(&ppnccitem)).into())
        }
        unsafe extern "system" fn GetLowerComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLowerComponent(this, ::core::mem::transmute_copy(&ppnccitem)).into())
        }
        INetCfgBindingInterface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetUpperComponent: GetUpperComponent::<Identity, Impl, OFFSET>,
            GetLowerComponent: GetLowerComponent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingPath_Impl: ::windows_core::BaseImpl {
    fn IsSamePathAs(this: &Self::This, ppath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn IsSubPathOf(this: &Self::This, ppath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<()>;
    fn Enable(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPathToken(this: &Self::This, ppszwpathtoken: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetOwner(this: &Self::This, ppcomponent: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn GetDepth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumBindingInterfaces(this: &Self::This, ppenuminterface: *mut ::core::option::Option<IEnumNetCfgBindingInterface>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetCfgBindingPath {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgBindingPath {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSamePathAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSamePathAs(this, ::windows_core::from_raw_borrowed(&ppath)).into())
        }
        unsafe extern "system" fn IsSubPathOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSubPathOf(this, ::windows_core::from_raw_borrowed(&ppath)).into())
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEnabled(this).into())
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn GetPathToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPathToken(this, ::core::mem::transmute_copy(&ppszwpathtoken)).into())
        }
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOwner(this, ::core::mem::transmute_copy(&ppcomponent)).into())
        }
        unsafe extern "system" fn GetDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumBindingInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumBindingInterfaces(this, ::core::mem::transmute_copy(&ppenuminterface)).into())
        }
        INetCfgBindingPath_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsSamePathAs: IsSamePathAs::<Identity, Impl, OFFSET>,
            IsSubPathOf: IsSubPathOf::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            GetPathToken: GetPathToken::<Identity, Impl, OFFSET>,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetDepth: GetDepth::<Identity, Impl, OFFSET>,
            EnumBindingInterfaces: EnumBindingInterfaces::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgClass_Impl: ::windows_core::BaseImpl {
    fn FindComponent(this: &Self::This, pszwinfid: &::windows_core::PCWSTR, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn EnumComponents(this: &Self::This, ppenumcomponent: *mut ::core::option::Option<IEnumNetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgClass {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgClass {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows_core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindComponent(this, ::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&ppnccitem)).into())
        }
        unsafe extern "system" fn EnumComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumComponents(this, ::core::mem::transmute_copy(&ppenumcomponent)).into())
        }
        INetCfgClass_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindComponent: FindComponent::<Identity, Impl, OFFSET>,
            EnumComponents: EnumComponents::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup_Impl: ::windows_core::BaseImpl {
    fn SelectAndInstall(this: &Self::This, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn Install(this: &Self::This, pszwinfid: &::windows_core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: &::windows_core::PCWSTR, pszwanswersections: &::windows_core::PCWSTR, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn DeInstall(this: &Self::This, pcomponent: ::core::option::Option<&INetCfgComponent>, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetCfgClassSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgClassSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SelectAndInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectAndInstall(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&ppnccitem)).into())
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows_core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: ::windows_core::PCWSTR, pszwanswersections: ::windows_core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Install(this, ::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno), ::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersections), ::core::mem::transmute_copy(&ppnccitem)).into())
        }
        unsafe extern "system" fn DeInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomponent: *mut ::core::ffi::c_void, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeInstall(this, ::windows_core::from_raw_borrowed(&pcomponent), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&pmszwrefs)).into())
        }
        INetCfgClassSetup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SelectAndInstall: SelectAndInstall::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            DeInstall: DeInstall::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup2_Impl: ::windows_core::BaseImpl + INetCfgClassSetup_Impl {
    fn UpdateNonEnumeratedComponent(this: &Self::This, picomp: ::core::option::Option<&INetCfgComponent>, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetCfgClassSetup2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INetCfgClassSetup);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClassSetup2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgClassSetup2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgClassSetup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateNonEnumeratedComponent(this, ::windows_core::from_raw_borrowed(&picomp), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno)).into())
        }
        INetCfgClassSetup2_Vtbl {
            base__: <INetCfgClassSetup as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateNonEnumeratedComponent: UpdateNonEnumeratedComponent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait INetCfgComponent_Impl: ::windows_core::BaseImpl {
    fn GetDisplayName(this: &Self::This, ppszwdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetDisplayName(this: &Self::This, pszwdisplayname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetHelpText(this: &Self::This, pszwhelptext: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetId(this: &Self::This, ppszwid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetCharacteristics(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetInstanceGuid(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetPnpDevNodeId(this: &Self::This, ppszwdevnodeid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetClassGuid(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetBindName(this: &Self::This, ppszwbindname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetDeviceStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OpenParamKey(this: &Self::This, phkey: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn RaisePropertyUi(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for INetCfgComponent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayName(this, ::core::mem::transmute_copy(&ppszwdisplayname)).into())
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwdisplayname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&pszwdisplayname)).into())
        }
        unsafe extern "system" fn GetHelpText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHelpText(this, ::core::mem::transmute_copy(&pszwhelptext)).into())
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetId(this, ::core::mem::transmute_copy(&ppszwid)).into())
        }
        unsafe extern "system" fn GetCharacteristics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCharacteristics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcharacteristics, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstanceGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInstanceGuid(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetPnpDevNodeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPnpDevNodeId(this, ::core::mem::transmute_copy(&ppszwdevnodeid)).into())
        }
        unsafe extern "system" fn GetClassGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassGuid(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetBindName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindName(this, ::core::mem::transmute_copy(&ppszwbindname)).into())
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenParamKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenParamKey(this, ::core::mem::transmute_copy(&phkey)).into())
        }
        unsafe extern "system" fn RaisePropertyUi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RaisePropertyUi(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&punkcontext)).into())
        }
        INetCfgComponent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetHelpText: GetHelpText::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, Impl, OFFSET>,
            GetInstanceGuid: GetInstanceGuid::<Identity, Impl, OFFSET>,
            GetPnpDevNodeId: GetPnpDevNodeId::<Identity, Impl, OFFSET>,
            GetClassGuid: GetClassGuid::<Identity, Impl, OFFSET>,
            GetBindName: GetBindName::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            OpenParamKey: OpenParamKey::<Identity, Impl, OFFSET>,
            RaisePropertyUi: RaisePropertyUi::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgComponentBindings_Impl: ::windows_core::BaseImpl {
    fn BindTo(this: &Self::This, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn UnbindFrom(this: &Self::This, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn SupportsBindingInterface(this: &Self::This, dwflags: u32, pszwinterfacename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsBoundTo(this: &Self::This, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn IsBindableTo(this: &Self::This, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn EnumBindingPaths(this: &Self::This, dwflags: u32, ppienum: *mut ::core::option::Option<IEnumNetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn MoveBefore(this: &Self::This, pncbitemsrc: ::core::option::Option<&INetCfgBindingPath>, pncbitemdest: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn MoveAfter(this: &Self::This, pncbitemsrc: ::core::option::Option<&INetCfgBindingPath>, pncbitemdest: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgComponentBindings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentBindings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindTo(this, ::windows_core::from_raw_borrowed(&pnccitem)).into())
        }
        unsafe extern "system" fn UnbindFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnbindFrom(this, ::windows_core::from_raw_borrowed(&pnccitem)).into())
        }
        unsafe extern "system" fn SupportsBindingInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SupportsBindingInterface(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszwinterfacename)).into())
        }
        unsafe extern "system" fn IsBoundTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsBoundTo(this, ::windows_core::from_raw_borrowed(&pnccitem)).into())
        }
        unsafe extern "system" fn IsBindableTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsBindableTo(this, ::windows_core::from_raw_borrowed(&pnccitem)).into())
        }
        unsafe extern "system" fn EnumBindingPaths<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumBindingPaths(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppienum)).into())
        }
        unsafe extern "system" fn MoveBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveBefore(this, ::windows_core::from_raw_borrowed(&pncbitemsrc), ::windows_core::from_raw_borrowed(&pncbitemdest)).into())
        }
        unsafe extern "system" fn MoveAfter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveAfter(this, ::windows_core::from_raw_borrowed(&pncbitemsrc), ::windows_core::from_raw_borrowed(&pncbitemdest)).into())
        }
        INetCfgComponentBindings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindTo: BindTo::<Identity, Impl, OFFSET>,
            UnbindFrom: UnbindFrom::<Identity, Impl, OFFSET>,
            SupportsBindingInterface: SupportsBindingInterface::<Identity, Impl, OFFSET>,
            IsBoundTo: IsBoundTo::<Identity, Impl, OFFSET>,
            IsBindableTo: IsBindableTo::<Identity, Impl, OFFSET>,
            EnumBindingPaths: EnumBindingPaths::<Identity, Impl, OFFSET>,
            MoveBefore: MoveBefore::<Identity, Impl, OFFSET>,
            MoveAfter: MoveAfter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentControl_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, picomp: ::core::option::Option<&INetCfgComponent>, pinetcfg: ::core::option::Option<&INetCfg>, finstalling: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ApplyRegistryChanges(this: &Self::This) -> ::windows_core::Result<()>;
    fn ApplyPnpChanges(this: &Self::This, picallback: ::core::option::Option<&INetCfgPnpReconfigCallback>) -> ::windows_core::Result<()>;
    fn CancelChanges(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetCfgComponentControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, pinetcfg: *mut ::core::ffi::c_void, finstalling: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&picomp), ::windows_core::from_raw_borrowed(&pinetcfg), ::core::mem::transmute_copy(&finstalling)).into())
        }
        unsafe extern "system" fn ApplyRegistryChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyRegistryChanges(this).into())
        }
        unsafe extern "system" fn ApplyPnpChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyPnpChanges(this, ::windows_core::from_raw_borrowed(&picallback)).into())
        }
        unsafe extern "system" fn CancelChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelChanges(this).into())
        }
        INetCfgComponentControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ApplyRegistryChanges: ApplyRegistryChanges::<Identity, Impl, OFFSET>,
            ApplyPnpChanges: ApplyPnpChanges::<Identity, Impl, OFFSET>,
            CancelChanges: CancelChanges::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgComponentNotifyBinding_Impl: ::windows_core::BaseImpl {
    fn QueryBindingPath(this: &Self::This, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn NotifyBindingPath(this: &Self::This, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgComponentNotifyBinding {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentNotifyBinding {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryBindingPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryBindingPath(this, ::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into())
        }
        unsafe extern "system" fn NotifyBindingPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyBindingPath(this, ::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into())
        }
        INetCfgComponentNotifyBinding_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryBindingPath: QueryBindingPath::<Identity, Impl, OFFSET>,
            NotifyBindingPath: NotifyBindingPath::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgComponentNotifyGlobal_Impl: ::windows_core::BaseImpl {
    fn GetSupportedNotifications(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SysQueryBindingPath(this: &Self::This, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn SysNotifyBindingPath(this: &Self::This, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn SysNotifyComponent(this: &Self::This, dwchangeflag: u32, picomp: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgComponentNotifyGlobal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentNotifyGlobal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedNotifications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwnotifications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SysQueryBindingPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SysQueryBindingPath(this, ::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into())
        }
        unsafe extern "system" fn SysNotifyBindingPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SysNotifyBindingPath(this, ::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into())
        }
        unsafe extern "system" fn SysNotifyComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SysNotifyComponent(this, ::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&picomp)).into())
        }
        INetCfgComponentNotifyGlobal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedNotifications: GetSupportedNotifications::<Identity, Impl, OFFSET>,
            SysQueryBindingPath: SysQueryBindingPath::<Identity, Impl, OFFSET>,
            SysNotifyBindingPath: SysNotifyBindingPath::<Identity, Impl, OFFSET>,
            SysNotifyComponent: SysNotifyComponent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentPropertyUi_Impl: ::windows_core::BaseImpl {
    fn QueryPropertyUi(this: &Self::This, punkreserved: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetContext(this: &Self::This, punkreserved: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MergePropPages(this: &Self::This, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ValidateProperties(this: &Self::This, hwndsheet: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn ApplyProperties(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelProperties(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetCfgComponentPropertyUi {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentPropertyUi {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryPropertyUi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryPropertyUi(this, ::windows_core::from_raw_borrowed(&punkreserved)).into())
        }
        unsafe extern "system" fn SetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContext(this, ::windows_core::from_raw_borrowed(&punkreserved)).into())
        }
        unsafe extern "system" fn MergePropPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MergePropPages(this, ::core::mem::transmute_copy(&pdwdefpages), ::core::mem::transmute_copy(&pahpspprivate), ::core::mem::transmute_copy(&pcpages), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pszstartpage)).into())
        }
        unsafe extern "system" fn ValidateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateProperties(this, ::core::mem::transmute_copy(&hwndsheet)).into())
        }
        unsafe extern "system" fn ApplyProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyProperties(this).into())
        }
        unsafe extern "system" fn CancelProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelProperties(this).into())
        }
        INetCfgComponentPropertyUi_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryPropertyUi: QueryPropertyUi::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            MergePropPages: MergePropPages::<Identity, Impl, OFFSET>,
            ValidateProperties: ValidateProperties::<Identity, Impl, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, Impl, OFFSET>,
            CancelProperties: CancelProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgComponentSetup_Impl: ::windows_core::BaseImpl {
    fn Install(this: &Self::This, dwsetupflags: u32) -> ::windows_core::Result<()>;
    fn Upgrade(this: &Self::This, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows_core::Result<()>;
    fn ReadAnswerFile(this: &Self::This, pszwanswerfile: &::windows_core::PCWSTR, pszwanswersections: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Removing(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgComponentSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Install(this, ::core::mem::transmute_copy(&dwsetupflags)).into())
        }
        unsafe extern "system" fn Upgrade<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Upgrade(this, ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefombuildno)).into())
        }
        unsafe extern "system" fn ReadAnswerFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows_core::PCWSTR, pszwanswersections: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadAnswerFile(this, ::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersections)).into())
        }
        unsafe extern "system" fn Removing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Removing(this).into())
        }
        INetCfgComponentSetup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Install: Install::<Identity, Impl, OFFSET>,
            Upgrade: Upgrade::<Identity, Impl, OFFSET>,
            ReadAnswerFile: ReadAnswerFile::<Identity, Impl, OFFSET>,
            Removing: Removing::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgComponentSysPrep_Impl: ::windows_core::BaseImpl {
    fn SaveAdapterParameters(this: &Self::This, pncsp: ::core::option::Option<&INetCfgSysPrep>, pszwanswersections: &::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreAdapterParameters(this: &Self::This, pszwanswerfile: &::windows_core::PCWSTR, pszwanswersection: &::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgComponentSysPrep {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentSysPrep {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SaveAdapterParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncsp: *mut ::core::ffi::c_void, pszwanswersections: ::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveAdapterParameters(this, ::windows_core::from_raw_borrowed(&pncsp), ::core::mem::transmute(&pszwanswersections), ::core::mem::transmute_copy(&padapterinstanceguid)).into())
        }
        unsafe extern "system" fn RestoreAdapterParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows_core::PCWSTR, pszwanswersection: ::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreAdapterParameters(this, ::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersection), ::core::mem::transmute_copy(&padapterinstanceguid)).into())
        }
        INetCfgComponentSysPrep_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SaveAdapterParameters: SaveAdapterParameters::<Identity, Impl, OFFSET>,
            RestoreAdapterParameters: RestoreAdapterParameters::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgComponentUpperEdge_Impl: ::windows_core::BaseImpl {
    fn GetInterfaceIdsForAdapter(this: &Self::This, padapter: ::core::option::Option<&INetCfgComponent>, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AddInterfacesToAdapter(this: &Self::This, padapter: ::core::option::Option<&INetCfgComponent>, dwnuminterfaces: u32) -> ::windows_core::Result<()>;
    fn RemoveInterfacesFromAdapter(this: &Self::This, padapter: ::core::option::Option<&INetCfgComponent>, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgComponentUpperEdge {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgComponentUpperEdge {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterfaceIdsForAdapter(this, ::windows_core::from_raw_borrowed(&padapter), ::core::mem::transmute_copy(&pdwnuminterfaces), ::core::mem::transmute_copy(&ppguidinterfaceids)).into())
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddInterfacesToAdapter(this, ::windows_core::from_raw_borrowed(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces)).into())
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveInterfacesFromAdapter(this, ::windows_core::from_raw_borrowed(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces), ::core::mem::transmute_copy(&pguidinterfaceids)).into())
        }
        INetCfgComponentUpperEdge_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInterfaceIdsForAdapter: GetInterfaceIdsForAdapter::<Identity, Impl, OFFSET>,
            AddInterfacesToAdapter: AddInterfacesToAdapter::<Identity, Impl, OFFSET>,
            RemoveInterfacesFromAdapter: RemoveInterfacesFromAdapter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgLock_Impl: ::windows_core::BaseImpl {
    fn AcquireWriteLock(this: &Self::This, cmstimeout: u32, pszwclientdescription: &::windows_core::PCWSTR, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn ReleaseWriteLock(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsWriteLocked(this: &Self::This, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgLock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgLock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireWriteLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: ::windows_core::PCWSTR, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireWriteLock(this, ::core::mem::transmute_copy(&cmstimeout), ::core::mem::transmute(&pszwclientdescription), ::core::mem::transmute_copy(&ppszwclientdescription)).into())
        }
        unsafe extern "system" fn ReleaseWriteLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseWriteLock(this).into())
        }
        unsafe extern "system" fn IsWriteLocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsWriteLocked(this, ::core::mem::transmute_copy(&ppszwclientdescription)).into())
        }
        INetCfgLock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireWriteLock: AcquireWriteLock::<Identity, Impl, OFFSET>,
            ReleaseWriteLock: ReleaseWriteLock::<Identity, Impl, OFFSET>,
            IsWriteLocked: IsWriteLocked::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetCfgPnpReconfigCallback_Impl: ::windows_core::BaseImpl {
    fn SendPnpReconfig(this: &Self::This, layer: NCPNP_RECONFIG_LAYER, pszwupper: &::windows_core::PCWSTR, pszwlower: &::windows_core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetCfgPnpReconfigCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgPnpReconfigCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendPnpReconfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: ::windows_core::PCWSTR, pszwlower: ::windows_core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendPnpReconfig(this, ::core::mem::transmute_copy(&layer), ::core::mem::transmute(&pszwupper), ::core::mem::transmute(&pszwlower), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwsizeofdata)).into())
        }
        INetCfgPnpReconfigCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendPnpReconfig: SendPnpReconfig::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgSysPrep_Impl: ::windows_core::BaseImpl {
    fn HrSetupSetFirstDword(this: &Self::This, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::Result<()>;
    fn HrSetupSetFirstString(this: &Self::This, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn HrSetupSetFirstStringAsBool(this: &Self::This, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn HrSetupSetFirstMultiSzField(this: &Self::This, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, pmszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetCfgSysPrep {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetCfgSysPrep {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HrSetupSetFirstDword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrSetupSetFirstDword(this, ::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn HrSetupSetFirstString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrSetupSetFirstString(this, ::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute(&pwszvalue)).into())
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrSetupSetFirstStringAsBool(this, ::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute_copy(&fvalue)).into())
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, pmszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrSetupSetFirstMultiSzField(this, ::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute(&pmszvalue)).into())
        }
        INetCfgSysPrep_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HrSetupSetFirstDword: HrSetupSetFirstDword::<Identity, Impl, OFFSET>,
            HrSetupSetFirstString: HrSetupSetFirstString::<Identity, Impl, OFFSET>,
            HrSetupSetFirstStringAsBool: HrSetupSetFirstStringAsBool::<Identity, Impl, OFFSET>,
            HrSetupSetFirstMultiSzField: HrSetupSetFirstMultiSzField::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetLanConnectionUiInfo_Impl: ::windows_core::BaseImpl {
    fn GetDeviceGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for INetLanConnectionUiInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetLanConnectionUiInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetLanConnectionUiInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetLanConnectionUiInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetLanConnectionUiInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDeviceGuid: GetDeviceGuid::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetRasConnectionIpUiInfo_Impl: ::windows_core::BaseImpl {
    fn GetUiInfo(this: &Self::This, pinfo: *mut RASCON_IPUI) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetRasConnectionIpUiInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetRasConnectionIpUiInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUiInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUiInfo(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        INetRasConnectionIpUiInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetUiInfo: GetUiInfo::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IProvisioningDomain_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, pszwpathtofolder: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Query(this: &Self::This, pszwdomain: &::windows_core::PCWSTR, pszwlanguage: &::windows_core::PCWSTR, pszwxpathquery: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IProvisioningDomain {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvisioningDomain {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&pszwpathtofolder)).into())
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwdomain: ::windows_core::PCWSTR, pszwlanguage: ::windows_core::PCWSTR, pszwxpathquery: ::windows_core::PCWSTR, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this, ::core::mem::transmute(&pszwdomain), ::core::mem::transmute(&pszwlanguage), ::core::mem::transmute(&pszwxpathquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvisioningDomain_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IProvisioningProfileWireless_Impl: ::windows_core::BaseImpl {
    fn CreateProfile(this: &Self::This, bstrxmlwirelessconfigprofile: &::windows_core::BSTR, bstrxmlconnectionconfigprofile: &::windows_core::BSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IProvisioningProfileWireless {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvisioningProfileWireless_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvisioningProfileWireless {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvisioningProfileWireless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmlconnectionconfigprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>, padapterinstanceguid: *const ::windows_core::GUID, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProfile(this, ::core::mem::transmute(&bstrxmlwirelessconfigprofile), ::core::mem::transmute(&bstrxmlconnectionconfigprofile), ::core::mem::transmute_copy(&padapterinstanceguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvisioningProfileWireless_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateProfile: CreateProfile::<Identity, Impl, OFFSET> }
    };
}
