pub trait IGetClusterDataInfo_Impl: ::windows_core::BaseImpl {
    fn GetClusterName(this: &Self::This, lpszname: ::windows_core::BSTR, pcchname: *mut i32) -> ::windows_core::Result<()>;
    fn GetClusterHandle(this: &Self::This) -> HCLUSTER;
    fn GetObjectCount(this: &Self::This) -> i32;
}
impl ::windows_core::Iids for IGetClusterDataInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterDataInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClusterName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcchname: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClusterName(this, ::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn GetClusterHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> HCLUSTER {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClusterHandle(this))
        }
        unsafe extern "system" fn GetObjectCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectCount(this))
        }
        IGetClusterDataInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClusterName: GetClusterName::<Identity, Impl, OFFSET>,
            GetClusterHandle: GetClusterHandle::<Identity, Impl, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetClusterGroupInfo_Impl: ::windows_core::BaseImpl {
    fn GetGroupHandle(this: &Self::This, lobjindex: i32) -> HGROUP;
}
impl ::windows_core::Iids for IGetClusterGroupInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterGroupInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterGroupInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGroupHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterGroupInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> HGROUP {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGroupHandle(this, ::core::mem::transmute_copy(&lobjindex)))
        }
        IGetClusterGroupInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetGroupHandle: GetGroupHandle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetClusterNetInterfaceInfo_Impl: ::windows_core::BaseImpl {
    fn GetNetInterfaceHandle(this: &Self::This, lobjindex: i32) -> HNETINTERFACE;
}
impl ::windows_core::Iids for IGetClusterNetInterfaceInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterNetInterfaceInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterNetInterfaceInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNetInterfaceHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterNetInterfaceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> HNETINTERFACE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNetInterfaceHandle(this, ::core::mem::transmute_copy(&lobjindex)))
        }
        IGetClusterNetInterfaceInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNetInterfaceHandle: GetNetInterfaceHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetClusterNetworkInfo_Impl: ::windows_core::BaseImpl {
    fn GetNetworkHandle(this: &Self::This, lobjindex: i32) -> HNETWORK;
}
impl ::windows_core::Iids for IGetClusterNetworkInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterNetworkInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterNetworkInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNetworkHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterNetworkInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> HNETWORK {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNetworkHandle(this, ::core::mem::transmute_copy(&lobjindex)))
        }
        IGetClusterNetworkInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNetworkHandle: GetNetworkHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetClusterNodeInfo_Impl: ::windows_core::BaseImpl {
    fn GetNodeHandle(this: &Self::This, lobjindex: i32) -> HNODE;
}
impl ::windows_core::Iids for IGetClusterNodeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterNodeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterNodeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNodeHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterNodeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> HNODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNodeHandle(this, ::core::mem::transmute_copy(&lobjindex)))
        }
        IGetClusterNodeInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetNodeHandle: GetNodeHandle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetClusterObjectInfo_Impl: ::windows_core::BaseImpl {
    fn GetObjectName(this: &Self::This, lobjindex: i32, lpszname: ::windows_core::BSTR, pcchname: *mut i32) -> ::windows_core::Result<()>;
    fn GetObjectType(this: &Self::This, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE;
}
impl ::windows_core::Iids for IGetClusterObjectInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterObjectInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterObjectInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcchname: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectName(this, ::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn GetObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectType(this, ::core::mem::transmute_copy(&lobjindex)))
        }
        IGetClusterObjectInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectName: GetObjectName::<Identity, Impl, OFFSET>,
            GetObjectType: GetObjectType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterResourceInfo_Impl: ::windows_core::BaseImpl {
    fn GetResourceHandle(this: &Self::This, lobjindex: i32) -> HRESOURCE;
    fn GetResourceTypeName(this: &Self::This, lobjindex: i32, lpszrestypename: ::windows_core::BSTR, pcchrestypename: *mut i32) -> ::windows_core::Result<()>;
    fn GetResourceNetworkName(this: &Self::This, lobjindex: i32, lpsznetname: ::windows_core::BSTR, pcchnetname: *mut u32) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IGetClusterResourceInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterResourceInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResourceHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> HRESOURCE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceHandle(this, ::core::mem::transmute_copy(&lobjindex)))
        }
        unsafe extern "system" fn GetResourceTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszrestypename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcchrestypename: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceTypeName(this, ::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpszrestypename), ::core::mem::transmute_copy(&pcchrestypename)).into())
        }
        unsafe extern "system" fn GetResourceNetworkName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpsznetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcchnetname: *mut u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceNetworkName(this, ::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpsznetname), ::core::mem::transmute_copy(&pcchnetname)))
        }
        IGetClusterResourceInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResourceHandle: GetResourceHandle::<Identity, Impl, OFFSET>,
            GetResourceTypeName: GetResourceTypeName::<Identity, Impl, OFFSET>,
            GetResourceNetworkName: GetResourceNetworkName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IGetClusterUIInfo_Impl: ::windows_core::BaseImpl {
    fn GetClusterName(this: &Self::This, lpszname: ::windows_core::BSTR, pcchname: *mut i32) -> ::windows_core::Result<()>;
    fn GetLocale(this: &Self::This) -> u32;
    fn GetFont(this: &Self::This) -> super::super::Graphics::Gdi::HFONT;
    fn GetIcon(this: &Self::This) -> super::super::UI::WindowsAndMessaging::HICON;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IGetClusterUIInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetClusterUIInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClusterName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcchname: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClusterName(this, ::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into())
        }
        unsafe extern "system" fn GetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocale(this))
        }
        unsafe extern "system" fn GetFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFont(this))
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIcon(this))
        }
        IGetClusterUIInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClusterName: GetClusterName::<Identity, Impl, OFFSET>,
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
            GetFont: GetFont::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusApplication_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DomainNames(this: &Self::This) -> ::windows_core::Result<ISDomainNames>;
    fn get_ClusterNames(this: &Self::This, bstrdomainname: &::windows_core::BSTR) -> ::windows_core::Result<ISClusterNames>;
    fn OpenCluster(this: &Self::This, bstrclustername: &::windows_core::BSTR) -> ::windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DomainNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdomains: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdomains, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ClusterNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdomainname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppclusters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ClusterNames(this, ::core::mem::transmute(&bstrdomainname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenCluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclustername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenCluster(this, ::core::mem::transmute(&bstrclustername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusApplication_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DomainNames: DomainNames::<Identity, Impl, OFFSET>,
            get_ClusterNames: get_ClusterNames::<Identity, Impl, OFFSET>,
            OpenCluster: OpenCluster::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusCryptoKeys_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddItem(this: &Self::This, bstrcryptokey: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusCryptoKeys {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusCryptoKeys {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrcyrptokey: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcyrptokey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcryptokey: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::core::mem::transmute(&bstrcryptokey)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusCryptoKeys_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusDisk_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Signature(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ScsiAddress(this: &Self::This) -> ::windows_core::Result<ISClusScsiAddress>;
    fn DiskNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Partitions(this: &Self::This) -> ::windows_core::Result<ISClusPartitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusDisk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusDisk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Signature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsignature: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Signature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScsiAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscsiaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScsiAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscsiaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DiskNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldisknumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiskNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldisknumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Partitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppartitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Partitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppartitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusDisk_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Signature: Signature::<Identity, Impl, OFFSET>,
            ScsiAddress: ScsiAddress::<Identity, Impl, OFFSET>,
            DiskNumber: DiskNumber::<Identity, Impl, OFFSET>,
            Partitions: Partitions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusDisks_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusDisk>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusDisks {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusDisks {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusDisks_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetInterface_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn State(this: &Self::This) -> ::windows_core::Result<CLUSTER_NETINTERFACE_STATE>;
    fn Cluster(this: &Self::This) -> ::windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNetInterface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNetInterface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cluster(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNetInterface_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetInterfaces_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNetInterfaces {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNetInterfaces {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNetInterfaces_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetwork_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrnetworkname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NetworkID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn State(this: &Self::This) -> ::windows_core::Result<CLUSTER_NETWORK_STATE>;
    fn NetInterfaces(this: &Self::This) -> ::windows_core::Result<ISClusNetworkNetInterfaces>;
    fn Cluster(this: &Self::This) -> ::windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNetwork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNetwork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnetworkname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrnetworkname)).into())
        }
        unsafe extern "system" fn NetworkID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnetworkid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnetworkid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cluster(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNetwork_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            NetworkID: NetworkID::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetworkNetInterfaces_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNetworkNetInterfaces {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNetworkNetInterfaces {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNetworkNetInterfaces_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetworks_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNetwork>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNetworks {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNetworks {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNetworks_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNode_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn NodeID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn State(this: &Self::This) -> ::windows_core::Result<CLUSTER_NODE_STATE>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Evict(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResourceGroups(this: &Self::This) -> ::windows_core::Result<ISClusResGroups>;
    fn Cluster(this: &Self::This) -> ::windows_core::Result<ISCluster>;
    fn NetInterfaces(this: &Self::This) -> ::windows_core::Result<ISClusNodeNetInterfaces>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NodeID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnodeid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnodeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Evict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Evict(this).into())
        }
        unsafe extern "system" fn ResourceGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresourcegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResourceGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cluster(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNode_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            NodeID: NodeID::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNodeNetInterfaces_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNodeNetInterfaces {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNodeNetInterfaces {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNodeNetInterfaces_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNodes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusNodes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusNodes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusNodes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPartition_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeLabel(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SerialNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MaximumComponentLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FileSystemFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FileSystem(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusPartition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusPartition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvolumelabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeLabel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvolumelabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plserialnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SerialNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plserialnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaximumComponentLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaximumComponentLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaximumcomponentlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FileSystemFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfilesystemflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSystemFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfilesystemflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FileSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfilesystem: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfilesystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusPartition_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Flags: Flags::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            VolumeLabel: VolumeLabel::<Identity, Impl, OFFSET>,
            SerialNumber: SerialNumber::<Identity, Impl, OFFSET>,
            MaximumComponentLength: MaximumComponentLength::<Identity, Impl, OFFSET>,
            FileSystemFlags: FileSystemFlags::<Identity, Impl, OFFSET>,
            FileSystem: FileSystem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPartitionEx_Impl: ::windows_core::BaseImpl + ISClusPartition_Impl {
    fn TotalSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FreeSpace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PartitionNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn VolumeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusPartitionEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISClusPartition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusPartitionEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TotalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltotalsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltotalsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfreespace: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeSpace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfreespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldevicenumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldevicenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PartitionNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpartitionnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PartitionNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpartitionnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvolumeguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvolumeguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusPartitionEx_Vtbl {
            base__: <ISClusPartition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TotalSize: TotalSize::<Identity, Impl, OFFSET>,
            FreeSpace: FreeSpace::<Identity, Impl, OFFSET>,
            DeviceNumber: DeviceNumber::<Identity, Impl, OFFSET>,
            PartitionNumber: PartitionNumber::<Identity, Impl, OFFSET>,
            VolumeGuid: VolumeGuid::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPartitions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusPartition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusPartitions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusPartitions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pppartition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppartition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusPartitions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusProperties_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusProperty>;
    fn CreateItem(this: &Self::This, bstrname: &::windows_core::BSTR, varvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusProperty>;
    fn UseDefaultValue(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SaveChanges(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Private(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Common(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Modified(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: super::super::System::Variant::VARIANT, pproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UseDefaultValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseDefaultValue(this, ::core::mem::transmute(&varindex)).into())
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarstatuscode: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveChanges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreadonly, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Private<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Private(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprivate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Common<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Common(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcommon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Modified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Modified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusProperties_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Private: Private::<Identity, Impl, OFFSET>,
            Common: Common::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusProperty_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ValueCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Values(this: &Self::This) -> ::windows_core::Result<ISClusPropertyValues>;
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(this: &Self::This, varvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(this: &Self::This, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows_core::Result<()>;
    fn Format(this: &Self::This) -> ::windows_core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(this: &Self::This, format: CLUSTER_PROPERTY_FORMAT) -> ::windows_core::Result<()>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Private(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Common(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Modified(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn UseDefaultValue(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValueCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Values<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Values(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterpropertyvalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&varvalue)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Format(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&format)).into())
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreadonly, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Private<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Private(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprivate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Common<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Common(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcommon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Modified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Modified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UseDefaultValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseDefaultValue(this).into())
        }
        ISClusProperty_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            ValueCount: ValueCount::<Identity, Impl, OFFSET>,
            Values: Values::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Private: Private::<Identity, Impl, OFFSET>,
            Common: Common::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPropertyValue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(this: &Self::This, varvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(this: &Self::This, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows_core::Result<()>;
    fn Format(this: &Self::This) -> ::windows_core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(this: &Self::This, format: CLUSTER_PROPERTY_FORMAT) -> ::windows_core::Result<()>;
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DataCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Data(this: &Self::This) -> ::windows_core::Result<ISClusPropertyValueData>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusPropertyValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusPropertyValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&varvalue)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Format(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&format)).into())
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvaluedata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterpropertyvaluedata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusPropertyValue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            DataCount: DataCount::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPropertyValueData_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CreateItem(this: &Self::This, varvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusPropertyValueData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusPropertyValueData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT, pvardata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusPropertyValueData_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPropertyValues_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusPropertyValue>;
    fn CreateItem(this: &Self::This, bstrname: &::windows_core::BSTR, varvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusPropertyValue>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusPropertyValues {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusPropertyValues {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pppropertyvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: super::super::System::Variant::VARIANT, pppropertyvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusPropertyValues_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusRefObject_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusRefObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRefObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusRefObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRefObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusRefObject_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Handle: Handle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusRegistryKeys_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddItem(this: &Self::This, bstrregistrykey: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusRegistryKeys {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusRegistryKeys {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrregistrykey: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrregistrykey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrregistrykey: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::core::mem::transmute(&bstrregistrykey)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusRegistryKeys_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResDependencies_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResource>;
    fn CreateItem(this: &Self::This, bstrresourcename: &::windows_core::BSTR, bstrresourcetype: &::windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows_core::Result<ISClusResource>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddItem(this: &Self::This, presource: ::core::option::Option<&ISClusResource>) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResDependencies {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResDependencies {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::windows_core::from_raw_borrowed(&presource)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResDependencies_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResDependents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResource>;
    fn CreateItem(this: &Self::This, bstrresourcename: &::windows_core::BSTR, bstrresourcetype: &::windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows_core::Result<ISClusResource>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddItem(this: &Self::This, presource: ::core::option::Option<&ISClusResource>) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResDependents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResDependents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::windows_core::from_raw_borrowed(&presource)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResDependents_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrgroupname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<CLUSTER_GROUP_STATE>;
    fn OwnerNode(this: &Self::This) -> ::windows_core::Result<ISClusNode>;
    fn Resources(this: &Self::This) -> ::windows_core::Result<ISClusResGroupResources>;
    fn PreferredOwnerNodes(this: &Self::This) -> ::windows_core::Result<ISClusResGroupPreferredOwnerNodes>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Online(this: &Self::This, vartimeout: &super::super::System::Variant::VARIANT, varnode: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Move(this: &Self::This, vartimeout: &super::super::System::Variant::VARIANT, varnode: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Offline(this: &Self::This, vartimeout: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Cluster(this: &Self::This) -> ::windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrgroupname)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OwnerNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclustergroupresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Resources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclustergroupresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PreferredOwnerNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredOwnerNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Online<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartimeout: super::super::System::Variant::VARIANT, varnode: super::super::System::Variant::VARIANT, pvarpending: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Online(this, ::core::mem::transmute(&vartimeout), ::core::mem::transmute(&varnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartimeout: super::super::System::Variant::VARIANT, varnode: super::super::System::Variant::VARIANT, pvarpending: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Move(this, ::core::mem::transmute(&vartimeout), ::core::mem::transmute(&varnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Offline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartimeout: super::super::System::Variant::VARIANT, pvarpending: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Offline(this, ::core::mem::transmute(&vartimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cluster(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusResGroup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            OwnerNode: OwnerNode::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
            PreferredOwnerNodes: PreferredOwnerNodes::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Online: Online::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Offline: Offline::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroupPreferredOwnerNodes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNode>;
    fn InsertItem(this: &Self::This, pnode: ::core::option::Option<&ISClusNode>, nposition: i32) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Modified(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SaveChanges(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddItem(this: &Self::This, pnode: ::core::option::Option<&ISClusNode>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResGroupPreferredOwnerNodes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResGroupPreferredOwnerNodes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, nposition: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertItem(this, ::windows_core::from_raw_borrowed(&pnode), ::core::mem::transmute_copy(&nposition)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        unsafe extern "system" fn Modified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Modified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveChanges(this).into())
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::windows_core::from_raw_borrowed(&pnode)).into())
        }
        ISClusResGroupPreferredOwnerNodes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            InsertItem: InsertItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroupResources_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResource>;
    fn CreateItem(this: &Self::This, bstrresourcename: &::windows_core::BSTR, bstrresourcetype: &::windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows_core::Result<ISClusResource>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResGroupResources {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResGroupResources {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResGroupResources_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroups_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResGroup>;
    fn CreateItem(this: &Self::This, bstrresourcegroupname: &::windows_core::BSTR) -> ::windows_core::Result<ISClusResGroup>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResGroups {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResGroups {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcegroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppresourcegroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcegroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcegroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResGroups_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResPossibleOwnerNodes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNode>;
    fn AddItem(this: &Self::This, pnode: ::core::option::Option<&ISClusNode>) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Modified(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResPossibleOwnerNodes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResPossibleOwnerNodes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::windows_core::from_raw_borrowed(&pnode)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::core::mem::transmute(&varindex)).into())
        }
        unsafe extern "system" fn Modified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Modified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusResPossibleOwnerNodes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResType_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cluster(this: &Self::This) -> ::windows_core::Result<ISCluster>;
    fn Resources(this: &Self::This) -> ::windows_core::Result<ISClusResTypeResources>;
    fn PossibleOwnerNodes(this: &Self::This) -> ::windows_core::Result<ISClusResTypePossibleOwnerNodes>;
    fn AvailableDisks(this: &Self::This) -> ::windows_core::Result<ISClusDisks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Cluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cluster(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterrestyperesources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Resources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterrestyperesources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PossibleOwnerNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AvailableDisks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppavailabledisks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AvailableDisks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppavailabledisks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusResType_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, Impl, OFFSET>,
            AvailableDisks: AvailableDisks::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResTypePossibleOwnerNodes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResTypePossibleOwnerNodes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResTypePossibleOwnerNodes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusResTypePossibleOwnerNodes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResTypeResources_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResource>;
    fn CreateItem(this: &Self::This, bstrresourcename: &::windows_core::BSTR, bstrgroupname: &::windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows_core::Result<ISClusResource>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResTypeResources {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResTypeResources {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResTypeResources_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResTypes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResType>;
    fn CreateItem(this: &Self::This, bstrresourcetypename: &::windows_core::BSTR, bstrdisplayname: &::windows_core::BSTR, bstrresourcetypedll: &::windows_core::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> ::windows_core::Result<ISClusResType>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResTypes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResTypes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusrestype: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusrestype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcetypename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdisplayname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcetypedll: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcetypename), ::core::mem::transmute(&bstrdisplayname), ::core::mem::transmute(&bstrresourcetypedll), ::core::mem::transmute_copy(&dwlooksalivepollinterval), ::core::mem::transmute_copy(&dwisalivepollinterval)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResTypes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResource_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrresourcename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<CLUSTER_RESOURCE_STATE>;
    fn CoreFlag(this: &Self::This) -> ::windows_core::Result<CLUS_FLAGS>;
    fn BecomeQuorumResource(this: &Self::This, bstrdevicepath: &::windows_core::BSTR, lmaxlogsize: i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Fail(this: &Self::This) -> ::windows_core::Result<()>;
    fn Online(this: &Self::This, ntimeout: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Offline(this: &Self::This, ntimeout: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ChangeResourceGroup(this: &Self::This, presourcegroup: ::core::option::Option<&ISClusResGroup>) -> ::windows_core::Result<()>;
    fn AddResourceNode(this: &Self::This, pnode: ::core::option::Option<&ISClusNode>) -> ::windows_core::Result<()>;
    fn RemoveResourceNode(this: &Self::This, pnode: ::core::option::Option<&ISClusNode>) -> ::windows_core::Result<()>;
    fn CanResourceBeDependent(this: &Self::This, presource: ::core::option::Option<&ISClusResource>) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PossibleOwnerNodes(this: &Self::This) -> ::windows_core::Result<ISClusResPossibleOwnerNodes>;
    fn Dependencies(this: &Self::This) -> ::windows_core::Result<ISClusResDependencies>;
    fn Dependents(this: &Self::This) -> ::windows_core::Result<ISClusResDependents>;
    fn Group(this: &Self::This) -> ::windows_core::Result<ISClusResGroup>;
    fn OwnerNode(this: &Self::This) -> ::windows_core::Result<ISClusNode>;
    fn Cluster(this: &Self::This) -> ::windows_core::Result<ISCluster>;
    fn ClassInfo(this: &Self::This) -> ::windows_core::Result<CLUSTER_RESOURCE_CLASS>;
    fn Disk(this: &Self::This) -> ::windows_core::Result<ISClusDisk>;
    fn RegistryKeys(this: &Self::This) -> ::windows_core::Result<ISClusRegistryKeys>;
    fn CryptoKeys(this: &Self::This) -> ::windows_core::Result<ISClusCryptoKeys>;
    fn TypeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(this: &Self::This) -> ::windows_core::Result<ISClusResType>;
    fn MaintenanceMode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetMaintenanceMode(this: &Self::This, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrresourcename)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CoreFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CoreFlag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwcoreflag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BecomeQuorumResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdevicepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmaxlogsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BecomeQuorumResource(this, ::core::mem::transmute(&bstrdevicepath), ::core::mem::transmute_copy(&lmaxlogsize)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Fail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Fail(this).into())
        }
        unsafe extern "system" fn Online<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Online(this, ::core::mem::transmute_copy(&ntimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Offline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Offline(this, ::core::mem::transmute_copy(&ntimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChangeResourceGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcegroup: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeResourceGroup(this, ::windows_core::from_raw_borrowed(&presourcegroup)).into())
        }
        unsafe extern "system" fn AddResourceNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddResourceNode(this, ::windows_core::from_raw_borrowed(&pnode)).into())
        }
        unsafe extern "system" fn RemoveResourceNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveResourceNode(this, ::windows_core::from_raw_borrowed(&pnode)).into())
        }
        unsafe extern "system" fn CanResourceBeDependent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pvardependent: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanResourceBeDependent(this, ::windows_core::from_raw_borrowed(&presource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardependent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PossibleOwnerNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresdependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresdependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Dependents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresdependents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dependents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresdependents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Group(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OwnerNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cluster<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cluster(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClassInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prcclassinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Disk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Disk(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegistryKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppregistrykeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegistryKeys(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregistrykeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CryptoKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcryptokeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CryptoKeys(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcryptokeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtypename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TypeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtypename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresourcetype: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaintenanceMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmaintenancemode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaintenanceMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmaintenancemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaintenanceMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaintenanceMode(this, ::core::mem::transmute_copy(&bmaintenancemode)).into())
        }
        ISClusResource_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CoreFlag: CoreFlag::<Identity, Impl, OFFSET>,
            BecomeQuorumResource: BecomeQuorumResource::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Fail: Fail::<Identity, Impl, OFFSET>,
            Online: Online::<Identity, Impl, OFFSET>,
            Offline: Offline::<Identity, Impl, OFFSET>,
            ChangeResourceGroup: ChangeResourceGroup::<Identity, Impl, OFFSET>,
            AddResourceNode: AddResourceNode::<Identity, Impl, OFFSET>,
            RemoveResourceNode: RemoveResourceNode::<Identity, Impl, OFFSET>,
            CanResourceBeDependent: CanResourceBeDependent::<Identity, Impl, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            Dependents: Dependents::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            OwnerNode: OwnerNode::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
            ClassInfo: ClassInfo::<Identity, Impl, OFFSET>,
            Disk: Disk::<Identity, Impl, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, Impl, OFFSET>,
            CryptoKeys: CryptoKeys::<Identity, Impl, OFFSET>,
            TypeName: TypeName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            MaintenanceMode: MaintenanceMode::<Identity, Impl, OFFSET>,
            SetMaintenanceMode: SetMaintenanceMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResources_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISClusResource>;
    fn CreateItem(this: &Self::This, bstrresourcename: &::windows_core::BSTR, bstrresourcetype: &::windows_core::BSTR, bstrgroupname: &::windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows_core::Result<ISClusResource>;
    fn DeleteItem(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusResources {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusResources {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItem(this, ::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute(&varindex)).into())
        }
        ISClusResources_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusScsiAddress_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn PortNumber(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PathId(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn TargetId(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Lun(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusScsiAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusScsiAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PortNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarportnumber: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PortNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarportnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PathId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarpathid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpathid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TargetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvartargetid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvartargetid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Lun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarlun: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Lun(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarlun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusScsiAddress_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PortNumber: PortNumber::<Identity, Impl, OFFSET>,
            PathId: PathId::<Identity, Impl, OFFSET>,
            TargetId: TargetId::<Identity, Impl, OFFSET>,
            Lun: Lun::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusVersion_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MajorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn BuildNumber(this: &Self::This) -> ::windows_core::Result<i16>;
    fn VendorId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CSDVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClusterHighestVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ClusterLowestVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MixedVersion(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusVersion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusVersion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrclustername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrclustername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnmajorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnminorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BuildNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnbuildnumber: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BuildNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnbuildnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VendorId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VendorId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvendorid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CSDVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsdversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSDVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsdversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClusterHighestVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnclusterhighestversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClusterHighestVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnclusterhighestversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClusterLowestVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnclusterlowestversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClusterLowestVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnclusterlowestversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MixedVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmixedversion: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MixedVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmixedversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusVersion_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            BuildNumber: BuildNumber::<Identity, Impl, OFFSET>,
            VendorId: VendorId::<Identity, Impl, OFFSET>,
            CSDVersion: CSDVersion::<Identity, Impl, OFFSET>,
            ClusterHighestVersion: ClusterHighestVersion::<Identity, Impl, OFFSET>,
            ClusterLowestVersion: ClusterLowestVersion::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            MixedVersion: MixedVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISCluster_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn CommonROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(this: &Self::This) -> ::windows_core::Result<ISClusProperties>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn Open(this: &Self::This, bstrclustername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrclustername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This) -> ::windows_core::Result<ISClusVersion>;
    fn SetQuorumResource(this: &Self::This, pclusterresource: ::core::option::Option<&ISClusResource>) -> ::windows_core::Result<()>;
    fn QuorumResource(this: &Self::This) -> ::windows_core::Result<ISClusResource>;
    fn QuorumLogSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetQuorumLogSize(this: &Self::This, nlogsize: i32) -> ::windows_core::Result<()>;
    fn QuorumPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetQuorumPath(this: &Self::This, ppath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Nodes(this: &Self::This) -> ::windows_core::Result<ISClusNodes>;
    fn ResourceGroups(this: &Self::This) -> ::windows_core::Result<ISClusResGroups>;
    fn Resources(this: &Self::This) -> ::windows_core::Result<ISClusResources>;
    fn ResourceTypes(this: &Self::This) -> ::windows_core::Result<ISClusResTypes>;
    fn Networks(this: &Self::This) -> ::windows_core::Result<ISClusNetworks>;
    fn NetInterfaces(this: &Self::This) -> ::windows_core::Result<ISClusNetInterfaces>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISCluster {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCluster {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommonProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateROProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclustername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&bstrclustername)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclustername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrclustername)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusversion: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuorumResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclusterresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuorumResource(this, ::windows_core::from_raw_borrowed(&pclusterresource)).into())
        }
        unsafe extern "system" fn QuorumResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuorumResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclusterresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QuorumLogSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlogsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuorumLogSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlogsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuorumLogSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nlogsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuorumLogSize(this, ::core::mem::transmute_copy(&nlogsize)).into())
        }
        unsafe extern "system" fn QuorumPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuorumPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuorumPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuorumPath(this, ::core::mem::transmute(&ppath)).into())
        }
        unsafe extern "system" fn Nodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Nodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResourceGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterresourcegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResourceGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresourcegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Resources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResourceTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresourcetypes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResourceTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcetypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Networks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetworks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Networks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISCluster_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetQuorumResource: SetQuorumResource::<Identity, Impl, OFFSET>,
            QuorumResource: QuorumResource::<Identity, Impl, OFFSET>,
            QuorumLogSize: QuorumLogSize::<Identity, Impl, OFFSET>,
            SetQuorumLogSize: SetQuorumLogSize::<Identity, Impl, OFFSET>,
            QuorumPath: QuorumPath::<Identity, Impl, OFFSET>,
            SetQuorumPath: SetQuorumPath::<Identity, Impl, OFFSET>,
            Nodes: Nodes::<Identity, Impl, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
            ResourceTypes: ResourceTypes::<Identity, Impl, OFFSET>,
            Networks: Networks::<Identity, Impl, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusterNames_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISClusterNames {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISClusterNames {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrclustername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrclustername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdomainname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomainname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISClusterNames_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISDomainNames_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISDomainNames {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISDomainNames {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrdomainname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomainname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISDomainNames_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWCContextMenuCallback_Impl: ::windows_core::BaseImpl {
    fn AddExtensionMenuItem(this: &Self::This, lpszname: &::windows_core::BSTR, lpszstatusbartext: &::windows_core::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWCContextMenuCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCContextMenuCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWCContextMenuCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddExtensionMenuItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCContextMenuCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszstatusbartext: ::std::mem::MaybeUninit<::windows_core::BSTR>, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExtensionMenuItem(this, ::core::mem::transmute(&lpszname), ::core::mem::transmute(&lpszstatusbartext), ::core::mem::transmute_copy(&ncommandid), ::core::mem::transmute_copy(&nsubmenucommandid), ::core::mem::transmute_copy(&uflags)).into())
        }
        IWCContextMenuCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddExtensionMenuItem: AddExtensionMenuItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWCPropertySheetCallback_Impl: ::windows_core::BaseImpl {
    fn AddPropertySheetPage(this: &Self::This, hpage: *const i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWCPropertySheetCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCPropertySheetCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWCPropertySheetCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPropertySheetPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCPropertySheetCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertySheetPage(this, ::core::mem::transmute_copy(&hpage)).into())
        }
        IWCPropertySheetCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPropertySheetPage: AddPropertySheetPage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWCWizard97Callback_Impl: ::windows_core::BaseImpl {
    fn AddWizard97Page(this: &Self::This, hpage: *const i32) -> ::windows_core::Result<()>;
    fn EnableNext(this: &Self::This, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWCWizard97Callback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCWizard97Callback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWCWizard97Callback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddWizard97Page<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCWizard97Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddWizard97Page(this, ::core::mem::transmute_copy(&hpage)).into())
        }
        unsafe extern "system" fn EnableNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCWizard97Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableNext(this, ::core::mem::transmute_copy(&hpage), ::core::mem::transmute_copy(&benable)).into())
        }
        IWCWizard97Callback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddWizard97Page: AddWizard97Page::<Identity, Impl, OFFSET>,
            EnableNext: EnableNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWCWizardCallback_Impl: ::windows_core::BaseImpl {
    fn AddWizardPage(this: &Self::This, hpage: *const i32) -> ::windows_core::Result<()>;
    fn EnableNext(this: &Self::This, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWCWizardCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCWizardCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWCWizardCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddWizardPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCWizardCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddWizardPage(this, ::core::mem::transmute_copy(&hpage)).into())
        }
        unsafe extern "system" fn EnableNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWCWizardCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableNext(this, ::core::mem::transmute_copy(&hpage), ::core::mem::transmute_copy(&benable)).into())
        }
        IWCWizardCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddWizardPage: AddWizardPage::<Identity, Impl, OFFSET>,
            EnableNext: EnableNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWEExtendContextMenu_Impl: ::windows_core::BaseImpl {
    fn AddContextMenuItems(this: &Self::This, pidata: ::core::option::Option<&::windows_core::IUnknown>, picallback: ::core::option::Option<&IWCContextMenuCallback>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWEExtendContextMenu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendContextMenu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWEExtendContextMenu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddContextMenuItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddContextMenuItems(this, ::windows_core::from_raw_borrowed(&pidata), ::windows_core::from_raw_borrowed(&picallback)).into())
        }
        IWEExtendContextMenu_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddContextMenuItems: AddContextMenuItems::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWEExtendPropertySheet_Impl: ::windows_core::BaseImpl {
    fn CreatePropertySheetPages(this: &Self::This, pidata: ::core::option::Option<&::windows_core::IUnknown>, picallback: ::core::option::Option<&IWCPropertySheetCallback>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWEExtendPropertySheet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendPropertySheet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWEExtendPropertySheet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertySheetPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendPropertySheet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePropertySheetPages(this, ::windows_core::from_raw_borrowed(&pidata), ::windows_core::from_raw_borrowed(&picallback)).into())
        }
        IWEExtendPropertySheet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertySheetPages: CreatePropertySheetPages::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWEExtendWizard_Impl: ::windows_core::BaseImpl {
    fn CreateWizardPages(this: &Self::This, pidata: ::core::option::Option<&::windows_core::IUnknown>, picallback: ::core::option::Option<&IWCWizardCallback>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWEExtendWizard {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendWizard_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWEExtendWizard {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateWizardPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendWizard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateWizardPages(this, ::windows_core::from_raw_borrowed(&pidata), ::windows_core::from_raw_borrowed(&picallback)).into())
        }
        IWEExtendWizard_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateWizardPages: CreateWizardPages::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWEExtendWizard97_Impl: ::windows_core::BaseImpl {
    fn CreateWizard97Pages(this: &Self::This, pidata: ::core::option::Option<&::windows_core::IUnknown>, picallback: ::core::option::Option<&IWCWizard97Callback>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWEExtendWizard97 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendWizard97_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWEExtendWizard97 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateWizard97Pages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEExtendWizard97_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateWizard97Pages(this, ::windows_core::from_raw_borrowed(&pidata), ::windows_core::from_raw_borrowed(&picallback)).into())
        }
        IWEExtendWizard97_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateWizard97Pages: CreateWizard97Pages::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWEInvokeCommand_Impl: ::windows_core::BaseImpl {
    fn InvokeCommand(this: &Self::This, ncommandid: u32, pidata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWEInvokeCommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEInvokeCommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWEInvokeCommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InvokeCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWEInvokeCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncommandid: u32, pidata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeCommand(this, ::core::mem::transmute_copy(&ncommandid), ::windows_core::from_raw_borrowed(&pidata)).into())
        }
        IWEInvokeCommand_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, InvokeCommand: InvokeCommand::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
