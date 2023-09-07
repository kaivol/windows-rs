#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalog_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetCollection(this: &Self::This, bstrcollname: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Connect(this: &Self::This, bstrconnectstring: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(this: &Self::This, retval: *mut i32) -> ::windows_core::Result<()>;
    fn MinorVersion(this: &Self::This, retval: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICatalog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatalog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCollection(this, ::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Connect(this, ::core::mem::transmute(&bstrconnectstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MajorVersion(this, ::core::mem::transmute_copy(&retval)).into())
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MinorVersion(this, ::core::mem::transmute_copy(&retval)).into())
        }
        ICatalog_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComponentUtil_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn InstallComponent(this: &Self::This, bstrdllfile: &::windows_core::BSTR, bstrtypelibfile: &::windows_core::BSTR, bstrproxystubdllfile: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportComponent(this: &Self::This, bstrclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportComponentByName(this: &Self::This, bstrprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetCLSIDs(this: &Self::This, bstrdllfile: &::windows_core::BSTR, bstrtypelibfile: &::windows_core::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IComponentUtil {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComponentUtil {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstallComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrproxystubdllfile: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallComponent(this, ::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute(&bstrproxystubdllfile)).into())
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportComponent(this, ::core::mem::transmute(&bstrclsid)).into())
        }
        unsafe extern "system" fn ImportComponentByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportComponentByName(this, ::core::mem::transmute(&bstrprogid)).into())
        }
        unsafe extern "system" fn GetCLSIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCLSIDs(this, ::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute_copy(&aclsids)).into())
        }
        IComponentUtil_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstallComponent: InstallComponent::<Identity, Impl, OFFSET>,
            ImportComponent: ImportComponent::<Identity, Impl, OFFSET>,
            ImportComponentByName: ImportComponentByName::<Identity, Impl, OFFSET>,
            GetCLSIDs: GetCLSIDs::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPackageUtil_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn InstallPackage(this: &Self::This, bstrpackagefile: &::windows_core::BSTR, bstrinstallpath: &::windows_core::BSTR, loptions: i32) -> ::windows_core::Result<()>;
    fn ExportPackage(this: &Self::This, bstrpackageid: &::windows_core::BSTR, bstrpackagefile: &::windows_core::BSTR, loptions: i32) -> ::windows_core::Result<()>;
    fn ShutdownPackage(this: &Self::This, bstrpackageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPackageUtil {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPackageUtil {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstallPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpackagefile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinstallpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallPackage(this, ::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute(&bstrinstallpath), ::core::mem::transmute_copy(&loptions)).into())
        }
        unsafe extern "system" fn ExportPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpackagefile: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportPackage(this, ::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute_copy(&loptions)).into())
        }
        unsafe extern "system" fn ShutdownPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownPackage(this, ::core::mem::transmute(&bstrpackageid)).into())
        }
        IPackageUtil_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstallPackage: InstallPackage::<Identity, Impl, OFFSET>,
            ExportPackage: ExportPackage::<Identity, Impl, OFFSET>,
            ShutdownPackage: ShutdownPackage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteComponentUtil_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn InstallRemoteComponent(this: &Self::This, bstrserver: &::windows_core::BSTR, bstrpackageid: &::windows_core::BSTR, bstrclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InstallRemoteComponentByName(this: &Self::This, bstrserver: &::windows_core::BSTR, bstrpackagename: &::windows_core::BSTR, bstrprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRemoteComponentUtil {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteComponentUtil {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstallRemoteComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallRemoteComponent(this, ::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrclsid)).into())
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpackagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallRemoteComponentByName(this, ::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackagename), ::core::mem::transmute(&bstrprogid)).into())
        }
        IRemoteComponentUtil_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstallRemoteComponent: InstallRemoteComponent::<Identity, Impl, OFFSET>,
            InstallRemoteComponentByName: InstallRemoteComponentByName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRoleAssociationUtil_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AssociateRole(this: &Self::This, bstrroleid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AssociateRoleByName(this: &Self::This, bstrrolename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRoleAssociationUtil {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRoleAssociationUtil {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssociateRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateRole(this, ::core::mem::transmute(&bstrroleid)).into())
        }
        unsafe extern "system" fn AssociateRoleByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateRoleByName(this, ::core::mem::transmute(&bstrrolename)).into())
        }
        IRoleAssociationUtil_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssociateRole: AssociateRole::<Identity, Impl, OFFSET>,
            AssociateRoleByName: AssociateRoleByName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
