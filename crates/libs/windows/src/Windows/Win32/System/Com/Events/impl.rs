pub trait IDontSupportEventSubscription_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDontSupportEventSubscription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDontSupportEventSubscription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDontSupportEventSubscription {
    const VTABLE: Self::Vtable = { IDontSupportEventSubscription_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumEventObject_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumEventObject>;
    fn Next(this: &Self::This, creqelem: u32, ppinterface: *mut ::core::option::Option<::windows_core::IUnknown>, cretelem: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cskipelem: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumEventObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumEventObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&creqelem), ::core::mem::transmute_copy(&ppinterface), ::core::mem::transmute_copy(&cretelem)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cskipelem)).into())
        }
        IEnumEventObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventClass_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn EventClassID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEventClassID(this: &Self::This, bstreventclassid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EventClassName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEventClassName(this: &Self::This, bstreventclassname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OwnerSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwnerSID(this: &Self::This, bstrownersid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FiringInterfaceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFiringInterfaceID(this: &Self::This, bstrfiringinterfaceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CustomConfigCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCustomConfigCLSID(this: &Self::This, bstrcustomconfigclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TypeLib(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTypeLib(this: &Self::This, bstrtypelib: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventClass {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventClass {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventClassID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventClassID(this, ::core::mem::transmute(&bstreventclassid)).into())
        }
        unsafe extern "system" fn EventClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventClassName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstreventclassname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventClassName(this, ::core::mem::transmute(&bstreventclassname)).into())
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrownersid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwnerSID(this, ::core::mem::transmute(&bstrownersid)).into())
        }
        unsafe extern "system" fn FiringInterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FiringInterfaceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfiringinterfaceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFiringInterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFiringInterfaceID(this, ::core::mem::transmute(&bstrfiringinterfaceid)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn CustomConfigCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CustomConfigCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcustomconfigclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCustomConfigCLSID(this, ::core::mem::transmute(&bstrcustomconfigclsid)).into())
        }
        unsafe extern "system" fn TypeLib<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TypeLib(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtypelib, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTypeLib<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtypelib: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypeLib(this, ::core::mem::transmute(&bstrtypelib)).into())
        }
        IEventClass_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventClassID: EventClassID::<Identity, Impl, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, Impl, OFFSET>,
            EventClassName: EventClassName::<Identity, Impl, OFFSET>,
            SetEventClassName: SetEventClassName::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            FiringInterfaceID: FiringInterfaceID::<Identity, Impl, OFFSET>,
            SetFiringInterfaceID: SetFiringInterfaceID::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            CustomConfigCLSID: CustomConfigCLSID::<Identity, Impl, OFFSET>,
            SetCustomConfigCLSID: SetCustomConfigCLSID::<Identity, Impl, OFFSET>,
            TypeLib: TypeLib::<Identity, Impl, OFFSET>,
            SetTypeLib: SetTypeLib::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventClass2_Impl: ::windows_core::BaseImpl + IEventClass_Impl {
    fn PublisherID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPublisherID(this: &Self::This, bstrpublisherid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MultiInterfacePublisherFilterCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMultiInterfacePublisherFilterCLSID(this: &Self::This, bstrpubfilclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AllowInprocActivation(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(this: &Self::This, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FireInParallel(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(this: &Self::This, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventClass2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEventClass);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventClass2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublisherid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPublisherID(this, ::core::mem::transmute(&bstrpublisherid)).into())
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MultiInterfacePublisherFilterCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpubfilclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultiInterfacePublisherFilterCLSID(this, ::core::mem::transmute(&bstrpubfilclsid)).into())
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInprocActivation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowinprocactivation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInprocActivation(this, ::core::mem::transmute_copy(&fallowinprocactivation)).into())
        }
        unsafe extern "system" fn FireInParallel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FireInParallel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffireinparallel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFireInParallel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFireInParallel(this, ::core::mem::transmute_copy(&ffireinparallel)).into())
        }
        IEventClass2_Vtbl {
            base__: <IEventClass as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            MultiInterfacePublisherFilterCLSID: MultiInterfacePublisherFilterCLSID::<Identity, Impl, OFFSET>,
            SetMultiInterfacePublisherFilterCLSID: SetMultiInterfacePublisherFilterCLSID::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            FireInParallel: FireInParallel::<Identity, Impl, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventControl_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn SetPublisherFilter(this: &Self::This, methodname: &::windows_core::BSTR, ppublisherfilter: ::core::option::Option<&IPublisherFilter>) -> ::windows_core::Result<()>;
    fn AllowInprocActivation(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(this: &Self::This, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSubscriptions(this: &Self::This, methodname: &::windows_core::BSTR, optionalcriteria: &::windows_core::BSTR, optionalerrorindex: *const i32) -> ::windows_core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(this: &Self::This, methodname: &::windows_core::BSTR, criteria: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPublisherFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppublisherfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPublisherFilter(this, ::core::mem::transmute(&methodname), ::windows_core::from_raw_borrowed(&ppublisherfilter)).into())
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInprocActivation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowinprocactivation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInprocActivation(this, ::core::mem::transmute_copy(&fallowinprocactivation)).into())
        }
        unsafe extern "system" fn GetSubscriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalcriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubscriptions(this, ::core::mem::transmute(&methodname), ::core::mem::transmute(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetDefaultQuery(this, ::core::mem::transmute(&methodname), ::core::mem::transmute(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEventControl_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPublisherFilter: SetPublisherFilter::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEventObjectChange_Impl: ::windows_core::BaseImpl {
    fn ChangedSubscription(this: &Self::This, changetype: EOC_ChangeType, bstrsubscriptionid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangedEventClass(this: &Self::This, changetype: EOC_ChangeType, bstreventclassid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangedPublisher(this: &Self::This, changetype: EOC_ChangeType, bstrpublisherid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEventObjectChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventObjectChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChangedSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangedSubscription(this, ::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstrsubscriptionid)).into())
        }
        unsafe extern "system" fn ChangedEventClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangedEventClass(this, ::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstreventclassid)).into())
        }
        unsafe extern "system" fn ChangedPublisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangedPublisher(this, ::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstrpublisherid)).into())
        }
        IEventObjectChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
            ChangedPublisher: ChangedPublisher::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEventObjectChange2_Impl: ::windows_core::BaseImpl {
    fn ChangedSubscription(this: &Self::This, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()>;
    fn ChangedEventClass(this: &Self::This, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEventObjectChange2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventObjectChange2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChangedSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangedSubscription(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn ChangedEventClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangedEventClass(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        IEventObjectChange2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventObjectCollection_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, objectid: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Variant::VARIANT>;
    fn NewEnum(this: &Self::This) -> ::windows_core::Result<IEnumEventObject>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, item: *const super::super::Variant::VARIANT, objectid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, objectid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventObjectCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventObjectCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *const super::super::Variant::VARIANT, objectid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&item), ::core::mem::transmute(&objectid)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&objectid)).into())
        }
        IEventObjectCollection_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            NewEnum: NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventProperty_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&propertyname)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&propertyvalue)).into())
        }
        IEventProperty_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventPublisher_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn PublisherID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPublisherID(this: &Self::This, bstrpublisherid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PublisherName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPublisherName(this: &Self::This, bstrpublishername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PublisherType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPublisherType(this: &Self::This, bstrpublishertype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OwnerSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwnerSID(this: &Self::This, bstrownersid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDefaultProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Variant::VARIANT>;
    fn PutDefaultProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemoveDefaultProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDefaultPropertyCollection(this: &Self::This) -> ::windows_core::Result<IEventObjectCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventPublisher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventPublisher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublisherid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPublisherID(this, ::core::mem::transmute(&bstrpublisherid)).into())
        }
        unsafe extern "system" fn PublisherName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublishername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPublisherName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpublishername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPublisherName(this, ::core::mem::transmute(&bstrpublishername)).into())
        }
        unsafe extern "system" fn PublisherType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublishertype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPublisherType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpublishertype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPublisherType(this, ::core::mem::transmute(&bstrpublishertype)).into())
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrownersid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwnerSID(this, ::core::mem::transmute(&bstrownersid)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn GetDefaultProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultProperty(this, ::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutDefaultProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutDefaultProperty(this, ::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into())
        }
        unsafe extern "system" fn RemoveDefaultProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveDefaultProperty(this, ::core::mem::transmute(&bstrpropertyname)).into())
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultPropertyCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEventPublisher_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            PublisherName: PublisherName::<Identity, Impl, OFFSET>,
            SetPublisherName: SetPublisherName::<Identity, Impl, OFFSET>,
            PublisherType: PublisherType::<Identity, Impl, OFFSET>,
            SetPublisherType: SetPublisherType::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDefaultProperty: GetDefaultProperty::<Identity, Impl, OFFSET>,
            PutDefaultProperty: PutDefaultProperty::<Identity, Impl, OFFSET>,
            RemoveDefaultProperty: RemoveDefaultProperty::<Identity, Impl, OFFSET>,
            GetDefaultPropertyCollection: GetDefaultPropertyCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventSubscription_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn SubscriptionID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubscriptionID(this: &Self::This, bstrsubscriptionid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SubscriptionName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubscriptionName(this: &Self::This, bstrsubscriptionname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PublisherID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPublisherID(this: &Self::This, bstrpublisherid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EventClassID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEventClassID(this: &Self::This, bstreventclassid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MethodName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMethodName(this: &Self::This, bstrmethodname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SubscriberCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubscriberCLSID(this: &Self::This, bstrsubscriberclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SubscriberInterface(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetSubscriberInterface(this: &Self::This, psubscriberinterface: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn PerUser(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetPerUser(this: &Self::This, fperuser: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OwnerSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwnerSID(this: &Self::This, bstrownersid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnabled(this: &Self::This, fenabled: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MachineName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMachineName(this: &Self::This, bstrmachinename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPublisherProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Variant::VARIANT>;
    fn PutPublisherProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemovePublisherProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPublisherPropertyCollection(this: &Self::This) -> ::windows_core::Result<IEventObjectCollection>;
    fn GetSubscriberProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Variant::VARIANT>;
    fn PutSubscriberProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemoveSubscriberProperty(this: &Self::This, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetSubscriberPropertyCollection(this: &Self::This) -> ::windows_core::Result<IEventObjectCollection>;
    fn InterfaceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInterfaceID(this: &Self::This, bstrinterfaceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventSubscription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventSubscription {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubscriptionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubscriptionID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubscriptionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubscriptionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscriptionID(this, ::core::mem::transmute(&bstrsubscriptionid)).into())
        }
        unsafe extern "system" fn SubscriptionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubscriptionName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubscriptionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubscriptionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscriptionName(this, ::core::mem::transmute(&bstrsubscriptionname)).into())
        }
        unsafe extern "system" fn PublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublisherid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPublisherID(this, ::core::mem::transmute(&bstrpublisherid)).into())
        }
        unsafe extern "system" fn EventClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventClassID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventClassID(this, ::core::mem::transmute(&bstreventclassid)).into())
        }
        unsafe extern "system" fn MethodName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MethodName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmethodname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMethodName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMethodName(this, ::core::mem::transmute(&bstrmethodname)).into())
        }
        unsafe extern "system" fn SubscriberCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubscriberCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubscriberclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubscriberCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscriberCLSID(this, ::core::mem::transmute(&bstrsubscriberclsid)).into())
        }
        unsafe extern "system" fn SubscriberInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubscriberInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubscriberinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubscriberInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscriberInterface(this, ::windows_core::from_raw_borrowed(&psubscriberinterface)).into())
        }
        unsafe extern "system" fn PerUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PerUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfperuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPerUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPerUser(this, ::core::mem::transmute_copy(&fperuser)).into())
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrownersid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwnerSID(this, ::core::mem::transmute(&bstrownersid)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn MachineName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MachineName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachinename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMachineName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmachinename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMachineName(this, ::core::mem::transmute(&bstrmachinename)).into())
        }
        unsafe extern "system" fn GetPublisherProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPublisherProperty(this, ::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutPublisherProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutPublisherProperty(this, ::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into())
        }
        unsafe extern "system" fn RemovePublisherProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePublisherProperty(this, ::core::mem::transmute(&bstrpropertyname)).into())
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPublisherPropertyCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubscriberProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubscriberProperty(this, ::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutSubscriberProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutSubscriberProperty(this, ::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into())
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSubscriberProperty(this, ::core::mem::transmute(&bstrpropertyname)).into())
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubscriberPropertyCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterfaceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinterfaceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterfaceID(this, ::core::mem::transmute(&bstrinterfaceid)).into())
        }
        IEventSubscription_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubscriptionID: SubscriptionID::<Identity, Impl, OFFSET>,
            SetSubscriptionID: SetSubscriptionID::<Identity, Impl, OFFSET>,
            SubscriptionName: SubscriptionName::<Identity, Impl, OFFSET>,
            SetSubscriptionName: SetSubscriptionName::<Identity, Impl, OFFSET>,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            EventClassID: EventClassID::<Identity, Impl, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, Impl, OFFSET>,
            MethodName: MethodName::<Identity, Impl, OFFSET>,
            SetMethodName: SetMethodName::<Identity, Impl, OFFSET>,
            SubscriberCLSID: SubscriberCLSID::<Identity, Impl, OFFSET>,
            SetSubscriberCLSID: SetSubscriberCLSID::<Identity, Impl, OFFSET>,
            SubscriberInterface: SubscriberInterface::<Identity, Impl, OFFSET>,
            SetSubscriberInterface: SetSubscriberInterface::<Identity, Impl, OFFSET>,
            PerUser: PerUser::<Identity, Impl, OFFSET>,
            SetPerUser: SetPerUser::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MachineName: MachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
            GetPublisherProperty: GetPublisherProperty::<Identity, Impl, OFFSET>,
            PutPublisherProperty: PutPublisherProperty::<Identity, Impl, OFFSET>,
            RemovePublisherProperty: RemovePublisherProperty::<Identity, Impl, OFFSET>,
            GetPublisherPropertyCollection: GetPublisherPropertyCollection::<Identity, Impl, OFFSET>,
            GetSubscriberProperty: GetSubscriberProperty::<Identity, Impl, OFFSET>,
            PutSubscriberProperty: PutSubscriberProperty::<Identity, Impl, OFFSET>,
            RemoveSubscriberProperty: RemoveSubscriberProperty::<Identity, Impl, OFFSET>,
            GetSubscriberPropertyCollection: GetSubscriberPropertyCollection::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            SetInterfaceID: SetInterfaceID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventSystem_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn Query(this: &Self::This, progid: &::windows_core::BSTR, querycriteria: &::windows_core::BSTR, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Store(this: &Self::This, progid: &::windows_core::BSTR, pinterface: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, progid: &::windows_core::BSTR, querycriteria: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn EventObjectChangeEventClassID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn QueryS(this: &Self::This, progid: &::windows_core::BSTR, querycriteria: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn RemoveS(this: &Self::This, progid: &::windows_core::BSTR, querycriteria: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventSystem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventSystem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Query(this, ::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria), ::core::mem::transmute_copy(&errorindex), ::core::mem::transmute_copy(&ppinterface)).into())
        }
        unsafe extern "system" fn Store<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Store(this, ::core::mem::transmute(&progid), ::windows_core::from_raw_borrowed(&pinterface)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Remove(this, ::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventObjectChangeEventClassID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryS(this, ::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveS(this, ::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)).into())
        }
        IEventSystem_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Query: Query::<Identity, Impl, OFFSET>,
            Store: Store::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Identity, Impl, OFFSET>,
            QueryS: QueryS::<Identity, Impl, OFFSET>,
            RemoveS: RemoveS::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFiringControl_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn FireSubscription(this: &Self::This, subscription: ::core::option::Option<&IEventSubscription>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFiringControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFiringControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFiringControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FireSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFiringControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subscription: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireSubscription(this, ::windows_core::from_raw_borrowed(&subscription)).into())
        }
        IFiringControl_Vtbl { base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FireSubscription: FireSubscription::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfaceEventControl_Impl: ::windows_core::BaseImpl {
    fn SetMultiInterfacePublisherFilter(this: &Self::This, classfilter: ::core::option::Option<&IMultiInterfacePublisherFilter>) -> ::windows_core::Result<()>;
    fn GetSubscriptions(this: &Self::This, eventiid: *const ::windows_core::GUID, bstrmethodname: &::windows_core::BSTR, optionalcriteria: &::windows_core::BSTR, optionalerrorindex: *const i32) -> ::windows_core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(this: &Self::This, eventiid: *const ::windows_core::GUID, bstrmethodname: &::windows_core::BSTR, bstrcriteria: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn AllowInprocActivation(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(this: &Self::This, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FireInParallel(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(this: &Self::This, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMultiInterfaceEventControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultiInterfaceEventControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultiInterfacePublisherFilter(this, ::windows_core::from_raw_borrowed(&classfilter)).into())
        }
        unsafe extern "system" fn GetSubscriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows_core::GUID, bstrmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalcriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubscriptions(this, ::core::mem::transmute_copy(&eventiid), ::core::mem::transmute(&bstrmethodname), ::core::mem::transmute(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows_core::GUID, bstrmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetDefaultQuery(this, ::core::mem::transmute_copy(&eventiid), ::core::mem::transmute(&bstrmethodname), ::core::mem::transmute(&bstrcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInprocActivation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowinprocactivation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInprocActivation(this, ::core::mem::transmute_copy(&fallowinprocactivation)).into())
        }
        unsafe extern "system" fn FireInParallel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FireInParallel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffireinparallel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFireInParallel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFireInParallel(this, ::core::mem::transmute_copy(&ffireinparallel)).into())
        }
        IMultiInterfaceEventControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMultiInterfacePublisherFilter: SetMultiInterfacePublisherFilter::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            FireInParallel: FireInParallel::<Identity, Impl, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMultiInterfacePublisherFilter_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, peic: ::core::option::Option<&IMultiInterfaceEventControl>) -> ::windows_core::Result<()>;
    fn PrepareToFire(this: &Self::This, iid: *const ::windows_core::GUID, methodname: &::windows_core::BSTR, firingcontrol: ::core::option::Option<&IFiringControl>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMultiInterfacePublisherFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultiInterfacePublisherFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&peic)).into())
        }
        unsafe extern "system" fn PrepareToFire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareToFire(this, ::core::mem::transmute_copy(&iid), ::core::mem::transmute(&methodname), ::windows_core::from_raw_borrowed(&firingcontrol)).into())
        }
        IMultiInterfacePublisherFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPublisherFilter_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, methodname: &::windows_core::BSTR, dispuserdefined: ::core::option::Option<&super::IDispatch>) -> ::windows_core::Result<()>;
    fn PrepareToFire(this: &Self::This, methodname: &::windows_core::BSTR, firingcontrol: ::core::option::Option<&IFiringControl>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPublisherFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPublisherFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, dispuserdefined: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&methodname), ::windows_core::from_raw_borrowed(&dispuserdefined)).into())
        }
        unsafe extern "system" fn PrepareToFire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareToFire(this, ::core::mem::transmute(&methodname), ::windows_core::from_raw_borrowed(&firingcontrol)).into())
        }
        IPublisherFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
