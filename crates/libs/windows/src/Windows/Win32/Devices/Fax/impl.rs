#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccount_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AccountName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Folders(this: &Self::This) -> ::windows_core::Result<IFaxAccountFolders>;
    fn ListenToAccountEvents(this: &Self::This, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn RegisteredEvents(this: &Self::This) -> ::windows_core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccount {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccount {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccountName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstraccountname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraccountname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Folders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Folders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ListenToAccountEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ListenToAccountEvents(this, ::core::mem::transmute_copy(&eventtypes)).into())
        }
        unsafe extern "system" fn RegisteredEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisteredEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pregisteredevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccount_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccountName: AccountName::<Identity, Impl, OFFSET>,
            Folders: Folders::<Identity, Impl, OFFSET>,
            ListenToAccountEvents: ListenToAccountEvents::<Identity, Impl, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountFolders_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(this: &Self::This) -> ::windows_core::Result<IFaxAccountOutgoingQueue>;
    fn IncomingQueue(this: &Self::This) -> ::windows_core::Result<IFaxAccountIncomingQueue>;
    fn IncomingArchive(this: &Self::This) -> ::windows_core::Result<IFaxAccountIncomingArchive>;
    fn OutgoingArchive(this: &Self::This) -> ::windows_core::Result<IFaxAccountOutgoingArchive>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountFolders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountFolders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OutgoingQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IncomingQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IncomingArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingarchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OutgoingArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingarchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccountFolders_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OutgoingQueue: OutgoingQueue::<Identity, Impl, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, Impl, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, Impl, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountIncomingArchive_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SizeHigh(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMessages(this: &Self::This, lprefetchsize: i32) -> ::windows_core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(this: &Self::This, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountIncomingArchive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountIncomingArchive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SizeLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeLow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeHigh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessages(this, ::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessage(this, ::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccountIncomingArchive_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountIncomingQueue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(this: &Self::This) -> ::windows_core::Result<IFaxIncomingJobs>;
    fn GetJob(this: &Self::This, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountIncomingQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountIncomingQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJobs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJob(this, ::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccountIncomingQueue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountNotify_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnIncomingJobRemoved(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnIncomingJobChanged(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows_core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows_core::Result<()>;
    fn OnOutgoingJobAdded(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingJobRemoved(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingJobChanged(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows_core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows_core::Result<()>;
    fn OnIncomingMessageAdded(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows_core::BSTR, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn OnIncomingMessageRemoved(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows_core::BSTR, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn OnOutgoingMessageAdded(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingMessageRemoved(this: &Self::This, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnServerShutDown(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingJobAdded(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingJobRemoved(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingJobChanged(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid), ::windows_core::from_raw_borrowed(&pjobstatus)).into())
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingJobAdded(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingJobRemoved(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingJobChanged(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid), ::windows_core::from_raw_borrowed(&pjobstatus)).into())
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingMessageAdded(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid), ::core::mem::transmute_copy(&faddedtoreceivefolder)).into())
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingMessageRemoved(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid), ::core::mem::transmute_copy(&fremovedfromreceivefolder)).into())
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingMessageAdded(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid)).into())
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingMessageRemoved(this, ::windows_core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid)).into())
        }
        unsafe extern "system" fn OnServerShutDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnServerShutDown(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        IFaxAccountNotify_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, Impl, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, Impl, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, Impl, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, Impl, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, Impl, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, Impl, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, Impl, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, Impl, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountOutgoingArchive_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SizeHigh(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMessages(this: &Self::This, lprefetchsize: i32) -> ::windows_core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(this: &Self::This, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountOutgoingArchive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountOutgoingArchive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SizeLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeLow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeHigh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessages(this, ::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessage(this, ::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccountOutgoingArchive_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountOutgoingQueue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(this: &Self::This) -> ::windows_core::Result<IFaxOutgoingJobs>;
    fn GetJob(this: &Self::This, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountOutgoingQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountOutgoingQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJobs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJob(this, ::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccountOutgoingQueue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountSet_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetAccounts(this: &Self::This) -> ::windows_core::Result<IFaxAccounts>;
    fn GetAccount(this: &Self::This, bstraccountname: &::windows_core::BSTR) -> ::windows_core::Result<IFaxAccount>;
    fn AddAccount(this: &Self::This, bstraccountname: &::windows_core::BSTR) -> ::windows_core::Result<IFaxAccount>;
    fn RemoveAccount(this: &Self::This, bstraccountname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccountSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccountSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAccounts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxaccounts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccounts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxaccounts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccount(this, ::core::mem::transmute(&bstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxaccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddAccount(this, ::core::mem::transmute(&bstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxaccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAccount(this, ::core::mem::transmute(&bstraccountname)).into())
        }
        IFaxAccountSet_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAccounts: GetAccounts::<Identity, Impl, OFFSET>,
            GetAccount: GetAccount::<Identity, Impl, OFFSET>,
            AddAccount: AddAccount::<Identity, Impl, OFFSET>,
            RemoveAccount: RemoveAccount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccounts_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxAccount>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxAccounts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxAccounts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxaccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxAccounts_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxActivity_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn IncomingMessages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RoutingMessages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OutgoingMessages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueuedMessages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxActivity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxActivity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IncomingMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plincomingmessages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plincomingmessages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoutingMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plroutingmessages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoutingMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plroutingmessages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OutgoingMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploutgoingmessages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ploutgoingmessages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueuedMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plqueuedmessages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueuedMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plqueuedmessages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        IFaxActivity_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IncomingMessages: IncomingMessages::<Identity, Impl, OFFSET>,
            RoutingMessages: RoutingMessages::<Identity, Impl, OFFSET>,
            OutgoingMessages: OutgoingMessages::<Identity, Impl, OFFSET>,
            QueuedMessages: QueuedMessages::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxActivityLogging_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn LogIncoming(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogIncoming(this: &Self::This, blogincoming: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn LogOutgoing(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogOutgoing(this: &Self::This, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DatabasePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDatabasePath(this: &Self::This, bstrdatabasepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxActivityLogging {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxActivityLogging {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LogIncoming<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblogincoming: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LogIncoming(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblogincoming, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLogIncoming<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blogincoming: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogIncoming(this, ::core::mem::transmute_copy(&blogincoming)).into())
        }
        unsafe extern "system" fn LogOutgoing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblogoutgoing: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LogOutgoing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblogoutgoing, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLogOutgoing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogOutgoing(this, ::core::mem::transmute_copy(&blogoutgoing)).into())
        }
        unsafe extern "system" fn DatabasePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdatabasepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DatabasePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdatabasepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDatabasePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdatabasepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDatabasePath(this, ::core::mem::transmute(&bstrdatabasepath)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        IFaxActivityLogging_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LogIncoming: LogIncoming::<Identity, Impl, OFFSET>,
            SetLogIncoming: SetLogIncoming::<Identity, Impl, OFFSET>,
            LogOutgoing: LogOutgoing::<Identity, Impl, OFFSET>,
            SetLogOutgoing: SetLogOutgoing::<Identity, Impl, OFFSET>,
            DatabasePath: DatabasePath::<Identity, Impl, OFFSET>,
            SetDatabasePath: SetDatabasePath::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxConfiguration_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(this: &Self::This, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ArchiveLocation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetArchiveLocation(this: &Self::This, bstrarchivelocation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SizeQuotaWarning(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(this: &Self::This, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn HighQuotaWaterMark(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(this: &Self::This, lhighquotawatermark: i32) -> ::windows_core::Result<()>;
    fn LowQuotaWaterMark(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(this: &Self::This, llowquotawatermark: i32) -> ::windows_core::Result<()>;
    fn ArchiveAgeLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetArchiveAgeLimit(this: &Self::This, larchiveagelimit: i32) -> ::windows_core::Result<()>;
    fn ArchiveSizeLow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ArchiveSizeHigh(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OutgoingQueueBlocked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueueBlocked(this: &Self::This, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn OutgoingQueuePaused(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueuePaused(this: &Self::This, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowPersonalCoverPages(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(this: &Self::This, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseDeviceTSID(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(this: &Self::This, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRetries(this: &Self::This, lretries: i32) -> ::windows_core::Result<()>;
    fn RetryDelay(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRetryDelay(this: &Self::This, lretrydelay: i32) -> ::windows_core::Result<()>;
    fn DiscountRateStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetDiscountRateStart(this: &Self::This, datediscountratestart: f64) -> ::windows_core::Result<()>;
    fn DiscountRateEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetDiscountRateEnd(this: &Self::This, datediscountrateend: f64) -> ::windows_core::Result<()>;
    fn OutgoingQueueAgeLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetOutgoingQueueAgeLimit(this: &Self::This, loutgoingqueueagelimit: i32) -> ::windows_core::Result<()>;
    fn Branding(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(this: &Self::This, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IncomingQueueBlocked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingQueueBlocked(this: &Self::This, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AutoCreateAccountOnConnect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoCreateAccountOnConnect(this: &Self::This, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IncomingFaxesArePublic(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingFaxesArePublic(this: &Self::This, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UseArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusearchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseArchive(this, ::core::mem::transmute_copy(&busearchive)).into())
        }
        unsafe extern "system" fn ArchiveLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrarchivelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArchiveLocation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrarchivelocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetArchiveLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrarchivelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetArchiveLocation(this, ::core::mem::transmute(&bstrarchivelocation)).into())
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeQuotaWarning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsizequotawarning, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSizeQuotaWarning(this, ::core::mem::transmute_copy(&bsizequotawarning)).into())
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HighQuotaWaterMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhighquotawatermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHighQuotaWaterMark(this, ::core::mem::transmute_copy(&lhighquotawatermark)).into())
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LowQuotaWaterMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllowquotawatermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowQuotaWaterMark(this, ::core::mem::transmute_copy(&llowquotawatermark)).into())
        }
        unsafe extern "system" fn ArchiveAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plarchiveagelimit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArchiveAgeLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarchiveagelimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetArchiveAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, larchiveagelimit: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetArchiveAgeLimit(this, ::core::mem::transmute_copy(&larchiveagelimit)).into())
        }
        unsafe extern "system" fn ArchiveSizeLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArchiveSizeLow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ArchiveSizeHigh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArchiveSizeHigh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OutgoingQueueBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboutgoingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingQueueBlocked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboutgoingblocked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutgoingQueueBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutgoingQueueBlocked(this, ::core::mem::transmute_copy(&boutgoingblocked)).into())
        }
        unsafe extern "system" fn OutgoingQueuePaused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboutgoingpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingQueuePaused(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboutgoingpaused, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutgoingQueuePaused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutgoingQueuePaused(this, ::core::mem::transmute_copy(&boutgoingpaused)).into())
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowPersonalCoverPages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pballowpersonalcoverpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowPersonalCoverPages(this, ::core::mem::transmute_copy(&ballowpersonalcoverpages)).into())
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseDeviceTSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusedevicetsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseDeviceTSID(this, ::core::mem::transmute_copy(&busedevicetsid)).into())
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRetries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRetries(this, ::core::mem::transmute_copy(&lretries)).into())
        }
        unsafe extern "system" fn RetryDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetryDelay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretrydelay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRetryDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRetryDelay(this, ::core::mem::transmute_copy(&lretrydelay)).into())
        }
        unsafe extern "system" fn DiscountRateStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiscountRateStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountratestart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiscountRateStart(this, ::core::mem::transmute_copy(&datediscountratestart)).into())
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiscountRateEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountrateend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiscountRateEnd(this, ::core::mem::transmute_copy(&datediscountrateend)).into())
        }
        unsafe extern "system" fn OutgoingQueueAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingQueueAgeLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ploutgoingqueueagelimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutgoingQueueAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loutgoingqueueagelimit: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutgoingQueueAgeLimit(this, ::core::mem::transmute_copy(&loutgoingqueueagelimit)).into())
        }
        unsafe extern "system" fn Branding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Branding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbbranding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBranding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBranding(this, ::core::mem::transmute_copy(&bbranding)).into())
        }
        unsafe extern "system" fn IncomingQueueBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbincomingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingQueueBlocked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbincomingblocked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIncomingQueueBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIncomingQueueBlocked(this, ::core::mem::transmute_copy(&bincomingblocked)).into())
        }
        unsafe extern "system" fn AutoCreateAccountOnConnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbautocreateaccountonconnect: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoCreateAccountOnConnect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbautocreateaccountonconnect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoCreateAccountOnConnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoCreateAccountOnConnect(this, ::core::mem::transmute_copy(&bautocreateaccountonconnect)).into())
        }
        unsafe extern "system" fn IncomingFaxesArePublic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbincomingfaxesarepublic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingFaxesArePublic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbincomingfaxesarepublic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIncomingFaxesArePublic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIncomingFaxesArePublic(this, ::core::mem::transmute_copy(&bincomingfaxesarepublic)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        IFaxConfiguration_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveLocation: ArchiveLocation::<Identity, Impl, OFFSET>,
            SetArchiveLocation: SetArchiveLocation::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            ArchiveAgeLimit: ArchiveAgeLimit::<Identity, Impl, OFFSET>,
            SetArchiveAgeLimit: SetArchiveAgeLimit::<Identity, Impl, OFFSET>,
            ArchiveSizeLow: ArchiveSizeLow::<Identity, Impl, OFFSET>,
            ArchiveSizeHigh: ArchiveSizeHigh::<Identity, Impl, OFFSET>,
            OutgoingQueueBlocked: OutgoingQueueBlocked::<Identity, Impl, OFFSET>,
            SetOutgoingQueueBlocked: SetOutgoingQueueBlocked::<Identity, Impl, OFFSET>,
            OutgoingQueuePaused: OutgoingQueuePaused::<Identity, Impl, OFFSET>,
            SetOutgoingQueuePaused: SetOutgoingQueuePaused::<Identity, Impl, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, Impl, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            SetRetries: SetRetries::<Identity, Impl, OFFSET>,
            RetryDelay: RetryDelay::<Identity, Impl, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, Impl, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, Impl, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, Impl, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, Impl, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, Impl, OFFSET>,
            OutgoingQueueAgeLimit: OutgoingQueueAgeLimit::<Identity, Impl, OFFSET>,
            SetOutgoingQueueAgeLimit: SetOutgoingQueueAgeLimit::<Identity, Impl, OFFSET>,
            Branding: Branding::<Identity, Impl, OFFSET>,
            SetBranding: SetBranding::<Identity, Impl, OFFSET>,
            IncomingQueueBlocked: IncomingQueueBlocked::<Identity, Impl, OFFSET>,
            SetIncomingQueueBlocked: SetIncomingQueueBlocked::<Identity, Impl, OFFSET>,
            AutoCreateAccountOnConnect: AutoCreateAccountOnConnect::<Identity, Impl, OFFSET>,
            SetAutoCreateAccountOnConnect: SetAutoCreateAccountOnConnect::<Identity, Impl, OFFSET>,
            IncomingFaxesArePublic: IncomingFaxesArePublic::<Identity, Impl, OFFSET>,
            SetIncomingFaxesArePublic: SetIncomingFaxesArePublic::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDevice_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProviderUniqueName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PoweredOff(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceivingNow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SendingNow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UsedRoutingMethods(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SendEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSendEnabled(this: &Self::This, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ReceiveMode(this: &Self::This) -> ::windows_core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM>;
    fn SetReceiveMode(this: &Self::This, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_core::Result<()>;
    fn RingsBeforeAnswer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRingsBeforeAnswer(this: &Self::This, lringsbeforeanswer: i32) -> ::windows_core::Result<()>;
    fn CSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCSID(this: &Self::This, bstrcsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTSID(this: &Self::This, bstrtsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetExtensionProperty(this: &Self::This, bstrguid: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtensionProperty(this: &Self::This, bstrguid: &::windows_core::BSTR, vproperty: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn UseRoutingMethod(this: &Self::This, bstrmethodguid: &::windows_core::BSTR, buse: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RingingNow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerCall(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProviderUniqueName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprovideruniquename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderUniqueName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprovideruniquename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PoweredOff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpoweredoff: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PoweredOff(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpoweredoff, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceivingNow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreceivingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceivingNow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbreceivingnow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendingNow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsendingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendingNow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsendingnow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UsedRoutingMethods<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsedRoutingMethods(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvusedroutingmethods, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn SendEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsendenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsendenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSendEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSendEnabled(this, ::core::mem::transmute_copy(&bsendenabled)).into())
        }
        unsafe extern "system" fn ReceiveMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceivemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReceiveMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceiveMode(this, ::core::mem::transmute_copy(&receivemode)).into())
        }
        unsafe extern "system" fn RingsBeforeAnswer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringsbeforeanswer: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RingsBeforeAnswer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plringsbeforeanswer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRingsBeforeAnswer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringsbeforeanswer: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRingsBeforeAnswer(this, ::core::mem::transmute_copy(&lringsbeforeanswer)).into())
        }
        unsafe extern "system" fn CSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCSID(this, ::core::mem::transmute(&bstrcsid)).into())
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTSID(this, ::core::mem::transmute(&bstrtsid)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtensionProperty(this, ::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtensionProperty(this, ::core::mem::transmute(&bstrguid), ::core::mem::transmute(&vproperty)).into())
        }
        unsafe extern "system" fn UseRoutingMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmethodguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, buse: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseRoutingMethod(this, ::core::mem::transmute(&bstrmethodguid), ::core::mem::transmute_copy(&buse)).into())
        }
        unsafe extern "system" fn RingingNow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbringingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RingingNow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbringingnow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AnswerCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnswerCall(this).into())
        }
        IFaxDevice_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            ProviderUniqueName: ProviderUniqueName::<Identity, Impl, OFFSET>,
            PoweredOff: PoweredOff::<Identity, Impl, OFFSET>,
            ReceivingNow: ReceivingNow::<Identity, Impl, OFFSET>,
            SendingNow: SendingNow::<Identity, Impl, OFFSET>,
            UsedRoutingMethods: UsedRoutingMethods::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SendEnabled: SendEnabled::<Identity, Impl, OFFSET>,
            SetSendEnabled: SetSendEnabled::<Identity, Impl, OFFSET>,
            ReceiveMode: ReceiveMode::<Identity, Impl, OFFSET>,
            SetReceiveMode: SetReceiveMode::<Identity, Impl, OFFSET>,
            RingsBeforeAnswer: RingsBeforeAnswer::<Identity, Impl, OFFSET>,
            SetRingsBeforeAnswer: SetRingsBeforeAnswer::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            SetCSID: SetCSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            SetTSID: SetTSID::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, Impl, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, Impl, OFFSET>,
            UseRoutingMethod: UseRoutingMethod::<Identity, Impl, OFFSET>,
            RingingNow: RingingNow::<Identity, Impl, OFFSET>,
            AnswerCall: AnswerCall::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDeviceIds_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<i32>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, ldeviceid: i32) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, lindex: i32) -> ::windows_core::Result<()>;
    fn SetOrder(this: &Self::This, ldeviceid: i32, lneworder: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDeviceIds {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDeviceIds {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldeviceid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&ldeviceid)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&lindex)).into())
        }
        unsafe extern "system" fn SetOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOrder(this, ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&lneworder)).into())
        }
        IFaxDeviceIds_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SetOrder: SetOrder::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDeviceProvider_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImageName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UniqueName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TapiProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MajorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MajorBuild(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorBuild(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Debug(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceIds(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDeviceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDeviceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrimagename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UniqueName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UniqueName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruniquename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TapiProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtapiprovidername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TapiProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtapiprovidername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Debug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Debug(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdebug, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitErrorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitErrorCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pliniterrorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdeviceids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceIds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvdeviceids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxDeviceProvider_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            UniqueName: UniqueName::<Identity, Impl, OFFSET>,
            TapiProviderName: TapiProviderName::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, Impl, OFFSET>,
            DeviceIds: DeviceIds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDeviceProviders_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxDeviceProvider>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDeviceProviders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDeviceProviders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxdeviceprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxdeviceprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxDeviceProviders_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDevices_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxDevice>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_ItemById(this: &Self::This, lid: i32) -> ::windows_core::Result<IFaxDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDevices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDevices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ItemById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lid: i32, ppfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ItemById(this, ::core::mem::transmute_copy(&lid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxDevices_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_ItemById: get_ItemById::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDocument_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Body(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBody(this: &Self::This, bstrbody: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Sender(this: &Self::This) -> ::windows_core::Result<IFaxSender>;
    fn Recipients(this: &Self::This) -> ::windows_core::Result<IFaxRecipients>;
    fn CoverPage(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCoverPage(this: &Self::This, bstrcoverpage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Subject(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubject(this: &Self::This, bstrsubject: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Note(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNote(this: &Self::This, bstrnote: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ScheduleTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetScheduleTime(this: &Self::This, datescheduletime: f64) -> ::windows_core::Result<()>;
    fn ReceiptAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetReceiptAddress(this: &Self::This, bstrreceiptaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DocumentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDocumentName(this: &Self::This, bstrdocumentname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CallHandle(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetCallHandle(this: &Self::This, lcallhandle: i32) -> ::windows_core::Result<()>;
    fn CoverPageType(this: &Self::This) -> ::windows_core::Result<FAX_COVERPAGE_TYPE_ENUM>;
    fn SetCoverPageType(this: &Self::This, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn ScheduleType(this: &Self::This) -> ::windows_core::Result<FAX_SCHEDULE_TYPE_ENUM>;
    fn SetScheduleType(this: &Self::This, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn ReceiptType(this: &Self::This) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetReceiptType(this: &Self::This, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn GroupBroadcastReceipts(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupBroadcastReceipts(this: &Self::This, busegrouping: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn SetPriority(this: &Self::This, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn TapiConnection(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn putref_TapiConnection(this: &Self::This, ptapiconnection: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, bstrfaxservername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ConnectedSubmit(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer>) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AttachFaxToReceipt(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttachFaxToReceipt(this: &Self::This, battachfax: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Body(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbody: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&bstrbody)).into())
        }
        unsafe extern "system" fn Sender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sender(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsender, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Recipients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxrecipients: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recipients(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CoverPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcoverpage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CoverPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcoverpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCoverPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcoverpage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoverPage(this, ::core::mem::transmute(&bstrcoverpage)).into())
        }
        unsafe extern "system" fn Subject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsubject: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubject(this, ::core::mem::transmute(&bstrsubject)).into())
        }
        unsafe extern "system" fn Note<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnote: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Note(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnote, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNote<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnote: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNote(this, ::core::mem::transmute(&bstrnote)).into())
        }
        unsafe extern "system" fn ScheduleTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatescheduletime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduleTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatescheduletime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScheduleTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datescheduletime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScheduleTime(this, ::core::mem::transmute_copy(&datescheduletime)).into())
        }
        unsafe extern "system" fn ReceiptAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreceiptaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReceiptAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreceiptaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceiptAddress(this, ::core::mem::transmute(&bstrreceiptaddress)).into())
        }
        unsafe extern "system" fn DocumentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocumentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDocumentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdocumentname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentName(this, ::core::mem::transmute(&bstrdocumentname)).into())
        }
        unsafe extern "system" fn CallHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCallHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcallhandle: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCallHandle(this, ::core::mem::transmute_copy(&lcallhandle)).into())
        }
        unsafe extern "system" fn CoverPageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CoverPageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoverpagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCoverPageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoverPageType(this, ::core::mem::transmute_copy(&coverpagetype)).into())
        }
        unsafe extern "system" fn ScheduleType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduleType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pscheduletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScheduleType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScheduleType(this, ::core::mem::transmute_copy(&scheduletype)).into())
        }
        unsafe extern "system" fn ReceiptType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceipttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReceiptType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceiptType(this, ::core::mem::transmute_copy(&receipttype)).into())
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusegrouping: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupBroadcastReceipts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusegrouping, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroupBroadcastReceipts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busegrouping: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupBroadcastReceipts(this, ::core::mem::transmute_copy(&busegrouping)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn TapiConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptapiconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TapiConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptapiconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_TapiConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptapiconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_TapiConnection(this, ::windows_core::from_raw_borrowed(&ptapiconnection)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Submit(this, ::core::mem::transmute(&bstrfaxservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfaxoutgoingjobids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectedSubmit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectedSubmit(this, ::windows_core::from_raw_borrowed(&pfaxserver)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfaxoutgoingjobids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AttachFaxToReceipt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbattachfax: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AttachFaxToReceipt(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbattachfax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttachFaxToReceipt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, battachfax: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttachFaxToReceipt(this, ::core::mem::transmute_copy(&battachfax)).into())
        }
        IFaxDocument_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipients: Recipients::<Identity, Impl, OFFSET>,
            CoverPage: CoverPage::<Identity, Impl, OFFSET>,
            SetCoverPage: SetCoverPage::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            Note: Note::<Identity, Impl, OFFSET>,
            SetNote: SetNote::<Identity, Impl, OFFSET>,
            ScheduleTime: ScheduleTime::<Identity, Impl, OFFSET>,
            SetScheduleTime: SetScheduleTime::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            SetReceiptAddress: SetReceiptAddress::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            SetDocumentName: SetDocumentName::<Identity, Impl, OFFSET>,
            CallHandle: CallHandle::<Identity, Impl, OFFSET>,
            SetCallHandle: SetCallHandle::<Identity, Impl, OFFSET>,
            CoverPageType: CoverPageType::<Identity, Impl, OFFSET>,
            SetCoverPageType: SetCoverPageType::<Identity, Impl, OFFSET>,
            ScheduleType: ScheduleType::<Identity, Impl, OFFSET>,
            SetScheduleType: SetScheduleType::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            SetReceiptType: SetReceiptType::<Identity, Impl, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            SetGroupBroadcastReceipts: SetGroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            TapiConnection: TapiConnection::<Identity, Impl, OFFSET>,
            putref_TapiConnection: putref_TapiConnection::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            ConnectedSubmit: ConnectedSubmit::<Identity, Impl, OFFSET>,
            AttachFaxToReceipt: AttachFaxToReceipt::<Identity, Impl, OFFSET>,
            SetAttachFaxToReceipt: SetAttachFaxToReceipt::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDocument2_Impl: ::windows_core::BaseImpl + IFaxDocument_Impl {
    fn SubmissionId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Bodies(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetBodies(this: &Self::This, vbodies: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit2(this: &Self::This, bstrfaxservername: &::windows_core::BSTR, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::Result<()>;
    fn ConnectedSubmit2(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer>, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxDocument2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFaxDocument);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxDocument2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubmissionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubmissionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmissionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bodies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbodies: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bodies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbodies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBodies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbodies: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBodies(this, ::core::mem::transmute(&vbodies)).into())
        }
        unsafe extern "system" fn Submit2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit2(this, ::core::mem::transmute(&bstrfaxservername), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)).into())
        }
        unsafe extern "system" fn ConnectedSubmit2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectedSubmit2(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)).into())
        }
        IFaxDocument2_Vtbl {
            base__: <IFaxDocument as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Bodies: Bodies::<Identity, Impl, OFFSET>,
            SetBodies: SetBodies::<Identity, Impl, OFFSET>,
            Submit2: Submit2::<Identity, Impl, OFFSET>,
            ConnectedSubmit2: ConnectedSubmit2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxEventLogging_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn InitEventsLevel(this: &Self::This) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInitEventsLevel(this: &Self::This, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()>;
    fn InboundEventsLevel(this: &Self::This) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInboundEventsLevel(this: &Self::This, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()>;
    fn OutboundEventsLevel(this: &Self::This) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetOutboundEventsLevel(this: &Self::This, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()>;
    fn GeneralEventsLevel(this: &Self::This) -> ::windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetGeneralEventsLevel(this: &Self::This, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxEventLogging {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxEventLogging {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitEventsLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piniteventlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitEventsLevel(this, ::core::mem::transmute_copy(&initeventlevel)).into())
        }
        unsafe extern "system" fn InboundEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InboundEventsLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinboundeventlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInboundEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInboundEventsLevel(this, ::core::mem::transmute_copy(&inboundeventlevel)).into())
        }
        unsafe extern "system" fn OutboundEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutboundEventsLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutboundeventlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutboundEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutboundEventsLevel(this, ::core::mem::transmute_copy(&outboundeventlevel)).into())
        }
        unsafe extern "system" fn GeneralEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GeneralEventsLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgeneraleventlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGeneralEventsLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGeneralEventsLevel(this, ::core::mem::transmute_copy(&generaleventlevel)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        IFaxEventLogging_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitEventsLevel: InitEventsLevel::<Identity, Impl, OFFSET>,
            SetInitEventsLevel: SetInitEventsLevel::<Identity, Impl, OFFSET>,
            InboundEventsLevel: InboundEventsLevel::<Identity, Impl, OFFSET>,
            SetInboundEventsLevel: SetInboundEventsLevel::<Identity, Impl, OFFSET>,
            OutboundEventsLevel: OutboundEventsLevel::<Identity, Impl, OFFSET>,
            SetOutboundEventsLevel: SetOutboundEventsLevel::<Identity, Impl, OFFSET>,
            GeneralEventsLevel: GeneralEventsLevel::<Identity, Impl, OFFSET>,
            SetGeneralEventsLevel: SetGeneralEventsLevel::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxFolders_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(this: &Self::This) -> ::windows_core::Result<IFaxOutgoingQueue>;
    fn IncomingQueue(this: &Self::This) -> ::windows_core::Result<IFaxIncomingQueue>;
    fn IncomingArchive(this: &Self::This) -> ::windows_core::Result<IFaxIncomingArchive>;
    fn OutgoingArchive(this: &Self::This) -> ::windows_core::Result<IFaxOutgoingArchive>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxFolders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxFolders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OutgoingQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IncomingQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IncomingArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncomingArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingarchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OutgoingArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingarchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxFolders_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OutgoingQueue: OutgoingQueue::<Identity, Impl, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, Impl, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, Impl, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRouting_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetExtensions(this: &Self::This) -> ::windows_core::Result<IFaxInboundRoutingExtensions>;
    fn GetMethods(this: &Self::This) -> ::windows_core::Result<IFaxInboundRoutingMethods>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxInboundRouting {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxInboundRouting {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingextensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingextensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMethods<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingmethods: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMethods(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingmethods, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxInboundRouting_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExtensions: GetExtensions::<Identity, Impl, OFFSET>,
            GetMethods: GetMethods::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingExtension_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImageName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UniqueName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MajorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MajorBuild(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorBuild(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Debug(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Methods(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxInboundRoutingExtension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxInboundRoutingExtension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrimagename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UniqueName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UniqueName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruniquename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Debug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Debug(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdebug, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitErrorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitErrorCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pliniterrorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Methods<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvmethods: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Methods(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvmethods, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxInboundRoutingExtension_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            UniqueName: UniqueName::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, Impl, OFFSET>,
            Methods: Methods::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingExtensions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxInboundRoutingExtension>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxInboundRoutingExtensions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxInboundRoutingExtensions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxinboundroutingextension: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingextension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxInboundRoutingExtensions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingMethod_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GUID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FunctionName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExtensionFriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExtensionImageName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lpriority: i32) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxInboundRoutingMethod {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxInboundRoutingMethod {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FunctionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfunctionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FunctionName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfunctionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtensionFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrextensionfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtensionFriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextensionfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtensionImageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrextensionimagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtensionImageName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextensionimagename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lpriority)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        IFaxInboundRoutingMethod_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            FunctionName: FunctionName::<Identity, Impl, OFFSET>,
            ExtensionFriendlyName: ExtensionFriendlyName::<Identity, Impl, OFFSET>,
            ExtensionImageName: ExtensionImageName::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingMethods_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxInboundRoutingMethod>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxInboundRoutingMethods {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxInboundRoutingMethods {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxinboundroutingmethod: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxInboundRoutingMethods_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingArchive_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(this: &Self::This, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ArchiveFolder(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetArchiveFolder(this: &Self::This, bstrarchivefolder: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SizeQuotaWarning(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(this: &Self::This, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn HighQuotaWaterMark(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(this: &Self::This, lhighquotawatermark: i32) -> ::windows_core::Result<()>;
    fn LowQuotaWaterMark(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(this: &Self::This, llowquotawatermark: i32) -> ::windows_core::Result<()>;
    fn AgeLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAgeLimit(this: &Self::This, lagelimit: i32) -> ::windows_core::Result<()>;
    fn SizeLow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SizeHigh(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMessages(this: &Self::This, lprefetchsize: i32) -> ::windows_core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(this: &Self::This, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingArchive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingArchive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UseArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusearchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseArchive(this, ::core::mem::transmute_copy(&busearchive)).into())
        }
        unsafe extern "system" fn ArchiveFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArchiveFolder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrarchivefolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetArchiveFolder(this, ::core::mem::transmute(&bstrarchivefolder)).into())
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeQuotaWarning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsizequotawarning, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSizeQuotaWarning(this, ::core::mem::transmute_copy(&bsizequotawarning)).into())
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HighQuotaWaterMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhighquotawatermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHighQuotaWaterMark(this, ::core::mem::transmute_copy(&lhighquotawatermark)).into())
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LowQuotaWaterMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllowquotawatermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowQuotaWaterMark(this, ::core::mem::transmute_copy(&llowquotawatermark)).into())
        }
        unsafe extern "system" fn AgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AgeLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plagelimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAgeLimit(this, ::core::mem::transmute_copy(&lagelimit)).into())
        }
        unsafe extern "system" fn SizeLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeLow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeHigh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessages(this, ::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessage(this, ::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxIncomingArchive_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, Impl, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingJob_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Size(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentPage(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(this: &Self::This) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AvailableOperations(this: &Self::This) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TransmissionStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CallerId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RoutingInformation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn JobType(this: &Self::This) -> ::windows_core::Result<FAX_JOB_TYPE_ENUM>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn CopyTiff(this: &Self::This, bstrtiffpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcurrentpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedStatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextendedstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AvailableOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AvailableOperations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pavailableoperations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallerId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcallerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoutingInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoutingInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrroutinginformation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JobType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JobType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjobtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTiff(this, ::core::mem::transmute(&bstrtiffpath)).into())
        }
        IFaxIncomingJob_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Size: Size::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
            JobType: JobType::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingJobs_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxIncomingJob>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingJobs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingJobs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxIncomingJobs_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingMessage_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Pages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Size(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TransmissionStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CallerId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RoutingInformation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CopyTiff(this: &Self::This, bstrtiffpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallerId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcallerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoutingInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoutingInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrroutinginformation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTiff(this, ::core::mem::transmute(&bstrtiffpath)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IFaxIncomingMessage_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingMessage2_Impl: ::windows_core::BaseImpl + IFaxIncomingMessage_Impl {
    fn Subject(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubject(this: &Self::This, bstrsubject: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SenderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSenderName(this: &Self::This, bstrsendername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SenderFaxNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSenderFaxNumber(this: &Self::This, bstrsenderfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HasCoverPage(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHasCoverPage(this: &Self::This, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Recipients(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRecipients(this: &Self::This, bstrrecipients: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn WasReAssigned(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Read(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(this: &Self::This, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ReAssign(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingMessage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFaxIncomingMessage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingMessage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Subject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsubject: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubject(this, ::core::mem::transmute(&bstrsubject)).into())
        }
        unsafe extern "system" fn SenderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsendername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsendername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsendername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderName(this, ::core::mem::transmute(&bstrsendername)).into())
        }
        unsafe extern "system" fn SenderFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsenderfaxnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderFaxNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsenderfaxnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsenderfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderFaxNumber(this, ::core::mem::transmute(&bstrsenderfaxnumber)).into())
        }
        unsafe extern "system" fn HasCoverPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasCoverPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbhascoverpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHasCoverPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHasCoverPage(this, ::core::mem::transmute_copy(&bhascoverpage)).into())
        }
        unsafe extern "system" fn Recipients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrecipients: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recipients(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrecipients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRecipients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrecipients: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecipients(this, ::core::mem::transmute(&bstrrecipients)).into())
        }
        unsafe extern "system" fn WasReAssigned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbwasreassigned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WasReAssigned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbwasreassigned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Read(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRead(this, ::core::mem::transmute_copy(&bread)).into())
        }
        unsafe extern "system" fn ReAssign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReAssign(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        IFaxIncomingMessage2_Vtbl {
            base__: <IFaxIncomingMessage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            SenderName: SenderName::<Identity, Impl, OFFSET>,
            SetSenderName: SetSenderName::<Identity, Impl, OFFSET>,
            SenderFaxNumber: SenderFaxNumber::<Identity, Impl, OFFSET>,
            SetSenderFaxNumber: SetSenderFaxNumber::<Identity, Impl, OFFSET>,
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            SetHasCoverPage: SetHasCoverPage::<Identity, Impl, OFFSET>,
            Recipients: Recipients::<Identity, Impl, OFFSET>,
            SetRecipients: SetRecipients::<Identity, Impl, OFFSET>,
            WasReAssigned: WasReAssigned::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            SetRead: SetRead::<Identity, Impl, OFFSET>,
            ReAssign: ReAssign::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingMessageIterator_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Message(this: &Self::This) -> ::windows_core::Result<IFaxIncomingMessage>;
    fn PrefetchSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrefetchSize(this: &Self::This, lprefetchsize: i32) -> ::windows_core::Result<()>;
    fn AtEOF(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MoveFirst(this: &Self::This) -> ::windows_core::Result<()>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingMessageIterator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingMessageIterator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrefetchSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrefetchSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprefetchsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrefetchSize(this, ::core::mem::transmute_copy(&lprefetchsize)).into())
        }
        unsafe extern "system" fn AtEOF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AtEOF(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbeof, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveFirst(this).into())
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveNext(this).into())
        }
        IFaxIncomingMessageIterator_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Message: Message::<Identity, Impl, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, Impl, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, Impl, OFFSET>,
            AtEOF: AtEOF::<Identity, Impl, OFFSET>,
            MoveFirst: MoveFirst::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingQueue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Blocked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(this: &Self::This, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetJobs(this: &Self::This) -> ::windows_core::Result<IFaxIncomingJobs>;
    fn GetJob(this: &Self::This, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxIncomingQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxIncomingQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Blocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Blocked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbblocked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlocked(this, ::core::mem::transmute_copy(&bblocked)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn GetJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJobs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJob(this, ::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxIncomingQueue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Blocked: Blocked::<Identity, Impl, OFFSET>,
            SetBlocked: SetBlocked::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxJobStatus_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn Pages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Size(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentPage(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExtendedStatusCode(this: &Self::This) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AvailableOperations(this: &Self::This) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn JobType(this: &Self::This) -> ::windows_core::Result<FAX_JOB_TYPE_ENUM>;
    fn ScheduledTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CallerId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RoutingInformation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxJobStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxJobStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcurrentpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedStatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextendedstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AvailableOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AvailableOperations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pavailableoperations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JobType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JobType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjobtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScheduledTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduledTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatescheduledtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallerId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcallerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoutingInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoutingInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrroutinginformation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxJobStatus_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            JobType: JobType::<Identity, Impl, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxLoggingOptions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EventLogging(this: &Self::This) -> ::windows_core::Result<IFaxEventLogging>;
    fn ActivityLogging(this: &Self::This) -> ::windows_core::Result<IFaxActivityLogging>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxLoggingOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxLoggingOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventLogging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxeventlogging: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventLogging(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxeventlogging, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivityLogging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxactivitylogging: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivityLogging(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxactivitylogging, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxLoggingOptions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventLogging: EventLogging::<Identity, Impl, OFFSET>,
            ActivityLogging: ActivityLogging::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRouting_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetGroups(this: &Self::This) -> ::windows_core::Result<IFaxOutboundRoutingGroups>;
    fn GetRules(this: &Self::This) -> ::windows_core::Result<IFaxOutboundRoutingRules>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutboundRouting {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutboundRouting {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutinggroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutinggroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutingrules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutboundRouting_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGroups: GetGroups::<Identity, Impl, OFFSET>,
            GetRules: GetRules::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingGroup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_GROUP_STATUS_ENUM>;
    fn DeviceIds(this: &Self::This) -> ::windows_core::Result<IFaxDeviceIds>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutboundRoutingGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutboundRoutingGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxdeviceids: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceIds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxdeviceids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutboundRoutingGroup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            DeviceIds: DeviceIds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingGroups_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxOutboundRoutingGroup>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IFaxOutboundRoutingGroup>;
    fn Remove(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutboundRoutingGroups {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutboundRoutingGroups {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutinggroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutinggroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&vindex)).into())
        }
        IFaxOutboundRoutingGroups_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingRule_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CountryCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AreaCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_RULE_STATUS_ENUM>;
    fn UseDevice(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDevice(this: &Self::This, busedevice: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDeviceId(this: &Self::This, deviceid: i32) -> ::windows_core::Result<()>;
    fn GroupName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetGroupName(this: &Self::This, bstrgroupname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutboundRoutingRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutboundRoutingRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CountryCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CountryCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcountrycode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AreaCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plareacode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AreaCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plareacode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UseDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusedevice: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusedevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busedevice: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseDevice(this, ::core::mem::transmute_copy(&busedevice)).into())
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceId(this, ::core::mem::transmute_copy(&deviceid)).into())
        }
        unsafe extern "system" fn GroupName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrgroupname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroupName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupName(this, ::core::mem::transmute(&bstrgroupname)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        IFaxOutboundRoutingRule_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CountryCode: CountryCode::<Identity, Impl, OFFSET>,
            AreaCode: AreaCode::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            UseDevice: UseDevice::<Identity, Impl, OFFSET>,
            SetUseDevice: SetUseDevice::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            GroupName: GroupName::<Identity, Impl, OFFSET>,
            SetGroupName: SetGroupName::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingRules_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<IFaxOutboundRoutingRule>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ItemByCountryAndArea(this: &Self::This, lcountrycode: i32, lareacode: i32) -> ::windows_core::Result<IFaxOutboundRoutingRule>;
    fn RemoveByCountryAndArea(this: &Self::This, lcountrycode: i32, lareacode: i32) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, lindex: i32) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: &::windows_core::BSTR, ldeviceid: i32) -> ::windows_core::Result<IFaxOutboundRoutingRule>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutboundRoutingRules {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutboundRoutingRules {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemByCountryAndArea<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemByCountryAndArea(this, ::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveByCountryAndArea<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveByCountryAndArea(this, ::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&lindex)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode), ::core::mem::transmute_copy(&busedevice), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute_copy(&ldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutboundRoutingRules_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ItemByCountryAndArea: ItemByCountryAndArea::<Identity, Impl, OFFSET>,
            RemoveByCountryAndArea: RemoveByCountryAndArea::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingArchive_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(this: &Self::This, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ArchiveFolder(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetArchiveFolder(this: &Self::This, bstrarchivefolder: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SizeQuotaWarning(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(this: &Self::This, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn HighQuotaWaterMark(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(this: &Self::This, lhighquotawatermark: i32) -> ::windows_core::Result<()>;
    fn LowQuotaWaterMark(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(this: &Self::This, llowquotawatermark: i32) -> ::windows_core::Result<()>;
    fn AgeLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAgeLimit(this: &Self::This, lagelimit: i32) -> ::windows_core::Result<()>;
    fn SizeLow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SizeHigh(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMessages(this: &Self::This, lprefetchsize: i32) -> ::windows_core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(this: &Self::This, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingArchive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingArchive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UseArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseArchive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusearchive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseArchive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseArchive(this, ::core::mem::transmute_copy(&busearchive)).into())
        }
        unsafe extern "system" fn ArchiveFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArchiveFolder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrarchivefolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetArchiveFolder(this, ::core::mem::transmute(&bstrarchivefolder)).into())
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeQuotaWarning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsizequotawarning, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSizeQuotaWarning(this, ::core::mem::transmute_copy(&bsizequotawarning)).into())
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HighQuotaWaterMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhighquotawatermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHighQuotaWaterMark(this, ::core::mem::transmute_copy(&lhighquotawatermark)).into())
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LowQuotaWaterMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllowquotawatermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowQuotaWaterMark(this, ::core::mem::transmute_copy(&llowquotawatermark)).into())
        }
        unsafe extern "system" fn AgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AgeLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plagelimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAgeLimit(this, ::core::mem::transmute_copy(&lagelimit)).into())
        }
        unsafe extern "system" fn SizeLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeLow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeHigh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessages(this, ::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessage(this, ::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutgoingArchive_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, Impl, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingJob_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Subject(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DocumentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Pages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Size(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SubmissionId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn OriginalScheduledTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SubmissionTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ReceiptType(this: &Self::This) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(this: &Self::This) -> ::windows_core::Result<IFaxSender>;
    fn Recipient(this: &Self::This) -> ::windows_core::Result<IFaxRecipient>;
    fn CurrentPage(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(this: &Self::This) -> ::windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AvailableOperations(this: &Self::This) -> ::windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ScheduledTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GroupBroadcastReceipts(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Restart(this: &Self::This) -> ::windows_core::Result<()>;
    fn CopyTiff(this: &Self::This, bstrtiffpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Subject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DocumentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocumentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubmissionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubmissionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmissionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OriginalScheduledTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdateoriginalscheduledtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubmissionTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatesubmissiontime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiptType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceipttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Sender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sender(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsender, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Recipient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recipient(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcurrentpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedStatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextendedstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AvailableOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AvailableOperations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pavailableoperations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScheduledTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduledTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatescheduledtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgroupbroadcastreceipts: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupBroadcastReceipts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbgroupbroadcastreceipts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Restart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restart(this).into())
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTiff(this, ::core::mem::transmute(&bstrtiffpath)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IFaxOutgoingJob_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Subject: Subject::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipient: Recipient::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Restart: Restart::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingJob2_Impl: ::windows_core::BaseImpl + IFaxOutgoingJob_Impl {
    fn HasCoverPage(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ScheduleType(this: &Self::This) -> ::windows_core::Result<FAX_SCHEDULE_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingJob2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFaxOutgoingJob);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingJob2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HasCoverPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasCoverPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbhascoverpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiptAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreceiptaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScheduleType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduleType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pscheduletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutgoingJob2_Vtbl {
            base__: <IFaxOutgoingJob as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            ScheduleType: ScheduleType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingJobs_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, vindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFaxOutgoingJob>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingJobs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingJobs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutgoingJobs_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingMessage_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SubmissionId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Subject(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DocumentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Pages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Size(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OriginalScheduledTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SubmissionTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(this: &Self::This) -> ::windows_core::Result<IFaxSender>;
    fn Recipient(this: &Self::This) -> ::windows_core::Result<IFaxRecipient>;
    fn DeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TransmissionStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CopyTiff(this: &Self::This, bstrtiffpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubmissionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubmissionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmissionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Subject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DocumentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocumentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OriginalScheduledTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdateoriginalscheduledtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubmissionTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatesubmissiontime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Sender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sender(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsender, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Recipient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recipient(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTiff(this, ::core::mem::transmute(&bstrtiffpath)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IFaxOutgoingMessage_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipient: Recipient::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingMessage2_Impl: ::windows_core::BaseImpl + IFaxOutgoingMessage_Impl {
    fn HasCoverPage(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptType(this: &Self::This) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn ReceiptAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Read(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(this: &Self::This, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingMessage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFaxOutgoingMessage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingMessage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HasCoverPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasCoverPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbhascoverpage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiptType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceipttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiptAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreceiptaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Read(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRead(this, ::core::mem::transmute_copy(&bread)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        IFaxOutgoingMessage2_Vtbl {
            base__: <IFaxOutgoingMessage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            SetRead: SetRead::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingMessageIterator_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Message(this: &Self::This) -> ::windows_core::Result<IFaxOutgoingMessage>;
    fn AtEOF(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PrefetchSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrefetchSize(this: &Self::This, lprefetchsize: i32) -> ::windows_core::Result<()>;
    fn MoveFirst(this: &Self::This) -> ::windows_core::Result<()>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingMessageIterator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingMessageIterator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AtEOF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AtEOF(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbeof, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrefetchSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrefetchSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprefetchsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrefetchSize(this, ::core::mem::transmute_copy(&lprefetchsize)).into())
        }
        unsafe extern "system" fn MoveFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveFirst(this).into())
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveNext(this).into())
        }
        IFaxOutgoingMessageIterator_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Message: Message::<Identity, Impl, OFFSET>,
            AtEOF: AtEOF::<Identity, Impl, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, Impl, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, Impl, OFFSET>,
            MoveFirst: MoveFirst::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingQueue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Blocked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(this: &Self::This, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Paused(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPaused(this: &Self::This, bpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowPersonalCoverPages(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(this: &Self::This, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseDeviceTSID(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(this: &Self::This, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Retries(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRetries(this: &Self::This, lretries: i32) -> ::windows_core::Result<()>;
    fn RetryDelay(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRetryDelay(this: &Self::This, lretrydelay: i32) -> ::windows_core::Result<()>;
    fn DiscountRateStart(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetDiscountRateStart(this: &Self::This, datediscountratestart: f64) -> ::windows_core::Result<()>;
    fn DiscountRateEnd(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetDiscountRateEnd(this: &Self::This, datediscountrateend: f64) -> ::windows_core::Result<()>;
    fn AgeLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAgeLimit(this: &Self::This, lagelimit: i32) -> ::windows_core::Result<()>;
    fn Branding(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(this: &Self::This, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetJobs(this: &Self::This) -> ::windows_core::Result<IFaxOutgoingJobs>;
    fn GetJob(this: &Self::This, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxOutgoingQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxOutgoingQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Blocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Blocked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbblocked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlocked(this, ::core::mem::transmute_copy(&bblocked)).into())
        }
        unsafe extern "system" fn Paused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Paused(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpaused, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPaused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPaused(this, ::core::mem::transmute_copy(&bpaused)).into())
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowPersonalCoverPages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pballowpersonalcoverpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowPersonalCoverPages(this, ::core::mem::transmute_copy(&ballowpersonalcoverpages)).into())
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseDeviceTSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusedevicetsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseDeviceTSID(this, ::core::mem::transmute_copy(&busedevicetsid)).into())
        }
        unsafe extern "system" fn Retries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRetries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRetries(this, ::core::mem::transmute_copy(&lretries)).into())
        }
        unsafe extern "system" fn RetryDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetryDelay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretrydelay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRetryDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRetryDelay(this, ::core::mem::transmute_copy(&lretrydelay)).into())
        }
        unsafe extern "system" fn DiscountRateStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiscountRateStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountratestart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiscountRateStart(this, ::core::mem::transmute_copy(&datediscountratestart)).into())
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiscountRateEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountrateend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiscountRateEnd(this, ::core::mem::transmute_copy(&datediscountrateend)).into())
        }
        unsafe extern "system" fn AgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AgeLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plagelimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAgeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAgeLimit(this, ::core::mem::transmute_copy(&lagelimit)).into())
        }
        unsafe extern "system" fn Branding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Branding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbbranding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBranding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBranding(this, ::core::mem::transmute_copy(&bbranding)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn GetJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJobs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJob(this, ::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxOutgoingQueue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Blocked: Blocked::<Identity, Impl, OFFSET>,
            SetBlocked: SetBlocked::<Identity, Impl, OFFSET>,
            Paused: Paused::<Identity, Impl, OFFSET>,
            SetPaused: SetPaused::<Identity, Impl, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, Impl, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            SetRetries: SetRetries::<Identity, Impl, OFFSET>,
            RetryDelay: RetryDelay::<Identity, Impl, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, Impl, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, Impl, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, Impl, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, Impl, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            Branding: Branding::<Identity, Impl, OFFSET>,
            SetBranding: SetBranding::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxReceiptOptions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AuthenticationType(this: &Self::This) -> ::windows_core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>;
    fn SetAuthenticationType(this: &Self::This, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn SMTPServer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSMTPServer(this: &Self::This, bstrsmtpserver: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SMTPPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSMTPPort(this: &Self::This, lsmtpport: i32) -> ::windows_core::Result<()>;
    fn SMTPSender(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSMTPSender(this: &Self::This, bstrsmtpsender: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SMTPUser(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSMTPUser(this: &Self::This, bstrsmtpuser: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AllowedReceipts(this: &Self::This) -> ::windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetAllowedReceipts(this: &Self::This, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn SMTPPassword(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSMTPPassword(this: &Self::This, bstrsmtppassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn UseForInboundRouting(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseForInboundRouting(this: &Self::This, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxReceiptOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxReceiptOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AuthenticationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationType(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn SMTPServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsmtpserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SMTPServer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtpserver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSMTPServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsmtpserver: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSMTPServer(this, ::core::mem::transmute(&bstrsmtpserver)).into())
        }
        unsafe extern "system" fn SMTPPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsmtpport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SMTPPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsmtpport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSMTPPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsmtpport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSMTPPort(this, ::core::mem::transmute_copy(&lsmtpport)).into())
        }
        unsafe extern "system" fn SMTPSender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsmtpsender: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SMTPSender(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtpsender, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSMTPSender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsmtpsender: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSMTPSender(this, ::core::mem::transmute(&bstrsmtpsender)).into())
        }
        unsafe extern "system" fn SMTPUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsmtpuser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SMTPUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtpuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSMTPUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsmtpuser: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSMTPUser(this, ::core::mem::transmute(&bstrsmtpuser)).into())
        }
        unsafe extern "system" fn AllowedReceipts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowedReceipts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallowedreceipts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowedReceipts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowedReceipts(this, ::core::mem::transmute_copy(&allowedreceipts)).into())
        }
        unsafe extern "system" fn SMTPPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsmtppassword: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SMTPPassword(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtppassword, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSMTPPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsmtppassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSMTPPassword(this, ::core::mem::transmute(&bstrsmtppassword)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn UseForInboundRouting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuseforinboundrouting: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseForInboundRouting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbuseforinboundrouting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseForInboundRouting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseForInboundRouting(this, ::core::mem::transmute_copy(&buseforinboundrouting)).into())
        }
        IFaxReceiptOptions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AuthenticationType: AuthenticationType::<Identity, Impl, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, Impl, OFFSET>,
            SMTPServer: SMTPServer::<Identity, Impl, OFFSET>,
            SetSMTPServer: SetSMTPServer::<Identity, Impl, OFFSET>,
            SMTPPort: SMTPPort::<Identity, Impl, OFFSET>,
            SetSMTPPort: SetSMTPPort::<Identity, Impl, OFFSET>,
            SMTPSender: SMTPSender::<Identity, Impl, OFFSET>,
            SetSMTPSender: SetSMTPSender::<Identity, Impl, OFFSET>,
            SMTPUser: SMTPUser::<Identity, Impl, OFFSET>,
            SetSMTPUser: SetSMTPUser::<Identity, Impl, OFFSET>,
            AllowedReceipts: AllowedReceipts::<Identity, Impl, OFFSET>,
            SetAllowedReceipts: SetAllowedReceipts::<Identity, Impl, OFFSET>,
            SMTPPassword: SMTPPassword::<Identity, Impl, OFFSET>,
            SetSMTPPassword: SetSMTPPassword::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            UseForInboundRouting: UseForInboundRouting::<Identity, Impl, OFFSET>,
            SetUseForInboundRouting: SetUseForInboundRouting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxRecipient_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn FaxNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFaxNumber(this: &Self::This, bstrfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxRecipient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxRecipient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FaxNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfaxnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFaxNumber(this, ::core::mem::transmute(&bstrfaxnumber)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        IFaxRecipient_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxRecipients_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<IFaxRecipient>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, bstrfaxnumber: &::windows_core::BSTR, bstrrecipientname: &::windows_core::BSTR) -> ::windows_core::Result<IFaxRecipient>;
    fn Remove(this: &Self::This, lindex: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxRecipients {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxRecipients {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrecipientname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&bstrfaxnumber), ::core::mem::transmute(&bstrrecipientname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&lindex)).into())
        }
        IFaxRecipients_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxSecurity_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDescriptor(this: &Self::This, vdescriptor: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GrantedRights(this: &Self::This) -> ::windows_core::Result<FAX_ACCESS_RIGHTS_ENUM>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn InformationType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetInformationType(this: &Self::This, linformationtype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Descriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Descriptor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvdescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vdescriptor: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescriptor(this, ::core::mem::transmute(&vdescriptor)).into())
        }
        unsafe extern "system" fn GrantedRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GrantedRights(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrantedrights, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn InformationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InformationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plinformationtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInformationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInformationType(this, ::core::mem::transmute_copy(&linformationtype)).into())
        }
        IFaxSecurity_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Descriptor: Descriptor::<Identity, Impl, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, Impl, OFFSET>,
            GrantedRights: GrantedRights::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            InformationType: InformationType::<Identity, Impl, OFFSET>,
            SetInformationType: SetInformationType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxSecurity2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDescriptor(this: &Self::This, vdescriptor: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GrantedRights(this: &Self::This) -> ::windows_core::Result<FAX_ACCESS_RIGHTS_ENUM_2>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn InformationType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetInformationType(this: &Self::This, linformationtype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxSecurity2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxSecurity2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Descriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Descriptor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvdescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vdescriptor: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescriptor(this, ::core::mem::transmute(&vdescriptor)).into())
        }
        unsafe extern "system" fn GrantedRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GrantedRights(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrantedrights, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn InformationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InformationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plinformationtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInformationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInformationType(this, ::core::mem::transmute_copy(&linformationtype)).into())
        }
        IFaxSecurity2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Descriptor: Descriptor::<Identity, Impl, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, Impl, OFFSET>,
            GrantedRights: GrantedRights::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            InformationType: InformationType::<Identity, Impl, OFFSET>,
            SetInformationType: SetInformationType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxSender_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn BillingCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBillingCode(this: &Self::This, bstrbillingcode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn City(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCity(this: &Self::This, bstrcity: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Company(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCompany(this: &Self::This, bstrcompany: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Country(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCountry(this: &Self::This, bstrcountry: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Department(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDepartment(this: &Self::This, bstrdepartment: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Email(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEmail(this: &Self::This, bstremail: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FaxNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFaxNumber(this: &Self::This, bstrfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HomePhone(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHomePhone(this: &Self::This, bstrhomephone: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTSID(this: &Self::This, bstrtsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OfficePhone(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOfficePhone(this: &Self::This, bstrofficephone: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OfficeLocation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOfficeLocation(this: &Self::This, bstrofficelocation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetState(this: &Self::This, bstrstate: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StreetAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStreetAddress(this: &Self::This, bstrstreetaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTitle(this: &Self::This, bstrtitle: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ZipCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetZipCode(this: &Self::This, bstrzipcode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoadDefaultSender(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveDefaultSender(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxSender {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxSender {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BillingCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbillingcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BillingCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbillingcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBillingCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbillingcode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBillingCode(this, ::core::mem::transmute(&bstrbillingcode)).into())
        }
        unsafe extern "system" fn City<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::City(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcity: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCity(this, ::core::mem::transmute(&bstrcity)).into())
        }
        unsafe extern "system" fn Company<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcompany: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Company(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcompany, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompany<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcompany: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompany(this, ::core::mem::transmute(&bstrcompany)).into())
        }
        unsafe extern "system" fn Country<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcountry: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Country(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcountry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCountry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcountry: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCountry(this, ::core::mem::transmute(&bstrcountry)).into())
        }
        unsafe extern "system" fn Department<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdepartment: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Department(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdepartment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDepartment(this, ::core::mem::transmute(&bstrdepartment)).into())
        }
        unsafe extern "system" fn Email<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstremail: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Email(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstremail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEmail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstremail: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEmail(this, ::core::mem::transmute(&bstremail)).into())
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FaxNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfaxnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFaxNumber(this, ::core::mem::transmute(&bstrfaxnumber)).into())
        }
        unsafe extern "system" fn HomePhone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrhomephone: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HomePhone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhomephone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHomePhone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhomephone: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHomePhone(this, ::core::mem::transmute(&bstrhomephone)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn TSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTSID(this, ::core::mem::transmute(&bstrtsid)).into())
        }
        unsafe extern "system" fn OfficePhone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrofficephone: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OfficePhone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrofficephone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOfficePhone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrofficephone: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOfficePhone(this, ::core::mem::transmute(&bstrofficephone)).into())
        }
        unsafe extern "system" fn OfficeLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrofficelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OfficeLocation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrofficelocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOfficeLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrofficelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOfficeLocation(this, ::core::mem::transmute(&bstrofficelocation)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrstate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute(&bstrstate)).into())
        }
        unsafe extern "system" fn StreetAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstreetaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StreetAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstreetaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStreetAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrstreetaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreetAddress(this, ::core::mem::transmute(&bstrstreetaddress)).into())
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&bstrtitle)).into())
        }
        unsafe extern "system" fn ZipCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrzipcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ZipCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrzipcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetZipCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrzipcode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZipCode(this, ::core::mem::transmute(&bstrzipcode)).into())
        }
        unsafe extern "system" fn LoadDefaultSender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadDefaultSender(this).into())
        }
        unsafe extern "system" fn SaveDefaultSender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveDefaultSender(this).into())
        }
        IFaxSender_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BillingCode: BillingCode::<Identity, Impl, OFFSET>,
            SetBillingCode: SetBillingCode::<Identity, Impl, OFFSET>,
            City: City::<Identity, Impl, OFFSET>,
            SetCity: SetCity::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            SetCompany: SetCompany::<Identity, Impl, OFFSET>,
            Country: Country::<Identity, Impl, OFFSET>,
            SetCountry: SetCountry::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Email: Email::<Identity, Impl, OFFSET>,
            SetEmail: SetEmail::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            HomePhone: HomePhone::<Identity, Impl, OFFSET>,
            SetHomePhone: SetHomePhone::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            SetTSID: SetTSID::<Identity, Impl, OFFSET>,
            OfficePhone: OfficePhone::<Identity, Impl, OFFSET>,
            SetOfficePhone: SetOfficePhone::<Identity, Impl, OFFSET>,
            OfficeLocation: OfficeLocation::<Identity, Impl, OFFSET>,
            SetOfficeLocation: SetOfficeLocation::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            StreetAddress: StreetAddress::<Identity, Impl, OFFSET>,
            SetStreetAddress: SetStreetAddress::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            ZipCode: ZipCode::<Identity, Impl, OFFSET>,
            SetZipCode: SetZipCode::<Identity, Impl, OFFSET>,
            LoadDefaultSender: LoadDefaultSender::<Identity, Impl, OFFSET>,
            SaveDefaultSender: SaveDefaultSender::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServer_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Connect(this: &Self::This, bstrservername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDeviceProviders(this: &Self::This) -> ::windows_core::Result<IFaxDeviceProviders>;
    fn GetDevices(this: &Self::This) -> ::windows_core::Result<IFaxDevices>;
    fn InboundRouting(this: &Self::This) -> ::windows_core::Result<IFaxInboundRouting>;
    fn Folders(this: &Self::This) -> ::windows_core::Result<IFaxFolders>;
    fn LoggingOptions(this: &Self::This) -> ::windows_core::Result<IFaxLoggingOptions>;
    fn MajorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MajorBuild(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorBuild(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Debug(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Activity(this: &Self::This) -> ::windows_core::Result<IFaxActivity>;
    fn OutboundRouting(this: &Self::This) -> ::windows_core::Result<IFaxOutboundRouting>;
    fn ReceiptOptions(this: &Self::This) -> ::windows_core::Result<IFaxReceiptOptions>;
    fn Security(this: &Self::This) -> ::windows_core::Result<IFaxSecurity>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetExtensionProperty(this: &Self::This, bstrguid: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtensionProperty(this: &Self::This, bstrguid: &::windows_core::BSTR, vproperty: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ListenToServerEvents(this: &Self::This, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::Result<()>;
    fn RegisterDeviceProvider(this: &Self::This, bstrguid: &::windows_core::BSTR, bstrfriendlyname: &::windows_core::BSTR, bstrimagename: &::windows_core::BSTR, tspname: &::windows_core::BSTR, lfspiversion: i32) -> ::windows_core::Result<()>;
    fn UnregisterDeviceProvider(this: &Self::This, bstruniquename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RegisterInboundRoutingExtension(this: &Self::This, bstrextensionname: &::windows_core::BSTR, bstrfriendlyname: &::windows_core::BSTR, bstrimagename: &::windows_core::BSTR, vmethods: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn UnregisterInboundRoutingExtension(this: &Self::This, bstrextensionuniquename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RegisteredEvents(this: &Self::This) -> ::windows_core::Result<FAX_SERVER_EVENTS_TYPE_ENUM>;
    fn APIVersion(this: &Self::This) -> ::windows_core::Result<FAX_SERVER_APIVERSION_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxServer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxServer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute(&bstrservername)).into())
        }
        unsafe extern "system" fn ServerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrservername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrservername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxdeviceproviders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxdeviceproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxdevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InboundRouting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxinboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InboundRouting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxinboundrouting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Folders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxfolders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Folders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxfolders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoggingOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxloggingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoggingOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxloggingoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Debug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Debug(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdebug, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Activity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OutboundRouting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxoutboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutboundRouting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxoutboundrouting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiptOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxreceiptoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiptOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxreceiptoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtensionProperty(this, ::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtensionProperty(this, ::core::mem::transmute(&bstrguid), ::core::mem::transmute(&vproperty)).into())
        }
        unsafe extern "system" fn ListenToServerEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ListenToServerEvents(this, ::core::mem::transmute_copy(&eventtypes)).into())
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrimagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, tspname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lfspiversion: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterDeviceProvider(this, ::core::mem::transmute(&bstrguid), ::core::mem::transmute(&bstrfriendlyname), ::core::mem::transmute(&bstrimagename), ::core::mem::transmute(&tspname), ::core::mem::transmute_copy(&lfspiversion)).into())
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstruniquename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDeviceProvider(this, ::core::mem::transmute(&bstruniquename)).into())
        }
        unsafe extern "system" fn RegisterInboundRoutingExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrextensionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrimagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, vmethods: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterInboundRoutingExtension(this, ::core::mem::transmute(&bstrextensionname), ::core::mem::transmute(&bstrfriendlyname), ::core::mem::transmute(&bstrimagename), ::core::mem::transmute(&vmethods)).into())
        }
        unsafe extern "system" fn UnregisterInboundRoutingExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrextensionuniquename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterInboundRoutingExtension(this, ::core::mem::transmute(&bstrextensionuniquename)).into())
        }
        unsafe extern "system" fn RegisteredEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisteredEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn APIVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::APIVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papiversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxServer_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            GetDeviceProviders: GetDeviceProviders::<Identity, Impl, OFFSET>,
            GetDevices: GetDevices::<Identity, Impl, OFFSET>,
            InboundRouting: InboundRouting::<Identity, Impl, OFFSET>,
            Folders: Folders::<Identity, Impl, OFFSET>,
            LoggingOptions: LoggingOptions::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Activity: Activity::<Identity, Impl, OFFSET>,
            OutboundRouting: OutboundRouting::<Identity, Impl, OFFSET>,
            ReceiptOptions: ReceiptOptions::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, Impl, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, Impl, OFFSET>,
            ListenToServerEvents: ListenToServerEvents::<Identity, Impl, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, Impl, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, Impl, OFFSET>,
            RegisterInboundRoutingExtension: RegisterInboundRoutingExtension::<Identity, Impl, OFFSET>,
            UnregisterInboundRoutingExtension: UnregisterInboundRoutingExtension::<Identity, Impl, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, Impl, OFFSET>,
            APIVersion: APIVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServer2_Impl: ::windows_core::BaseImpl + IFaxServer_Impl {
    fn Configuration(this: &Self::This) -> ::windows_core::Result<IFaxConfiguration>;
    fn CurrentAccount(this: &Self::This) -> ::windows_core::Result<IFaxAccount>;
    fn FaxAccountSet(this: &Self::This) -> ::windows_core::Result<IFaxAccountSet>;
    fn Security2(this: &Self::This) -> ::windows_core::Result<IFaxSecurity2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxServer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFaxServer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxServer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Configuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Configuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxconfiguration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcurrentaccount: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAccount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcurrentaccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FaxAccountSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxaccountset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FaxAccountSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxaccountset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfaxsecurity2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsecurity2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFaxServer2_Vtbl {
            base__: <IFaxServer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Configuration: Configuration::<Identity, Impl, OFFSET>,
            CurrentAccount: CurrentAccount::<Identity, Impl, OFFSET>,
            FaxAccountSet: FaxAccountSet::<Identity, Impl, OFFSET>,
            Security2: Security2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServerNotify_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxServerNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxServerNotify {
    const VTABLE: Self::Vtable = { IFaxServerNotify_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServerNotify2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnIncomingJobRemoved(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnIncomingJobChanged(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows_core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows_core::Result<()>;
    fn OnOutgoingJobAdded(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingJobRemoved(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingJobChanged(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows_core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows_core::Result<()>;
    fn OnIncomingMessageAdded(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnIncomingMessageRemoved(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingMessageAdded(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnOutgoingMessageRemoved(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnReceiptOptionsChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnActivityLoggingConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnSecurityConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnEventLoggingConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnOutgoingQueueConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnOutgoingArchiveConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnIncomingArchiveConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnDevicesConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnOutboundRoutingGroupsConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnOutboundRoutingRulesConfigChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnServerActivityChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows_core::Result<()>;
    fn OnQueuesStatusChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn OnNewCall(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, lcallid: i32, ldeviceid: i32, bstrcallerid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnServerShutDown(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
    fn OnDeviceStatusChange(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn OnGeneralServerConfigChanged(this: &Self::This, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFaxServerNotify2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFaxServerNotify2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingJobAdded(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingJobRemoved(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingJobChanged(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid), ::windows_core::from_raw_borrowed(&pjobstatus)).into())
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingJobAdded(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingJobRemoved(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into())
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingJobChanged(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid), ::windows_core::from_raw_borrowed(&pjobstatus)).into())
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingMessageAdded(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into())
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingMessageRemoved(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into())
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingMessageAdded(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into())
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingMessageRemoved(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into())
        }
        unsafe extern "system" fn OnReceiptOptionsChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReceiptOptionsChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnActivityLoggingConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityLoggingConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnSecurityConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSecurityConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnEventLoggingConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEventLoggingConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnOutgoingQueueConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingQueueConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnOutgoingArchiveConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutgoingArchiveConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnIncomingArchiveConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIncomingArchiveConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnDevicesConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDevicesConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnOutboundRoutingGroupsConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutboundRoutingGroupsConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnOutboundRoutingRulesConfigChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOutboundRoutingRulesConfigChange(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnServerActivityChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnServerActivityChange(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&lincomingmessages), ::core::mem::transmute_copy(&lroutingmessages), ::core::mem::transmute_copy(&loutgoingmessages), ::core::mem::transmute_copy(&lqueuedmessages)).into())
        }
        unsafe extern "system" fn OnQueuesStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQueuesStatusChange(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&boutgoingqueueblocked), ::core::mem::transmute_copy(&boutgoingqueuepaused), ::core::mem::transmute_copy(&bincomingqueueblocked)).into())
        }
        unsafe extern "system" fn OnNewCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, lcallid: i32, ldeviceid: i32, bstrcallerid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNewCall(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&lcallid), ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute(&bstrcallerid)).into())
        }
        unsafe extern "system" fn OnServerShutDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnServerShutDown(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        unsafe extern "system" fn OnDeviceStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDeviceStatusChange(this, ::windows_core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&bpoweredoff), ::core::mem::transmute_copy(&bsending), ::core::mem::transmute_copy(&breceiving), ::core::mem::transmute_copy(&bringing)).into())
        }
        unsafe extern "system" fn OnGeneralServerConfigChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnGeneralServerConfigChanged(this, ::windows_core::from_raw_borrowed(&pfaxserver)).into())
        }
        IFaxServerNotify2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, Impl, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, Impl, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, Impl, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, Impl, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, Impl, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, Impl, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, Impl, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, Impl, OFFSET>,
            OnReceiptOptionsChange: OnReceiptOptionsChange::<Identity, Impl, OFFSET>,
            OnActivityLoggingConfigChange: OnActivityLoggingConfigChange::<Identity, Impl, OFFSET>,
            OnSecurityConfigChange: OnSecurityConfigChange::<Identity, Impl, OFFSET>,
            OnEventLoggingConfigChange: OnEventLoggingConfigChange::<Identity, Impl, OFFSET>,
            OnOutgoingQueueConfigChange: OnOutgoingQueueConfigChange::<Identity, Impl, OFFSET>,
            OnOutgoingArchiveConfigChange: OnOutgoingArchiveConfigChange::<Identity, Impl, OFFSET>,
            OnIncomingArchiveConfigChange: OnIncomingArchiveConfigChange::<Identity, Impl, OFFSET>,
            OnDevicesConfigChange: OnDevicesConfigChange::<Identity, Impl, OFFSET>,
            OnOutboundRoutingGroupsConfigChange: OnOutboundRoutingGroupsConfigChange::<Identity, Impl, OFFSET>,
            OnOutboundRoutingRulesConfigChange: OnOutboundRoutingRulesConfigChange::<Identity, Impl, OFFSET>,
            OnServerActivityChange: OnServerActivityChange::<Identity, Impl, OFFSET>,
            OnQueuesStatusChange: OnQueuesStatusChange::<Identity, Impl, OFFSET>,
            OnNewCall: OnNewCall::<Identity, Impl, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, Impl, OFFSET>,
            OnDeviceStatusChange: OnDeviceStatusChange::<Identity, Impl, OFFSET>,
            OnGeneralServerConfigChanged: OnGeneralServerConfigChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub trait IStiDevice_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: &::windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows_core::Result<()>;
    fn GetCapabilities(this: &Self::This, pdevcaps: *mut STI_DEV_CAPS) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::Result<()>;
    fn DeviceReset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Diagnostic(this: &Self::This, pbuffer: *mut STI_DIAG) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::Result<()>;
    fn GetLastError(this: &Self::This) -> ::windows_core::Result<u32>;
    fn LockDevice(this: &Self::This, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn UnLockDevice(this: &Self::This) -> ::windows_core::Result<()>;
    fn RawReadData(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawWriteData(this: &Self::This, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawReadCommand(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawWriteCommand(this: &Self::This, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn Subscribe(this: &Self::This, lpsubsribe: *mut STISUBSCRIBE) -> ::windows_core::Result<()>;
    fn GetLastNotificationData(this: &Self::This, lpnotify: *mut STINOTIFY) -> ::windows_core::Result<()>;
    fn UnSubscribe(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetLastErrorInfo(this: &Self::This, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows_core::Iids for IStiDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStiDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: ::windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&hinst), ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&dwversion), ::core::mem::transmute_copy(&dwmode)).into())
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::core::mem::transmute_copy(&pdevcaps)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pdevstatus)).into())
        }
        unsafe extern "system" fn DeviceReset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceReset(this).into())
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Diagnostic(this, ::core::mem::transmute_copy(&pbuffer)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into())
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastdeviceerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockDevice(this, ::core::mem::transmute_copy(&dwtimeout)).into())
        }
        unsafe extern "system" fn UnLockDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnLockDevice(this).into())
        }
        unsafe extern "system" fn RawReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawReadData(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawWriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawWriteData(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawReadCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawReadCommand(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawWriteCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawWriteCommand(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn Subscribe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Subscribe(this, ::core::mem::transmute_copy(&lpsubsribe)).into())
        }
        unsafe extern "system" fn GetLastNotificationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastNotificationData(this, ::core::mem::transmute_copy(&lpnotify)).into())
        }
        unsafe extern "system" fn UnSubscribe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnSubscribe(this).into())
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastErrorInfo(this, ::core::mem::transmute_copy(&plasterrorinfo)).into())
        }
        IStiDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            DeviceReset: DeviceReset::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            LockDevice: LockDevice::<Identity, Impl, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            Subscribe: Subscribe::<Identity, Impl, OFFSET>,
            GetLastNotificationData: GetLastNotificationData::<Identity, Impl, OFFSET>,
            UnSubscribe: UnSubscribe::<Identity, Impl, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub trait IStiDeviceControl_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, dwdevicetype: u32, dwmode: u32, pwszportname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn RawReadData(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawWriteData(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawReadCommand(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawWriteCommand(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawDeviceControl(this: &Self::This, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::Result<()>;
    fn GetLastError(this: &Self::This, lpdwlasterror: *mut u32) -> ::windows_core::Result<()>;
    fn GetMyDevicePortName(this: &Self::This, lpszdevicepath: ::windows_core::PWSTR, cwdevicepathsize: u32) -> ::windows_core::Result<()>;
    fn GetMyDeviceHandle(this: &Self::This, lph: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetMyDeviceOpenMode(this: &Self::This, pdwopenmode: *mut u32) -> ::windows_core::Result<()>;
    fn WriteToErrorLog(this: &Self::This, dwmessagetype: u32, pszmessage: &::windows_core::PCWSTR, dwerrorcode: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows_core::Iids for IStiDeviceControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStiDeviceControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&dwdevicetype), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute(&pwszportname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn RawReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawReadData(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawWriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawWriteData(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawReadCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawReadCommand(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawWriteCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawWriteCommand(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawDeviceControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawDeviceControl(this, ::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into())
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdwlasterror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastError(this, ::core::mem::transmute_copy(&lpdwlasterror)).into())
        }
        unsafe extern "system" fn GetMyDevicePortName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszdevicepath: ::windows_core::PWSTR, cwdevicepathsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMyDevicePortName(this, ::core::mem::transmute_copy(&lpszdevicepath), ::core::mem::transmute_copy(&cwdevicepathsize)).into())
        }
        unsafe extern "system" fn GetMyDeviceHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMyDeviceHandle(this, ::core::mem::transmute_copy(&lph)).into())
        }
        unsafe extern "system" fn GetMyDeviceOpenMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwopenmode: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMyDeviceOpenMode(this, ::core::mem::transmute_copy(&pdwopenmode)).into())
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows_core::PCWSTR, dwerrorcode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToErrorLog(this, ::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute(&pszmessage), ::core::mem::transmute_copy(&dwerrorcode)).into())
        }
        IStiDeviceControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            RawDeviceControl: RawDeviceControl::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GetMyDevicePortName: GetMyDevicePortName::<Identity, Impl, OFFSET>,
            GetMyDeviceHandle: GetMyDeviceHandle::<Identity, Impl, OFFSET>,
            GetMyDeviceOpenMode: GetMyDeviceOpenMode::<Identity, Impl, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
pub trait IStiUSD_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pheldcb: ::core::option::Option<&IStiDeviceControl>, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<STI_USD_CAPS>;
    fn GetStatus(this: &Self::This, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::Result<()>;
    fn DeviceReset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Diagnostic(this: &Self::This, pbuffer: *mut STI_DIAG) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::Result<()>;
    fn GetLastError(this: &Self::This) -> ::windows_core::Result<u32>;
    fn LockDevice(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnLockDevice(this: &Self::This) -> ::windows_core::Result<()>;
    fn RawReadData(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawWriteData(this: &Self::This, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawReadCommand(this: &Self::This, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn RawWriteCommand(this: &Self::This, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn SetNotificationHandle(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetNotificationData(this: &Self::This, lpnotify: *mut STINOTIFY) -> ::windows_core::Result<()>;
    fn GetLastErrorInfo(this: &Self::This, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for IStiUSD {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStiUSD {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheldcb: *mut ::core::ffi::c_void, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pheldcb), ::core::mem::transmute_copy(&dwstiversion), ::core::mem::transmute_copy(&hparameterskey)).into())
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevcaps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pdevstatus)).into())
        }
        unsafe extern "system" fn DeviceReset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceReset(this).into())
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Diagnostic(this, ::core::mem::transmute_copy(&pbuffer)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&cboutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into())
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastdeviceerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockDevice(this).into())
        }
        unsafe extern "system" fn UnLockDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnLockDevice(this).into())
        }
        unsafe extern "system" fn RawReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawReadData(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawWriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawWriteData(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawReadCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawReadCommand(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn RawWriteCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RawWriteCommand(this, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn SetNotificationHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationHandle(this, ::core::mem::transmute_copy(&hevent)).into())
        }
        unsafe extern "system" fn GetNotificationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNotificationData(this, ::core::mem::transmute_copy(&lpnotify)).into())
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastErrorInfo(this, ::core::mem::transmute_copy(&plasterrorinfo)).into())
        }
        IStiUSD_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            DeviceReset: DeviceReset::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            LockDevice: LockDevice::<Identity, Impl, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            SetNotificationHandle: SetNotificationHandle::<Identity, Impl, OFFSET>,
            GetNotificationData: GetNotificationData::<Identity, Impl, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStillImageW_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows_core::Result<()>;
    fn GetDeviceList(this: &Self::This, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDeviceInfo(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateDevice(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR, dwmode: u32, pdevice: *mut ::core::option::Option<IStiDevice>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetDeviceValue(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows_core::Result<()>;
    fn SetDeviceValue(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows_core::Result<()>;
    fn GetSTILaunchInformation(this: &Self::This, pwszdevicename: ::windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn RegisterLaunchApplication(this: &Self::This, pwszappname: &::windows_core::PCWSTR, pwszcommandline: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterLaunchApplication(this: &Self::This, pwszappname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnableHwNotifications(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetHwNotificationState(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn RefreshDeviceBus(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LaunchApplicationForDevice(this: &Self::This, pwszdevicename: &::windows_core::PCWSTR, pwszappname: &::windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows_core::Result<()>;
    fn SetupDeviceParameters(this: &Self::This, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows_core::Result<()>;
    fn WriteToErrorLog(this: &Self::This, dwmessagetype: u32, pszmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IStillImageW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStillImageW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&hinst), ::core::mem::transmute_copy(&dwversion)).into())
        }
        unsafe extern "system" fn GetDeviceList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceList(this, ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwitemsreturned), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceInfo(this, ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, dwmode: u32, pdevice: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdevice), ::windows_core::from_raw_borrowed(&punkouter)).into())
        }
        unsafe extern "system" fn GetDeviceValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceValue(this, ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn SetDeviceValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceValue(this, ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn GetSTILaunchInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSTILaunchInformation(this, ::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pdweventcode), ::core::mem::transmute_copy(&pwszeventname)).into())
        }
        unsafe extern "system" fn RegisterLaunchApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszappname: ::windows_core::PCWSTR, pwszcommandline: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterLaunchApplication(this, ::core::mem::transmute(&pwszappname), ::core::mem::transmute(&pwszcommandline)).into())
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszappname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterLaunchApplication(this, ::core::mem::transmute(&pwszappname)).into())
        }
        unsafe extern "system" fn EnableHwNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableHwNotifications(this, ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&bnewstate)).into())
        }
        unsafe extern "system" fn GetHwNotificationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHwNotificationState(this, ::core::mem::transmute(&pwszdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcurrentstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RefreshDeviceBus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshDeviceBus(this, ::core::mem::transmute(&pwszdevicename)).into())
        }
        unsafe extern "system" fn LaunchApplicationForDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows_core::PCWSTR, pwszappname: ::windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LaunchApplicationForDevice(this, ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute(&pwszappname), ::core::mem::transmute_copy(&pstinotify)).into())
        }
        unsafe extern "system" fn SetupDeviceParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetupDeviceParameters(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToErrorLog(this, ::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute(&pszmessage)).into())
        }
        IStillImageW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDeviceList: GetDeviceList::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            GetDeviceValue: GetDeviceValue::<Identity, Impl, OFFSET>,
            SetDeviceValue: SetDeviceValue::<Identity, Impl, OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Identity, Impl, OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Identity, Impl, OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Identity, Impl, OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Identity, Impl, OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Identity, Impl, OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Identity, Impl, OFFSET>,
            LaunchApplicationForDevice: LaunchApplicationForDevice::<Identity, Impl, OFFSET>,
            SetupDeviceParameters: SetupDeviceParameters::<Identity, Impl, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
