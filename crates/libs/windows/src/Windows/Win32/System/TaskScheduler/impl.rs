#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAction_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Id(this: &Self::This, pid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetId(this: &Self::This, id: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Id(this, ::core::mem::transmute_copy(&pid)).into())
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Type(this, ::core::mem::transmute_copy(&ptype)).into())
        }
        IAction_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActionCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This, pcount: *mut i32) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IAction>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn XmlText(this: &Self::This, ptext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetXmlText(this: &Self::This, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Create(this: &Self::This, r#type: TASK_ACTION_TYPE) -> ::windows_core::Result<IAction>;
    fn Remove(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn Context(this: &Self::This, pcontext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetContext(this: &Self::This, context: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IActionCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActionCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn XmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::XmlText(this, ::core::mem::transmute_copy(&ptext)).into())
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXmlText(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&index)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn Context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Context(this, ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn SetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContext(this, ::core::mem::transmute(&context)).into())
        }
        IActionCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBootTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn Delay(this: &Self::This, pdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDelay(this: &Self::This, delay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBootTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBootTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delay(this, ::core::mem::transmute_copy(&pdelay)).into())
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelay(this, ::core::mem::transmute(&delay)).into())
        }
        IBootTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComHandlerAction_Impl: ::windows_core::BaseImpl + IAction_Impl {
    fn ClassId(this: &Self::This, pclsid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetClassId(this: &Self::This, clsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Data(this: &Self::This, pdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetData(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IComHandlerAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComHandlerAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClassId(this, ::core::mem::transmute_copy(&pclsid)).into())
        }
        unsafe extern "system" fn SetClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassId(this, ::core::mem::transmute(&clsid)).into())
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Data(this, ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&data)).into())
        }
        IComHandlerAction_Vtbl {
            base__: <IAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClassId: ClassId::<Identity, Impl, OFFSET>,
            SetClassId: SetClassId::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDailyTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn DaysInterval(this: &Self::This, pdays: *mut i16) -> ::windows_core::Result<()>;
    fn SetDaysInterval(this: &Self::This, days: i16) -> ::windows_core::Result<()>;
    fn RandomDelay(this: &Self::This, prandomdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRandomDelay(this: &Self::This, randomdelay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDailyTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDailyTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DaysInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DaysInterval(this, ::core::mem::transmute_copy(&pdays)).into())
        }
        unsafe extern "system" fn SetDaysInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysInterval(this, ::core::mem::transmute_copy(&days)).into())
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RandomDelay(this, ::core::mem::transmute_copy(&prandomdelay)).into())
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, randomdelay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRandomDelay(this, ::core::mem::transmute(&randomdelay)).into())
        }
        IDailyTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DaysInterval: DaysInterval::<Identity, Impl, OFFSET>,
            SetDaysInterval: SetDaysInterval::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEmailAction_Impl: ::windows_core::BaseImpl + IAction_Impl {
    fn Server(this: &Self::This, pserver: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetServer(this: &Self::This, server: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Subject(this: &Self::This, psubject: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSubject(this: &Self::This, subject: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn To(this: &Self::This, pto: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetTo(this: &Self::This, to: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Cc(this: &Self::This, pcc: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetCc(this: &Self::This, cc: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Bcc(this: &Self::This, pbcc: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetBcc(this: &Self::This, bcc: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReplyTo(this: &Self::This, preplyto: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetReplyTo(this: &Self::This, replyto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn From(this: &Self::This, pfrom: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetFrom(this: &Self::This, from: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HeaderFields(this: &Self::This) -> ::windows_core::Result<ITaskNamedValueCollection>;
    fn SetHeaderFields(this: &Self::This, pheaderfields: ::core::option::Option<&ITaskNamedValueCollection>) -> ::windows_core::Result<()>;
    fn Body(this: &Self::This, pbody: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetBody(this: &Self::This, body: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Attachments(this: &Self::This, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn SetAttachments(this: &Self::This, pattachements: *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEmailAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEmailAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Server<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Server(this, ::core::mem::transmute_copy(&pserver)).into())
        }
        unsafe extern "system" fn SetServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, server: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServer(this, ::core::mem::transmute(&server)).into())
        }
        unsafe extern "system" fn Subject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Subject(this, ::core::mem::transmute_copy(&psubject)).into())
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subject: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubject(this, ::core::mem::transmute(&subject)).into())
        }
        unsafe extern "system" fn To<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::To(this, ::core::mem::transmute_copy(&pto)).into())
        }
        unsafe extern "system" fn SetTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, to: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTo(this, ::core::mem::transmute(&to)).into())
        }
        unsafe extern "system" fn Cc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cc(this, ::core::mem::transmute_copy(&pcc)).into())
        }
        unsafe extern "system" fn SetCc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cc: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCc(this, ::core::mem::transmute(&cc)).into())
        }
        unsafe extern "system" fn Bcc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bcc(this, ::core::mem::transmute_copy(&pbcc)).into())
        }
        unsafe extern "system" fn SetBcc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcc: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBcc(this, ::core::mem::transmute(&bcc)).into())
        }
        unsafe extern "system" fn ReplyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preplyto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplyTo(this, ::core::mem::transmute_copy(&preplyto)).into())
        }
        unsafe extern "system" fn SetReplyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, replyto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReplyTo(this, ::core::mem::transmute(&replyto)).into())
        }
        unsafe extern "system" fn From<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrom: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::From(this, ::core::mem::transmute_copy(&pfrom)).into())
        }
        unsafe extern "system" fn SetFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, from: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrom(this, ::core::mem::transmute(&from)).into())
        }
        unsafe extern "system" fn HeaderFields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppheaderfields: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HeaderFields(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppheaderfields, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHeaderFields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheaderfields: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHeaderFields(this, ::windows_core::from_raw_borrowed(&pheaderfields)).into())
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Body(this, ::core::mem::transmute_copy(&pbody)).into())
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, body: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&body)).into())
        }
        unsafe extern "system" fn Attachments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attachments(this, ::core::mem::transmute_copy(&pattachements)).into())
        }
        unsafe extern "system" fn SetAttachments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttachments(this, ::core::mem::transmute_copy(&pattachements)).into())
        }
        IEmailAction_Vtbl {
            base__: <IAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Server: Server::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            To: To::<Identity, Impl, OFFSET>,
            SetTo: SetTo::<Identity, Impl, OFFSET>,
            Cc: Cc::<Identity, Impl, OFFSET>,
            SetCc: SetCc::<Identity, Impl, OFFSET>,
            Bcc: Bcc::<Identity, Impl, OFFSET>,
            SetBcc: SetBcc::<Identity, Impl, OFFSET>,
            ReplyTo: ReplyTo::<Identity, Impl, OFFSET>,
            SetReplyTo: SetReplyTo::<Identity, Impl, OFFSET>,
            From: From::<Identity, Impl, OFFSET>,
            SetFrom: SetFrom::<Identity, Impl, OFFSET>,
            HeaderFields: HeaderFields::<Identity, Impl, OFFSET>,
            SetHeaderFields: SetHeaderFields::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Attachments: Attachments::<Identity, Impl, OFFSET>,
            SetAttachments: SetAttachments::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumWorkItems_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgpwsznames: *mut *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWorkItems>;
}
impl ::windows_core::Iids for IEnumWorkItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWorkItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgpwsznames), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumworkitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumWorkItems_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn Subscription(this: &Self::This, pquery: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSubscription(this: &Self::This, query: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Delay(this: &Self::This, pdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDelay(this: &Self::This, delay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ValueQueries(this: &Self::This) -> ::windows_core::Result<ITaskNamedValueCollection>;
    fn SetValueQueries(this: &Self::This, pnamedxpaths: ::core::option::Option<&ITaskNamedValueCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Subscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquery: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Subscription(this, ::core::mem::transmute_copy(&pquery)).into())
        }
        unsafe extern "system" fn SetSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscription(this, ::core::mem::transmute(&query)).into())
        }
        unsafe extern "system" fn Delay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delay(this, ::core::mem::transmute_copy(&pdelay)).into())
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelay(this, ::core::mem::transmute(&delay)).into())
        }
        unsafe extern "system" fn ValueQueries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueQueries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamedxpaths, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValueQueries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamedxpaths: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueQueries(this, ::windows_core::from_raw_borrowed(&pnamedxpaths)).into())
        }
        IEventTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Subscription: Subscription::<Identity, Impl, OFFSET>,
            SetSubscription: SetSubscription::<Identity, Impl, OFFSET>,
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
            ValueQueries: ValueQueries::<Identity, Impl, OFFSET>,
            SetValueQueries: SetValueQueries::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExecAction_Impl: ::windows_core::BaseImpl + IAction_Impl {
    fn Path(this: &Self::This, ppath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetPath(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Arguments(this: &Self::This, pargument: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetArguments(this: &Self::This, argument: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn WorkingDirectory(this: &Self::This, pworkingdirectory: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetWorkingDirectory(this: &Self::This, workingdirectory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IExecAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExecAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Path(this, ::core::mem::transmute_copy(&ppath)).into())
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn Arguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pargument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Arguments(this, ::core::mem::transmute_copy(&pargument)).into())
        }
        unsafe extern "system" fn SetArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, argument: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetArguments(this, ::core::mem::transmute(&argument)).into())
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WorkingDirectory(this, ::core::mem::transmute_copy(&pworkingdirectory)).into())
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, workingdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkingDirectory(this, ::core::mem::transmute(&workingdirectory)).into())
        }
        IExecAction_Vtbl {
            base__: <IAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            Arguments: Arguments::<Identity, Impl, OFFSET>,
            SetArguments: SetArguments::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExecAction2_Impl: ::windows_core::BaseImpl + IExecAction_Impl {
    fn HideAppWindow(this: &Self::This, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetHideAppWindow(this: &Self::This, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IExecAction2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IExecAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExecAction2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HideAppWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HideAppWindow(this, ::core::mem::transmute_copy(&phideappwindow)).into())
        }
        unsafe extern "system" fn SetHideAppWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExecAction2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHideAppWindow(this, ::core::mem::transmute_copy(&hideappwindow)).into())
        }
        IExecAction2_Vtbl {
            base__: <IExecAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HideAppWindow: HideAppWindow::<Identity, Impl, OFFSET>,
            SetHideAppWindow: SetHideAppWindow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIdleSettings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn IdleDuration(this: &Self::This, pdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetIdleDuration(this: &Self::This, delay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn WaitTimeout(this: &Self::This, ptimeout: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetWaitTimeout(this: &Self::This, timeout: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StopOnIdleEnd(this: &Self::This, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetStopOnIdleEnd(this: &Self::This, stop: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RestartOnIdle(this: &Self::This, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRestartOnIdle(this: &Self::This, restart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIdleSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdleSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IdleDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IdleDuration(this, ::core::mem::transmute_copy(&pdelay)).into())
        }
        unsafe extern "system" fn SetIdleDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIdleDuration(this, ::core::mem::transmute(&delay)).into())
        }
        unsafe extern "system" fn WaitTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimeout: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitTimeout(this, ::core::mem::transmute_copy(&ptimeout)).into())
        }
        unsafe extern "system" fn SetWaitTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWaitTimeout(this, ::core::mem::transmute(&timeout)).into())
        }
        unsafe extern "system" fn StopOnIdleEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopOnIdleEnd(this, ::core::mem::transmute_copy(&pstop)).into())
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStopOnIdleEnd(this, ::core::mem::transmute_copy(&stop)).into())
        }
        unsafe extern "system" fn RestartOnIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestartOnIdle(this, ::core::mem::transmute_copy(&prestart)).into())
        }
        unsafe extern "system" fn SetRestartOnIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRestartOnIdle(this, ::core::mem::transmute_copy(&restart)).into())
        }
        IIdleSettings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IdleDuration: IdleDuration::<Identity, Impl, OFFSET>,
            SetIdleDuration: SetIdleDuration::<Identity, Impl, OFFSET>,
            WaitTimeout: WaitTimeout::<Identity, Impl, OFFSET>,
            SetWaitTimeout: SetWaitTimeout::<Identity, Impl, OFFSET>,
            StopOnIdleEnd: StopOnIdleEnd::<Identity, Impl, OFFSET>,
            SetStopOnIdleEnd: SetStopOnIdleEnd::<Identity, Impl, OFFSET>,
            RestartOnIdle: RestartOnIdle::<Identity, Impl, OFFSET>,
            SetRestartOnIdle: SetRestartOnIdle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIdleTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIdleTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIdleTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIdleTrigger {
    const VTABLE: Self::Vtable = { IIdleTrigger_Vtbl { base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILogonTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn Delay(this: &Self::This, pdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDelay(this: &Self::This, delay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UserId(this: &Self::This, puser: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetUserId(this: &Self::This, user: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ILogonTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILogonTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delay(this, ::core::mem::transmute_copy(&pdelay)).into())
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelay(this, ::core::mem::transmute(&delay)).into())
        }
        unsafe extern "system" fn UserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UserId(this, ::core::mem::transmute_copy(&puser)).into())
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserId(this, ::core::mem::transmute(&user)).into())
        }
        ILogonTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
            UserId: UserId::<Identity, Impl, OFFSET>,
            SetUserId: SetUserId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMaintenanceSettings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SetPeriod(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Period(this: &Self::This, target: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDeadline(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Deadline(this: &Self::This, target: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetExclusive(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Exclusive(this: &Self::This, target: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMaintenanceSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMaintenanceSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPeriod(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Period<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Period(this, ::core::mem::transmute_copy(&target)).into())
        }
        unsafe extern "system" fn SetDeadline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeadline(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Deadline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deadline(this, ::core::mem::transmute_copy(&target)).into())
        }
        unsafe extern "system" fn SetExclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExclusive(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Exclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Exclusive(this, ::core::mem::transmute_copy(&target)).into())
        }
        IMaintenanceSettings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPeriod: SetPeriod::<Identity, Impl, OFFSET>,
            Period: Period::<Identity, Impl, OFFSET>,
            SetDeadline: SetDeadline::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
            SetExclusive: SetExclusive::<Identity, Impl, OFFSET>,
            Exclusive: Exclusive::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMonthlyDOWTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn DaysOfWeek(this: &Self::This, pdays: *mut i16) -> ::windows_core::Result<()>;
    fn SetDaysOfWeek(this: &Self::This, days: i16) -> ::windows_core::Result<()>;
    fn WeeksOfMonth(this: &Self::This, pweeks: *mut i16) -> ::windows_core::Result<()>;
    fn SetWeeksOfMonth(this: &Self::This, weeks: i16) -> ::windows_core::Result<()>;
    fn MonthsOfYear(this: &Self::This, pmonths: *mut i16) -> ::windows_core::Result<()>;
    fn SetMonthsOfYear(this: &Self::This, months: i16) -> ::windows_core::Result<()>;
    fn RunOnLastWeekOfMonth(this: &Self::This, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRunOnLastWeekOfMonth(this: &Self::This, lastweek: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RandomDelay(this: &Self::This, prandomdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRandomDelay(this: &Self::This, randomdelay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMonthlyDOWTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMonthlyDOWTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DaysOfWeek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DaysOfWeek(this, ::core::mem::transmute_copy(&pdays)).into())
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysOfWeek(this, ::core::mem::transmute_copy(&days)).into())
        }
        unsafe extern "system" fn WeeksOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WeeksOfMonth(this, ::core::mem::transmute_copy(&pweeks)).into())
        }
        unsafe extern "system" fn SetWeeksOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWeeksOfMonth(this, ::core::mem::transmute_copy(&weeks)).into())
        }
        unsafe extern "system" fn MonthsOfYear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MonthsOfYear(this, ::core::mem::transmute_copy(&pmonths)).into())
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMonthsOfYear(this, ::core::mem::transmute_copy(&months)).into())
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunOnLastWeekOfMonth(this, ::core::mem::transmute_copy(&plastweek)).into())
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastweek: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRunOnLastWeekOfMonth(this, ::core::mem::transmute_copy(&lastweek)).into())
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RandomDelay(this, ::core::mem::transmute_copy(&prandomdelay)).into())
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, randomdelay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRandomDelay(this, ::core::mem::transmute(&randomdelay)).into())
        }
        IMonthlyDOWTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DaysOfWeek: DaysOfWeek::<Identity, Impl, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, Impl, OFFSET>,
            WeeksOfMonth: WeeksOfMonth::<Identity, Impl, OFFSET>,
            SetWeeksOfMonth: SetWeeksOfMonth::<Identity, Impl, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, Impl, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, Impl, OFFSET>,
            RunOnLastWeekOfMonth: RunOnLastWeekOfMonth::<Identity, Impl, OFFSET>,
            SetRunOnLastWeekOfMonth: SetRunOnLastWeekOfMonth::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMonthlyTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn DaysOfMonth(this: &Self::This, pdays: *mut i32) -> ::windows_core::Result<()>;
    fn SetDaysOfMonth(this: &Self::This, days: i32) -> ::windows_core::Result<()>;
    fn MonthsOfYear(this: &Self::This, pmonths: *mut i16) -> ::windows_core::Result<()>;
    fn SetMonthsOfYear(this: &Self::This, months: i16) -> ::windows_core::Result<()>;
    fn RunOnLastDayOfMonth(this: &Self::This, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRunOnLastDayOfMonth(this: &Self::This, lastday: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RandomDelay(this: &Self::This, prandomdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRandomDelay(this: &Self::This, randomdelay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMonthlyTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMonthlyTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DaysOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DaysOfMonth(this, ::core::mem::transmute_copy(&pdays)).into())
        }
        unsafe extern "system" fn SetDaysOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysOfMonth(this, ::core::mem::transmute_copy(&days)).into())
        }
        unsafe extern "system" fn MonthsOfYear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MonthsOfYear(this, ::core::mem::transmute_copy(&pmonths)).into())
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMonthsOfYear(this, ::core::mem::transmute_copy(&months)).into())
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunOnLastDayOfMonth(this, ::core::mem::transmute_copy(&plastday)).into())
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastday: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRunOnLastDayOfMonth(this, ::core::mem::transmute_copy(&lastday)).into())
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RandomDelay(this, ::core::mem::transmute_copy(&prandomdelay)).into())
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, randomdelay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRandomDelay(this, ::core::mem::transmute(&randomdelay)).into())
        }
        IMonthlyTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DaysOfMonth: DaysOfMonth::<Identity, Impl, OFFSET>,
            SetDaysOfMonth: SetDaysOfMonth::<Identity, Impl, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, Impl, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, Impl, OFFSET>,
            RunOnLastDayOfMonth: RunOnLastDayOfMonth::<Identity, Impl, OFFSET>,
            SetRunOnLastDayOfMonth: SetRunOnLastDayOfMonth::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkSettings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This, pname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This, pid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetId(this: &Self::This, id: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetworkSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Name(this, ::core::mem::transmute_copy(&pname)).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Id(this, ::core::mem::transmute_copy(&pid)).into())
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id)).into())
        }
        INetworkSettings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrincipal_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Id(this: &Self::This, pid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetId(this: &Self::This, id: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisplayName(this: &Self::This, pname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDisplayName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UserId(this: &Self::This, puser: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetUserId(this: &Self::This, user: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LogonType(this: &Self::This, plogon: *mut TASK_LOGON_TYPE) -> ::windows_core::Result<()>;
    fn SetLogonType(this: &Self::This, logon: TASK_LOGON_TYPE) -> ::windows_core::Result<()>;
    fn GroupId(this: &Self::This, pgroup: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetGroupId(this: &Self::This, group: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RunLevel(this: &Self::This, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows_core::Result<()>;
    fn SetRunLevel(this: &Self::This, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrincipal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrincipal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Id(this, ::core::mem::transmute_copy(&pid)).into())
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id)).into())
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayName(this, ::core::mem::transmute_copy(&pname)).into())
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn UserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UserId(this, ::core::mem::transmute_copy(&puser)).into())
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserId(this, ::core::mem::transmute(&user)).into())
        }
        unsafe extern "system" fn LogonType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogonType(this, ::core::mem::transmute_copy(&plogon)).into())
        }
        unsafe extern "system" fn SetLogonType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogonType(this, ::core::mem::transmute_copy(&logon)).into())
        }
        unsafe extern "system" fn GroupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroup: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GroupId(this, ::core::mem::transmute_copy(&pgroup)).into())
        }
        unsafe extern "system" fn SetGroupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, group: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupId(this, ::core::mem::transmute(&group)).into())
        }
        unsafe extern "system" fn RunLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunLevel(this, ::core::mem::transmute_copy(&prunlevel)).into())
        }
        unsafe extern "system" fn SetRunLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRunLevel(this, ::core::mem::transmute_copy(&runlevel)).into())
        }
        IPrincipal_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            UserId: UserId::<Identity, Impl, OFFSET>,
            SetUserId: SetUserId::<Identity, Impl, OFFSET>,
            LogonType: LogonType::<Identity, Impl, OFFSET>,
            SetLogonType: SetLogonType::<Identity, Impl, OFFSET>,
            GroupId: GroupId::<Identity, Impl, OFFSET>,
            SetGroupId: SetGroupId::<Identity, Impl, OFFSET>,
            RunLevel: RunLevel::<Identity, Impl, OFFSET>,
            SetRunLevel: SetRunLevel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrincipal2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ProcessTokenSidType(this: &Self::This, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::Result<()>;
    fn SetProcessTokenSidType(this: &Self::This, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::Result<()>;
    fn RequiredPrivilegeCount(this: &Self::This, pcount: *mut i32) -> ::windows_core::Result<()>;
    fn get_RequiredPrivilege(this: &Self::This, index: i32, pprivilege: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddRequiredPrivilege(this: &Self::This, privilege: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrincipal2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrincipal2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessTokenSidType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessTokenSidType(this, ::core::mem::transmute_copy(&pprocesstokensidtype)).into())
        }
        unsafe extern "system" fn SetProcessTokenSidType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProcessTokenSidType(this, ::core::mem::transmute_copy(&processtokensidtype)).into())
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequiredPrivilegeCount(this, ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn get_RequiredPrivilege<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_RequiredPrivilege(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pprivilege)).into())
        }
        unsafe extern "system" fn AddRequiredPrivilege<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, privilege: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRequiredPrivilege(this, ::core::mem::transmute(&privilege)).into())
        }
        IPrincipal2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProcessTokenSidType: ProcessTokenSidType::<Identity, Impl, OFFSET>,
            SetProcessTokenSidType: SetProcessTokenSidType::<Identity, Impl, OFFSET>,
            RequiredPrivilegeCount: RequiredPrivilegeCount::<Identity, Impl, OFFSET>,
            get_RequiredPrivilege: get_RequiredPrivilege::<Identity, Impl, OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait IProvideTaskPage_Impl: ::windows_core::BaseImpl {
    fn GetPage(this: &Self::This, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::UI::Controls::HPROPSHEETPAGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::windows_core::Iids for IProvideTaskPage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideTaskPage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideTaskPage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideTaskPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPage(this, ::core::mem::transmute_copy(&tptype), ::core::mem::transmute_copy(&fpersistchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideTaskPage_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetPage: GetPage::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegisteredTask_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn State(this: &Self::This) -> ::windows_core::Result<TASK_STATE>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Run(this: &Self::This, params: &super::Variant::VARIANT) -> ::windows_core::Result<IRunningTask>;
    fn RunEx(this: &Self::This, params: &super::Variant::VARIANT, flags: i32, sessionid: i32, user: &::windows_core::BSTR) -> ::windows_core::Result<IRunningTask>;
    fn GetInstances(this: &Self::This, flags: i32) -> ::windows_core::Result<IRunningTaskCollection>;
    fn LastRunTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn LastTaskResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfMissedRuns(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NextRunTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Definition(this: &Self::This) -> ::windows_core::Result<ITaskDefinition>;
    fn Xml(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSecurityDescriptor(this: &Self::This, securityinformation: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSecurityDescriptor(this: &Self::This, sddl: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This, flags: i32) -> ::windows_core::Result<()>;
    fn GetRunTimes(this: &Self::This, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRegisteredTask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRegisteredTask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, params: super::Variant::VARIANT, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Run(this, ::core::mem::transmute(&params)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RunEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, params: super::Variant::VARIANT, flags: i32, sessionid: i32, user: ::std::mem::MaybeUninit<::windows_core::BSTR>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunEx(this, ::core::mem::transmute(&params), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute(&user)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstances(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtasks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastRunTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastRunTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plastruntime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastTaskResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastTaskResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plasttaskresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfMissedRuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfMissedRuns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumberofmissedruns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextRunTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextRunTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnextruntime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Definition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Definition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Xml(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityDescriptor(this, ::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psddl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sddl: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::core::mem::transmute(&sddl), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetRunTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRunTimes(this, ::core::mem::transmute_copy(&pststart), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&pruntimes)).into())
        }
        IRegisteredTask_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            RunEx: RunEx::<Identity, Impl, OFFSET>,
            GetInstances: GetInstances::<Identity, Impl, OFFSET>,
            LastRunTime: LastRunTime::<Identity, Impl, OFFSET>,
            LastTaskResult: LastTaskResult::<Identity, Impl, OFFSET>,
            NumberOfMissedRuns: NumberOfMissedRuns::<Identity, Impl, OFFSET>,
            NextRunTime: NextRunTime::<Identity, Impl, OFFSET>,
            Definition: Definition::<Identity, Impl, OFFSET>,
            Xml: Xml::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegisteredTaskCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<IRegisteredTask>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRegisteredTaskCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRegisteredTaskCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, ppregisteredtask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregisteredtask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRegisteredTaskCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegistrationInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Description(this: &Self::This, pdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDescription(this: &Self::This, description: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Author(this: &Self::This, pauthor: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetAuthor(this: &Self::This, author: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This, pversion: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetVersion(this: &Self::This, version: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Date(this: &Self::This, pdate: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDate(this: &Self::This, date: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Documentation(this: &Self::This, pdocumentation: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDocumentation(this: &Self::This, documentation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn XmlText(this: &Self::This, ptext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetXmlText(this: &Self::This, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn URI(this: &Self::This, puri: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetURI(this: &Self::This, uri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SecurityDescriptor(this: &Self::This, psddl: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetSecurityDescriptor(this: &Self::This, sddl: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Source(this: &Self::This, psource: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSource(this: &Self::This, source: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRegistrationInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRegistrationInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Description(this, ::core::mem::transmute_copy(&pdescription)).into())
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&description)).into())
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Author(this, ::core::mem::transmute_copy(&pauthor)).into())
        }
        unsafe extern "system" fn SetAuthor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, author: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthor(this, ::core::mem::transmute(&author)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Version(this, ::core::mem::transmute_copy(&pversion)).into())
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute(&version)).into())
        }
        unsafe extern "system" fn Date<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Date(this, ::core::mem::transmute_copy(&pdate)).into())
        }
        unsafe extern "system" fn SetDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, date: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDate(this, ::core::mem::transmute(&date)).into())
        }
        unsafe extern "system" fn Documentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdocumentation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Documentation(this, ::core::mem::transmute_copy(&pdocumentation)).into())
        }
        unsafe extern "system" fn SetDocumentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentation(this, ::core::mem::transmute(&documentation)).into())
        }
        unsafe extern "system" fn XmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::XmlText(this, ::core::mem::transmute_copy(&ptext)).into())
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXmlText(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn URI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::URI(this, ::core::mem::transmute_copy(&puri)).into())
        }
        unsafe extern "system" fn SetURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetURI(this, ::core::mem::transmute(&uri)).into())
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psddl: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SecurityDescriptor(this, ::core::mem::transmute_copy(&psddl)).into())
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sddl: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::core::mem::transmute(&sddl)).into())
        }
        unsafe extern "system" fn Source<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Source(this, ::core::mem::transmute_copy(&psource)).into())
        }
        unsafe extern "system" fn SetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSource(this, ::core::mem::transmute(&source)).into())
        }
        IRegistrationInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            SetAuthor: SetAuthor::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            Date: Date::<Identity, Impl, OFFSET>,
            SetDate: SetDate::<Identity, Impl, OFFSET>,
            Documentation: Documentation::<Identity, Impl, OFFSET>,
            SetDocumentation: SetDocumentation::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
            URI: URI::<Identity, Impl, OFFSET>,
            SetURI: SetURI::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            Source: Source::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegistrationTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn Delay(this: &Self::This, pdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDelay(this: &Self::This, delay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRegistrationTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRegistrationTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delay(this, ::core::mem::transmute_copy(&pdelay)).into())
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegistrationTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelay(this, ::core::mem::transmute(&delay)).into())
        }
        IRegistrationTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRepetitionPattern_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Interval(this: &Self::This, pinterval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetInterval(this: &Self::This, interval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Duration(this: &Self::This, pduration: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDuration(this: &Self::This, duration: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StopAtDurationEnd(this: &Self::This, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetStopAtDurationEnd(this: &Self::This, stop: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRepetitionPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRepetitionPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Interval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinterval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Interval(this, ::core::mem::transmute_copy(&pinterval)).into())
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterval(this, ::core::mem::transmute(&interval)).into())
        }
        unsafe extern "system" fn Duration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pduration: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Duration(this, ::core::mem::transmute_copy(&pduration)).into())
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuration(this, ::core::mem::transmute(&duration)).into())
        }
        unsafe extern "system" fn StopAtDurationEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopAtDurationEnd(this, ::core::mem::transmute_copy(&pstop)).into())
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStopAtDurationEnd(this, ::core::mem::transmute_copy(&stop)).into())
        }
        IRepetitionPattern_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            StopAtDurationEnd: StopAtDurationEnd::<Identity, Impl, OFFSET>,
            SetStopAtDurationEnd: SetStopAtDurationEnd::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRunningTask_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InstanceGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn State(this: &Self::This) -> ::windows_core::Result<TASK_STATE>;
    fn CurrentAction(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnginePID(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRunningTask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRunningTask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstanceGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstanceGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn EnginePID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnginePID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRunningTask_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            InstanceGuid: InstanceGuid::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            EnginePID: EnginePID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRunningTaskCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<IRunningTask>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRunningTaskCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRunningTaskCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRunningTaskCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IScheduledWorkItem_Impl: ::windows_core::BaseImpl {
    fn CreateTrigger(this: &Self::This, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows_core::Result<()>;
    fn DeleteTrigger(this: &Self::This, itrigger: u16) -> ::windows_core::Result<()>;
    fn GetTriggerCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetTrigger(this: &Self::This, itrigger: u16) -> ::windows_core::Result<ITaskTrigger>;
    fn GetTriggerString(this: &Self::This, itrigger: u16) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetRunTimes(this: &Self::This, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetNextRunTime(this: &Self::This, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn SetIdleWait(this: &Self::This, widleminutes: u16, wdeadlineminutes: u16) -> ::windows_core::Result<()>;
    fn GetIdleWait(this: &Self::This, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows_core::Result<()>;
    fn Run(this: &Self::This) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
    fn EditWorkItem(this: &Self::This, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetMostRecentRunTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetExitCode(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetComment(this: &Self::This, pwszcomment: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetComment(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCreator(this: &Self::This, pwszcreator: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCreator(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetWorkItemData(this: &Self::This, cbdata: u16, rgbdata: *const u8) -> ::windows_core::Result<()>;
    fn GetWorkItemData(this: &Self::This, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetErrorRetryCount(this: &Self::This, wretrycount: u16) -> ::windows_core::Result<()>;
    fn GetErrorRetryCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetErrorRetryInterval(this: &Self::This, wretryinterval: u16) -> ::windows_core::Result<()>;
    fn GetErrorRetryInterval(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetFlags(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetAccountInformation(this: &Self::This, pwszaccountname: &::windows_core::PCWSTR, pwszpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccountInformation(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IScheduledWorkItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScheduledWorkItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTrigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTrigger(this, ::core::mem::transmute_copy(&pinewtrigger), ::core::mem::transmute_copy(&pptrigger)).into())
        }
        unsafe extern "system" fn DeleteTrigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTrigger(this, ::core::mem::transmute_copy(&itrigger)).into())
        }
        unsafe extern "system" fn GetTriggerCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTriggerCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTrigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTrigger(this, ::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrigger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTriggerString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTriggerString(this, ::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwsztrigger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRunTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRunTimes(this, ::core::mem::transmute_copy(&pstbegin), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&rgsttasktimes)).into())
        }
        unsafe extern "system" fn GetNextRunTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextRunTime(this, ::core::mem::transmute_copy(&pstnextrun)).into())
        }
        unsafe extern "system" fn SetIdleWait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIdleWait(this, ::core::mem::transmute_copy(&widleminutes), ::core::mem::transmute_copy(&wdeadlineminutes)).into())
        }
        unsafe extern "system" fn GetIdleWait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdleWait(this, ::core::mem::transmute_copy(&pwidleminutes), ::core::mem::transmute_copy(&pwdeadlineminutes)).into())
        }
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Run(this).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        unsafe extern "system" fn EditWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditWorkItem(this, ::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn GetMostRecentRunTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMostRecentRunTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastrun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExitCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExitCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwexitcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetComment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcomment: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComment(this, ::core::mem::transmute(&pwszcomment)).into())
        }
        unsafe extern "system" fn GetComment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszcomment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetComment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszcomment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcreator: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreator(this, ::core::mem::transmute(&pwszcreator)).into())
        }
        unsafe extern "system" fn GetCreator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszcreator: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCreator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszcreator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWorkItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkItemData(this, ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&rgbdata)).into())
        }
        unsafe extern "system" fn GetWorkItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWorkItemData(this, ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&prgbdata)).into())
        }
        unsafe extern "system" fn SetErrorRetryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorRetryCount(this, ::core::mem::transmute_copy(&wretrycount)).into())
        }
        unsafe extern "system" fn GetErrorRetryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorRetryCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwretrycount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetErrorRetryInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorRetryInterval(this, ::core::mem::transmute_copy(&wretryinterval)).into())
        }
        unsafe extern "system" fn GetErrorRetryInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorRetryInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwretryinterval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccountInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszaccountname: ::windows_core::PCWSTR, pwszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccountInformation(this, ::core::mem::transmute(&pwszaccountname), ::core::mem::transmute(&pwszpassword)).into())
        }
        unsafe extern "system" fn GetAccountInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccountInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszaccountname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IScheduledWorkItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTrigger: CreateTrigger::<Identity, Impl, OFFSET>,
            DeleteTrigger: DeleteTrigger::<Identity, Impl, OFFSET>,
            GetTriggerCount: GetTriggerCount::<Identity, Impl, OFFSET>,
            GetTrigger: GetTrigger::<Identity, Impl, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, Impl, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, Impl, OFFSET>,
            GetNextRunTime: GetNextRunTime::<Identity, Impl, OFFSET>,
            SetIdleWait: SetIdleWait::<Identity, Impl, OFFSET>,
            GetIdleWait: GetIdleWait::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            EditWorkItem: EditWorkItem::<Identity, Impl, OFFSET>,
            GetMostRecentRunTime: GetMostRecentRunTime::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetExitCode: GetExitCode::<Identity, Impl, OFFSET>,
            SetComment: SetComment::<Identity, Impl, OFFSET>,
            GetComment: GetComment::<Identity, Impl, OFFSET>,
            SetCreator: SetCreator::<Identity, Impl, OFFSET>,
            GetCreator: GetCreator::<Identity, Impl, OFFSET>,
            SetWorkItemData: SetWorkItemData::<Identity, Impl, OFFSET>,
            GetWorkItemData: GetWorkItemData::<Identity, Impl, OFFSET>,
            SetErrorRetryCount: SetErrorRetryCount::<Identity, Impl, OFFSET>,
            GetErrorRetryCount: GetErrorRetryCount::<Identity, Impl, OFFSET>,
            SetErrorRetryInterval: SetErrorRetryInterval::<Identity, Impl, OFFSET>,
            GetErrorRetryInterval: GetErrorRetryInterval::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, Impl, OFFSET>,
            GetAccountInformation: GetAccountInformation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISessionStateChangeTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn Delay(this: &Self::This, pdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDelay(this: &Self::This, delay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UserId(this: &Self::This, puser: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetUserId(this: &Self::This, user: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StateChange(this: &Self::This, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::Result<()>;
    fn SetStateChange(this: &Self::This, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISessionStateChangeTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISessionStateChangeTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delay(this, ::core::mem::transmute_copy(&pdelay)).into())
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelay(this, ::core::mem::transmute(&delay)).into())
        }
        unsafe extern "system" fn UserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UserId(this, ::core::mem::transmute_copy(&puser)).into())
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserId(this, ::core::mem::transmute(&user)).into())
        }
        unsafe extern "system" fn StateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StateChange(this, ::core::mem::transmute_copy(&ptype)).into())
        }
        unsafe extern "system" fn SetStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStateChange(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        ISessionStateChangeTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
            UserId: UserId::<Identity, Impl, OFFSET>,
            SetUserId: SetUserId::<Identity, Impl, OFFSET>,
            StateChange: StateChange::<Identity, Impl, OFFSET>,
            SetStateChange: SetStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IShowMessageAction_Impl: ::windows_core::BaseImpl + IAction_Impl {
    fn Title(this: &Self::This, ptitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetTitle(this: &Self::This, title: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MessageBody(this: &Self::This, pmessagebody: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetMessageBody(this: &Self::This, messagebody: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IShowMessageAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IShowMessageAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Title(this, ::core::mem::transmute_copy(&ptitle)).into())
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&title)).into())
        }
        unsafe extern "system" fn MessageBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmessagebody: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MessageBody(this, ::core::mem::transmute_copy(&pmessagebody)).into())
        }
        unsafe extern "system" fn SetMessageBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagebody: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageBody(this, ::core::mem::transmute(&messagebody)).into())
        }
        IShowMessageAction_Vtbl {
            base__: <IAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            MessageBody: MessageBody::<Identity, Impl, OFFSET>,
            SetMessageBody: SetMessageBody::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITask_Impl: ::windows_core::BaseImpl + IScheduledWorkItem_Impl {
    fn SetApplicationName(this: &Self::This, pwszapplicationname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetApplicationName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetParameters(this: &Self::This, pwszparameters: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetParameters(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetWorkingDirectory(this: &Self::This, pwszworkingdirectory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetWorkingDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetPriority(this: &Self::This, dwpriority: u32) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetTaskFlags(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetTaskFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxRunTime(this: &Self::This, dwmaxruntimems: u32) -> ::windows_core::Result<()>;
    fn GetMaxRunTime(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IScheduledWorkItem);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetApplicationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszapplicationname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationName(this, ::core::mem::transmute(&pwszapplicationname)).into())
        }
        unsafe extern "system" fn GetApplicationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplicationName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszapplicationname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszparameters: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute(&pwszparameters)).into())
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszparameters: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszworkingdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkingDirectory(this, ::core::mem::transmute(&pwszworkingdirectory)).into())
        }
        unsafe extern "system" fn GetWorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWorkingDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszworkingdirectory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&dwpriority)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTaskFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTaskFlags(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetTaskFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTaskFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxRunTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxRunTime(this, ::core::mem::transmute_copy(&dwmaxruntimems)).into())
        }
        unsafe extern "system" fn GetMaxRunTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxRunTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxruntimems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITask_Vtbl {
            base__: <IScheduledWorkItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetApplicationName: SetApplicationName::<Identity, Impl, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            GetWorkingDirectory: GetWorkingDirectory::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetTaskFlags: SetTaskFlags::<Identity, Impl, OFFSET>,
            GetTaskFlags: GetTaskFlags::<Identity, Impl, OFFSET>,
            SetMaxRunTime: SetMaxRunTime::<Identity, Impl, OFFSET>,
            GetMaxRunTime: GetMaxRunTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskDefinition_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn RegistrationInfo(this: &Self::This) -> ::windows_core::Result<IRegistrationInfo>;
    fn SetRegistrationInfo(this: &Self::This, pregistrationinfo: ::core::option::Option<&IRegistrationInfo>) -> ::windows_core::Result<()>;
    fn Triggers(this: &Self::This) -> ::windows_core::Result<ITriggerCollection>;
    fn SetTriggers(this: &Self::This, ptriggers: ::core::option::Option<&ITriggerCollection>) -> ::windows_core::Result<()>;
    fn Settings(this: &Self::This) -> ::windows_core::Result<ITaskSettings>;
    fn SetSettings(this: &Self::This, psettings: ::core::option::Option<&ITaskSettings>) -> ::windows_core::Result<()>;
    fn Data(this: &Self::This, pdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetData(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Principal(this: &Self::This) -> ::windows_core::Result<IPrincipal>;
    fn SetPrincipal(this: &Self::This, pprincipal: ::core::option::Option<&IPrincipal>) -> ::windows_core::Result<()>;
    fn Actions(this: &Self::This) -> ::windows_core::Result<IActionCollection>;
    fn SetActions(this: &Self::This, pactions: ::core::option::Option<&IActionCollection>) -> ::windows_core::Result<()>;
    fn XmlText(this: &Self::This, pxml: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetXmlText(this: &Self::This, xml: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegistrationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegistrationInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregistrationinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRegistrationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pregistrationinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRegistrationInfo(this, ::windows_core::from_raw_borrowed(&pregistrationinfo)).into())
        }
        unsafe extern "system" fn Triggers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptriggers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Triggers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptriggers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTriggers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptriggers: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTriggers(this, ::windows_core::from_raw_borrowed(&ptriggers)).into())
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Settings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSettings(this, ::windows_core::from_raw_borrowed(&psettings)).into())
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Data(this, ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn Principal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprincipal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Principal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprincipal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrincipal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprincipal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrincipal(this, ::windows_core::from_raw_borrowed(&pprincipal)).into())
        }
        unsafe extern "system" fn Actions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppactions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Actions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppactions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActions(this, ::windows_core::from_raw_borrowed(&pactions)).into())
        }
        unsafe extern "system" fn XmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::XmlText(this, ::core::mem::transmute_copy(&pxml)).into())
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXmlText(this, ::core::mem::transmute(&xml)).into())
        }
        ITaskDefinition_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegistrationInfo: RegistrationInfo::<Identity, Impl, OFFSET>,
            SetRegistrationInfo: SetRegistrationInfo::<Identity, Impl, OFFSET>,
            Triggers: Triggers::<Identity, Impl, OFFSET>,
            SetTriggers: SetTriggers::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            SetSettings: SetSettings::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Principal: Principal::<Identity, Impl, OFFSET>,
            SetPrincipal: SetPrincipal::<Identity, Impl, OFFSET>,
            Actions: Actions::<Identity, Impl, OFFSET>,
            SetActions: SetActions::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskFolder_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFolder(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<ITaskFolder>;
    fn GetFolders(this: &Self::This, flags: i32) -> ::windows_core::Result<ITaskFolderCollection>;
    fn CreateFolder(this: &Self::This, subfoldername: &::windows_core::BSTR, sddl: &super::Variant::VARIANT) -> ::windows_core::Result<ITaskFolder>;
    fn DeleteFolder(this: &Self::This, subfoldername: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<()>;
    fn GetTask(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IRegisteredTask>;
    fn GetTasks(this: &Self::This, flags: i32) -> ::windows_core::Result<IRegisteredTaskCollection>;
    fn DeleteTask(this: &Self::This, name: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<()>;
    fn RegisterTask(this: &Self::This, path: &::windows_core::BSTR, xmltext: &::windows_core::BSTR, flags: i32, userid: &super::Variant::VARIANT, password: &super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Variant::VARIANT) -> ::windows_core::Result<IRegisteredTask>;
    fn RegisterTaskDefinition(this: &Self::This, path: &::windows_core::BSTR, pdefinition: ::core::option::Option<&ITaskDefinition>, flags: i32, userid: &super::Variant::VARIANT, password: &super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Variant::VARIANT) -> ::windows_core::Result<IRegisteredTask>;
    fn GetSecurityDescriptor(this: &Self::This, securityinformation: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSecurityDescriptor(this: &Self::This, sddl: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskFolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskFolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolder(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolders(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subfoldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, sddl: super::Variant::VARIANT, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolder(this, ::core::mem::transmute(&subfoldername), ::core::mem::transmute(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subfoldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFolder(this, ::core::mem::transmute(&subfoldername), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTask(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTasks(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptasks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTask(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn RegisterTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, xmltext: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, userid: super::Variant::VARIANT, password: super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: super::Variant::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterTask(this, ::core::mem::transmute(&path), ::core::mem::transmute(&xmltext), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&userid), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterTaskDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdefinition: *mut ::core::ffi::c_void, flags: i32, userid: super::Variant::VARIANT, password: super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: super::Variant::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterTaskDefinition(this, ::core::mem::transmute(&path), ::windows_core::from_raw_borrowed(&pdefinition), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&userid), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityDescriptor(this, ::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psddl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sddl: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::core::mem::transmute(&sddl), ::core::mem::transmute_copy(&flags)).into())
        }
        ITaskFolder_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            GetFolders: GetFolders::<Identity, Impl, OFFSET>,
            CreateFolder: CreateFolder::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            GetTask: GetTask::<Identity, Impl, OFFSET>,
            GetTasks: GetTasks::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            RegisterTask: RegisterTask::<Identity, Impl, OFFSET>,
            RegisterTaskDefinition: RegisterTaskDefinition::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskFolderCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<ITaskFolder>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskFolderCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskFolderCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITaskFolderCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITaskHandler_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This, phandlerservices: ::core::option::Option<&::windows_core::IUnknown>, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITaskHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::windows_core::from_raw_borrowed(&phandlerservices), ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        ITaskHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITaskHandlerStatus_Impl: ::windows_core::BaseImpl {
    fn UpdateStatus(this: &Self::This, percentcomplete: i16, statusmessage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TaskCompleted(this: &Self::This, taskerrcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITaskHandlerStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandlerStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskHandlerStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandlerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateStatus(this, ::core::mem::transmute_copy(&percentcomplete), ::core::mem::transmute(&statusmessage)).into())
        }
        unsafe extern "system" fn TaskCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskHandlerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskerrcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TaskCompleted(this, ::core::mem::transmute_copy(&taskerrcode)).into())
        }
        ITaskHandlerStatus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateStatus: UpdateStatus::<Identity, Impl, OFFSET>,
            TaskCompleted: TaskCompleted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskNamedValueCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This, pcount: *mut i32) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<ITaskNamedValuePair>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Create(this: &Self::This, name: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<ITaskNamedValuePair>;
    fn Remove(this: &Self::This, index: i32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskNamedValueCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskNamedValueCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppair, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>, pppair: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppair, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        ITaskNamedValueCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskNamedValuePair_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This, pname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This, pvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskNamedValuePair {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskNamedValuePair {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Name(this, ::core::mem::transmute_copy(&pname)).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Value(this, ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&value)).into())
        }
        ITaskNamedValuePair_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITaskScheduler_Impl: ::windows_core::BaseImpl {
    fn SetTargetComputer(this: &Self::This, pwszcomputer: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetTargetComputer(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Enum(this: &Self::This) -> ::windows_core::Result<IEnumWorkItems>;
    fn Activate(this: &Self::This, pwszname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Delete(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn NewWorkItem(this: &Self::This, pwsztaskname: &::windows_core::PCWSTR, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn AddWorkItem(this: &Self::This, pwsztaskname: &::windows_core::PCWSTR, pworkitem: ::core::option::Option<&IScheduledWorkItem>) -> ::windows_core::Result<()>;
    fn IsOfType(this: &Self::This, pwszname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITaskScheduler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskScheduler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTargetComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcomputer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetComputer(this, ::core::mem::transmute(&pwszcomputer)).into())
        }
        unsafe extern "system" fn GetTargetComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszcomputer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumworkitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Activate(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn NewWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows_core::PCWSTR, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NewWorkItem(this, ::core::mem::transmute(&pwsztaskname), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows_core::PCWSTR, pworkitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddWorkItem(this, ::core::mem::transmute(&pwsztaskname), ::windows_core::from_raw_borrowed(&pworkitem)).into())
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsOfType(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&riid)).into())
        }
        ITaskScheduler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTargetComputer: SetTargetComputer::<Identity, Impl, OFFSET>,
            GetTargetComputer: GetTargetComputer::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            NewWorkItem: NewWorkItem::<Identity, Impl, OFFSET>,
            AddWorkItem: AddWorkItem::<Identity, Impl, OFFSET>,
            IsOfType: IsOfType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskService_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetFolder(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<ITaskFolder>;
    fn GetRunningTasks(this: &Self::This, flags: i32) -> ::windows_core::Result<IRunningTaskCollection>;
    fn NewTask(this: &Self::This, flags: u32) -> ::windows_core::Result<ITaskDefinition>;
    fn Connect(this: &Self::This, servername: &super::Variant::VARIANT, user: &super::Variant::VARIANT, domain: &super::Variant::VARIANT, password: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Connected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TargetServer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ConnectedUser(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ConnectedDomain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HighestVersion(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolder(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRunningTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRunningTasks(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtasks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NewTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NewTask(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servername: super::Variant::VARIANT, user: super::Variant::VARIANT, domain: super::Variant::VARIANT, password: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute(&servername), ::core::mem::transmute(&user), ::core::mem::transmute(&domain), ::core::mem::transmute(&password)).into())
        }
        unsafe extern "system" fn Connected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Connected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TargetServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetServer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pserver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectedUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectedUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectedDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectedDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HighestVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HighestVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITaskService_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            GetRunningTasks: GetRunningTasks::<Identity, Impl, OFFSET>,
            NewTask: NewTask::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Connected: Connected::<Identity, Impl, OFFSET>,
            TargetServer: TargetServer::<Identity, Impl, OFFSET>,
            ConnectedUser: ConnectedUser::<Identity, Impl, OFFSET>,
            ConnectedDomain: ConnectedDomain::<Identity, Impl, OFFSET>,
            HighestVersion: HighestVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskSettings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AllowDemandStart(this: &Self::This, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetAllowDemandStart(this: &Self::This, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RestartInterval(this: &Self::This, prestartinterval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRestartInterval(this: &Self::This, restartinterval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RestartCount(this: &Self::This, prestartcount: *mut i32) -> ::windows_core::Result<()>;
    fn SetRestartCount(this: &Self::This, restartcount: i32) -> ::windows_core::Result<()>;
    fn MultipleInstances(this: &Self::This, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows_core::Result<()>;
    fn SetMultipleInstances(this: &Self::This, policy: TASK_INSTANCES_POLICY) -> ::windows_core::Result<()>;
    fn StopIfGoingOnBatteries(this: &Self::This, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetStopIfGoingOnBatteries(this: &Self::This, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DisallowStartIfOnBatteries(this: &Self::This, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetDisallowStartIfOnBatteries(this: &Self::This, disallowstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowHardTerminate(this: &Self::This, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetAllowHardTerminate(this: &Self::This, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn StartWhenAvailable(this: &Self::This, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetStartWhenAvailable(this: &Self::This, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn XmlText(this: &Self::This, ptext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetXmlText(this: &Self::This, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RunOnlyIfNetworkAvailable(this: &Self::This, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRunOnlyIfNetworkAvailable(this: &Self::This, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ExecutionTimeLimit(this: &Self::This, pexecutiontimelimit: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetExecutionTimeLimit(this: &Self::This, executiontimelimit: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DeleteExpiredTaskAfter(this: &Self::This, pexpirationdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDeleteExpiredTaskAfter(this: &Self::This, expirationdelay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This, ppriority: *mut i32) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, priority: i32) -> ::windows_core::Result<()>;
    fn Compatibility(this: &Self::This, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows_core::Result<()>;
    fn SetCompatibility(this: &Self::This, compatlevel: TASK_COMPATIBILITY) -> ::windows_core::Result<()>;
    fn Hidden(this: &Self::This, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetHidden(this: &Self::This, hidden: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IdleSettings(this: &Self::This) -> ::windows_core::Result<IIdleSettings>;
    fn SetIdleSettings(this: &Self::This, pidlesettings: ::core::option::Option<&IIdleSettings>) -> ::windows_core::Result<()>;
    fn RunOnlyIfIdle(this: &Self::This, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRunOnlyIfIdle(this: &Self::This, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn WakeToRun(this: &Self::This, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetWakeToRun(this: &Self::This, wake: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn NetworkSettings(this: &Self::This) -> ::windows_core::Result<INetworkSettings>;
    fn SetNetworkSettings(this: &Self::This, pnetworksettings: ::core::option::Option<&INetworkSettings>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllowDemandStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllowDemandStart(this, ::core::mem::transmute_copy(&pallowdemandstart)).into())
        }
        unsafe extern "system" fn SetAllowDemandStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowDemandStart(this, ::core::mem::transmute_copy(&allowdemandstart)).into())
        }
        unsafe extern "system" fn RestartInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prestartinterval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestartInterval(this, ::core::mem::transmute_copy(&prestartinterval)).into())
        }
        unsafe extern "system" fn SetRestartInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restartinterval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRestartInterval(this, ::core::mem::transmute(&restartinterval)).into())
        }
        unsafe extern "system" fn RestartCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestartCount(this, ::core::mem::transmute_copy(&prestartcount)).into())
        }
        unsafe extern "system" fn SetRestartCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRestartCount(this, ::core::mem::transmute_copy(&restartcount)).into())
        }
        unsafe extern "system" fn MultipleInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MultipleInstances(this, ::core::mem::transmute_copy(&ppolicy)).into())
        }
        unsafe extern "system" fn SetMultipleInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultipleInstances(this, ::core::mem::transmute_copy(&policy)).into())
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopIfGoingOnBatteries(this, ::core::mem::transmute_copy(&pstopifonbatteries)).into())
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStopIfGoingOnBatteries(this, ::core::mem::transmute_copy(&stopifonbatteries)).into())
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisallowStartIfOnBatteries(this, ::core::mem::transmute_copy(&pdisallowstart)).into())
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisallowStartIfOnBatteries(this, ::core::mem::transmute_copy(&disallowstart)).into())
        }
        unsafe extern "system" fn AllowHardTerminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllowHardTerminate(this, ::core::mem::transmute_copy(&pallowhardterminate)).into())
        }
        unsafe extern "system" fn SetAllowHardTerminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowHardTerminate(this, ::core::mem::transmute_copy(&allowhardterminate)).into())
        }
        unsafe extern "system" fn StartWhenAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartWhenAvailable(this, ::core::mem::transmute_copy(&pstartwhenavailable)).into())
        }
        unsafe extern "system" fn SetStartWhenAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartWhenAvailable(this, ::core::mem::transmute_copy(&startwhenavailable)).into())
        }
        unsafe extern "system" fn XmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::XmlText(this, ::core::mem::transmute_copy(&ptext)).into())
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXmlText(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunOnlyIfNetworkAvailable(this, ::core::mem::transmute_copy(&prunonlyifnetworkavailable)).into())
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRunOnlyIfNetworkAvailable(this, ::core::mem::transmute_copy(&runonlyifnetworkavailable)).into())
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecutionTimeLimit(this, ::core::mem::transmute_copy(&pexecutiontimelimit)).into())
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executiontimelimit: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExecutionTimeLimit(this, ::core::mem::transmute(&executiontimelimit)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enabled(this, ::core::mem::transmute_copy(&penabled)).into())
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteExpiredTaskAfter(this, ::core::mem::transmute_copy(&pexpirationdelay)).into())
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expirationdelay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeleteExpiredTaskAfter(this, ::core::mem::transmute(&expirationdelay)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Priority(this, ::core::mem::transmute_copy(&ppriority)).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn Compatibility<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compatibility(this, ::core::mem::transmute_copy(&pcompatlevel)).into())
        }
        unsafe extern "system" fn SetCompatibility<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompatibility(this, ::core::mem::transmute_copy(&compatlevel)).into())
        }
        unsafe extern "system" fn Hidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Hidden(this, ::core::mem::transmute_copy(&phidden)).into())
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHidden(this, ::core::mem::transmute_copy(&hidden)).into())
        }
        unsafe extern "system" fn IdleSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppidlesettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IdleSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidlesettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIdleSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidlesettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIdleSettings(this, ::windows_core::from_raw_borrowed(&pidlesettings)).into())
        }
        unsafe extern "system" fn RunOnlyIfIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunOnlyIfIdle(this, ::core::mem::transmute_copy(&prunonlyifidle)).into())
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRunOnlyIfIdle(this, ::core::mem::transmute_copy(&runonlyifidle)).into())
        }
        unsafe extern "system" fn WakeToRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WakeToRun(this, ::core::mem::transmute_copy(&pwake)).into())
        }
        unsafe extern "system" fn SetWakeToRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wake: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWakeToRun(this, ::core::mem::transmute_copy(&wake)).into())
        }
        unsafe extern "system" fn NetworkSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworksettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetworksettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkSettings(this, ::windows_core::from_raw_borrowed(&pnetworksettings)).into())
        }
        ITaskSettings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllowDemandStart: AllowDemandStart::<Identity, Impl, OFFSET>,
            SetAllowDemandStart: SetAllowDemandStart::<Identity, Impl, OFFSET>,
            RestartInterval: RestartInterval::<Identity, Impl, OFFSET>,
            SetRestartInterval: SetRestartInterval::<Identity, Impl, OFFSET>,
            RestartCount: RestartCount::<Identity, Impl, OFFSET>,
            SetRestartCount: SetRestartCount::<Identity, Impl, OFFSET>,
            MultipleInstances: MultipleInstances::<Identity, Impl, OFFSET>,
            SetMultipleInstances: SetMultipleInstances::<Identity, Impl, OFFSET>,
            StopIfGoingOnBatteries: StopIfGoingOnBatteries::<Identity, Impl, OFFSET>,
            SetStopIfGoingOnBatteries: SetStopIfGoingOnBatteries::<Identity, Impl, OFFSET>,
            DisallowStartIfOnBatteries: DisallowStartIfOnBatteries::<Identity, Impl, OFFSET>,
            SetDisallowStartIfOnBatteries: SetDisallowStartIfOnBatteries::<Identity, Impl, OFFSET>,
            AllowHardTerminate: AllowHardTerminate::<Identity, Impl, OFFSET>,
            SetAllowHardTerminate: SetAllowHardTerminate::<Identity, Impl, OFFSET>,
            StartWhenAvailable: StartWhenAvailable::<Identity, Impl, OFFSET>,
            SetStartWhenAvailable: SetStartWhenAvailable::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
            RunOnlyIfNetworkAvailable: RunOnlyIfNetworkAvailable::<Identity, Impl, OFFSET>,
            SetRunOnlyIfNetworkAvailable: SetRunOnlyIfNetworkAvailable::<Identity, Impl, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, Impl, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            DeleteExpiredTaskAfter: DeleteExpiredTaskAfter::<Identity, Impl, OFFSET>,
            SetDeleteExpiredTaskAfter: SetDeleteExpiredTaskAfter::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Compatibility: Compatibility::<Identity, Impl, OFFSET>,
            SetCompatibility: SetCompatibility::<Identity, Impl, OFFSET>,
            Hidden: Hidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            IdleSettings: IdleSettings::<Identity, Impl, OFFSET>,
            SetIdleSettings: SetIdleSettings::<Identity, Impl, OFFSET>,
            RunOnlyIfIdle: RunOnlyIfIdle::<Identity, Impl, OFFSET>,
            SetRunOnlyIfIdle: SetRunOnlyIfIdle::<Identity, Impl, OFFSET>,
            WakeToRun: WakeToRun::<Identity, Impl, OFFSET>,
            SetWakeToRun: SetWakeToRun::<Identity, Impl, OFFSET>,
            NetworkSettings: NetworkSettings::<Identity, Impl, OFFSET>,
            SetNetworkSettings: SetNetworkSettings::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskSettings2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DisallowStartOnRemoteAppSession(this: &Self::This, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(this: &Self::This, disallowstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseUnifiedSchedulingEngine(this: &Self::This, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(this: &Self::This, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskSettings2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskSettings2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisallowStartOnRemoteAppSession(this, ::core::mem::transmute_copy(&pdisallowstart)).into())
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisallowStartOnRemoteAppSession(this, ::core::mem::transmute_copy(&disallowstart)).into())
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseUnifiedSchedulingEngine(this, ::core::mem::transmute_copy(&puseunifiedengine)).into())
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseUnifiedSchedulingEngine(this, ::core::mem::transmute_copy(&useunifiedengine)).into())
        }
        ITaskSettings2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskSettings3_Impl: ::windows_core::BaseImpl + ITaskSettings_Impl {
    fn DisallowStartOnRemoteAppSession(this: &Self::This, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(this: &Self::This, disallowstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseUnifiedSchedulingEngine(this: &Self::This, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(this: &Self::This, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn MaintenanceSettings(this: &Self::This) -> ::windows_core::Result<IMaintenanceSettings>;
    fn SetMaintenanceSettings(this: &Self::This, pmaintenancesettings: ::core::option::Option<&IMaintenanceSettings>) -> ::windows_core::Result<()>;
    fn CreateMaintenanceSettings(this: &Self::This) -> ::windows_core::Result<IMaintenanceSettings>;
    fn Volatile(this: &Self::This, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetVolatile(this: &Self::This, volatile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITaskSettings3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITaskSettings);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskSettings3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisallowStartOnRemoteAppSession(this, ::core::mem::transmute_copy(&pdisallowstart)).into())
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisallowStartOnRemoteAppSession(this, ::core::mem::transmute_copy(&disallowstart)).into())
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseUnifiedSchedulingEngine(this, ::core::mem::transmute_copy(&puseunifiedengine)).into())
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseUnifiedSchedulingEngine(this, ::core::mem::transmute_copy(&useunifiedengine)).into())
        }
        unsafe extern "system" fn MaintenanceSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaintenanceSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmaintenancesettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaintenanceSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaintenancesettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaintenanceSettings(this, ::windows_core::from_raw_borrowed(&pmaintenancesettings)).into())
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMaintenanceSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmaintenancesettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Volatile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Volatile(this, ::core::mem::transmute_copy(&pvolatile)).into())
        }
        unsafe extern "system" fn SetVolatile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volatile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolatile(this, ::core::mem::transmute_copy(&volatile)).into())
        }
        ITaskSettings3_Vtbl {
            base__: <ITaskSettings as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
            MaintenanceSettings: MaintenanceSettings::<Identity, Impl, OFFSET>,
            SetMaintenanceSettings: SetMaintenanceSettings::<Identity, Impl, OFFSET>,
            CreateMaintenanceSettings: CreateMaintenanceSettings::<Identity, Impl, OFFSET>,
            Volatile: Volatile::<Identity, Impl, OFFSET>,
            SetVolatile: SetVolatile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITaskTrigger_Impl: ::windows_core::BaseImpl {
    fn SetTrigger(this: &Self::This, ptrigger: *const TASK_TRIGGER) -> ::windows_core::Result<()>;
    fn GetTrigger(this: &Self::This, ptrigger: *mut TASK_TRIGGER) -> ::windows_core::Result<()>;
    fn GetTriggerString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ITaskTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTrigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrigger(this, ::core::mem::transmute_copy(&ptrigger)).into())
        }
        unsafe extern "system" fn GetTrigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTrigger(this, ::core::mem::transmute_copy(&ptrigger)).into())
        }
        unsafe extern "system" fn GetTriggerString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTriggerString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwsztrigger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITaskTrigger_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTrigger: SetTrigger::<Identity, Impl, OFFSET>,
            GetTrigger: GetTrigger::<Identity, Impl, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITaskVariables_Impl: ::windows_core::BaseImpl {
    fn GetInput(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOutput(this: &Self::This, input: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetContext(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITaskVariables {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITaskVariables {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutput(this, ::core::mem::transmute(&input)).into())
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITaskVariables_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInput: GetInput::<Identity, Impl, OFFSET>,
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimeTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn RandomDelay(this: &Self::This, prandomdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRandomDelay(this: &Self::This, randomdelay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITimeTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimeTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimeTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RandomDelay(this, ::core::mem::transmute_copy(&prandomdelay)).into())
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimeTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, randomdelay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRandomDelay(this, ::core::mem::transmute(&randomdelay)).into())
        }
        ITimeTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITrigger_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Type(this: &Self::This, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This, pid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetId(this: &Self::This, id: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Repetition(this: &Self::This) -> ::windows_core::Result<IRepetitionPattern>;
    fn SetRepetition(this: &Self::This, prepeat: ::core::option::Option<&IRepetitionPattern>) -> ::windows_core::Result<()>;
    fn ExecutionTimeLimit(this: &Self::This, ptimelimit: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetExecutionTimeLimit(this: &Self::This, timelimit: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartBoundary(this: &Self::This, pstart: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetStartBoundary(this: &Self::This, start: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EndBoundary(this: &Self::This, pend: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetEndBoundary(this: &Self::This, end: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Type(this, ::core::mem::transmute_copy(&ptype)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Id(this, ::core::mem::transmute_copy(&pid)).into())
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id)).into())
        }
        unsafe extern "system" fn Repetition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepeat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Repetition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprepeat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRepetition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prepeat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRepetition(this, ::windows_core::from_raw_borrowed(&prepeat)).into())
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimelimit: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecutionTimeLimit(this, ::core::mem::transmute_copy(&ptimelimit)).into())
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timelimit: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExecutionTimeLimit(this, ::core::mem::transmute(&timelimit)).into())
        }
        unsafe extern "system" fn StartBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstart: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartBoundary(this, ::core::mem::transmute_copy(&pstart)).into())
        }
        unsafe extern "system" fn SetStartBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, start: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartBoundary(this, ::core::mem::transmute(&start)).into())
        }
        unsafe extern "system" fn EndBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pend: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndBoundary(this, ::core::mem::transmute_copy(&pend)).into())
        }
        unsafe extern "system" fn SetEndBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, end: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndBoundary(this, ::core::mem::transmute(&end)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enabled(this, ::core::mem::transmute_copy(&penabled)).into())
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        ITrigger_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Repetition: Repetition::<Identity, Impl, OFFSET>,
            SetRepetition: SetRepetition::<Identity, Impl, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, Impl, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, Impl, OFFSET>,
            StartBoundary: StartBoundary::<Identity, Impl, OFFSET>,
            SetStartBoundary: SetStartBoundary::<Identity, Impl, OFFSET>,
            EndBoundary: EndBoundary::<Identity, Impl, OFFSET>,
            SetEndBoundary: SetEndBoundary::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITriggerCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This, pcount: *mut i32) -> ::windows_core::Result<()>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<ITrigger>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Create(this: &Self::This, r#type: TASK_TRIGGER_TYPE2) -> ::windows_core::Result<ITrigger>;
    fn Remove(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITriggerCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITriggerCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrigger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrigger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&index)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        ITriggerCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWeeklyTrigger_Impl: ::windows_core::BaseImpl + ITrigger_Impl {
    fn DaysOfWeek(this: &Self::This, pdays: *mut i16) -> ::windows_core::Result<()>;
    fn SetDaysOfWeek(this: &Self::This, days: i16) -> ::windows_core::Result<()>;
    fn WeeksInterval(this: &Self::This, pweeks: *mut i16) -> ::windows_core::Result<()>;
    fn SetWeeksInterval(this: &Self::This, weeks: i16) -> ::windows_core::Result<()>;
    fn RandomDelay(this: &Self::This, prandomdelay: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRandomDelay(this: &Self::This, randomdelay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWeeklyTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITrigger);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWeeklyTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DaysOfWeek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DaysOfWeek(this, ::core::mem::transmute_copy(&pdays)).into())
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysOfWeek(this, ::core::mem::transmute_copy(&days)).into())
        }
        unsafe extern "system" fn WeeksInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WeeksInterval(this, ::core::mem::transmute_copy(&pweeks)).into())
        }
        unsafe extern "system" fn SetWeeksInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWeeksInterval(this, ::core::mem::transmute_copy(&weeks)).into())
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RandomDelay(this, ::core::mem::transmute_copy(&prandomdelay)).into())
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, randomdelay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRandomDelay(this, ::core::mem::transmute(&randomdelay)).into())
        }
        IWeeklyTrigger_Vtbl {
            base__: <ITrigger as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DaysOfWeek: DaysOfWeek::<Identity, Impl, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, Impl, OFFSET>,
            WeeksInterval: WeeksInterval::<Identity, Impl, OFFSET>,
            SetWeeksInterval: SetWeeksInterval::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
