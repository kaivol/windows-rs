#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DIFsrmClassificationEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DIFsrmClassificationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DIFsrmClassificationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DIFsrmClassificationEvents {
    const VTABLE: Self::Vtable = { DIFsrmClassificationEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmAccessDeniedRemediationClient_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Show(this: &Self::This, parentwnd: usize, accesspath: &::windows_core::BSTR, errortype: AdrClientErrorType, flags: i32, windowtitle: &::windows_core::BSTR, windowmessage: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmAccessDeniedRemediationClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmAccessDeniedRemediationClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>, windowmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>, result: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Show(this, ::core::mem::transmute_copy(&parentwnd), ::core::mem::transmute(&accesspath), ::core::mem::transmute_copy(&errortype), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&windowtitle), ::core::mem::transmute(&windowmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmAccessDeniedRemediationClient_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Show: Show::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmAction_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn ActionType(this: &Self::This) -> ::windows_core::Result<FsrmActionType>;
    fn RunLimitInterval(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRunLimitInterval(this: &Self::This, minutes: i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actiontype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RunLimitInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunLimitInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRunLimitInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRunLimitInterval(this, ::core::mem::transmute_copy(&minutes)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IFsrmAction_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            ActionType: ActionType::<Identity, Impl, OFFSET>,
            RunLimitInterval: RunLimitInterval::<Identity, Impl, OFFSET>,
            SetRunLimitInterval: SetRunLimitInterval::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionCommand_Impl: ::windows_core::BaseImpl + IFsrmAction_Impl {
    fn ExecutablePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetExecutablePath(this: &Self::This, executablepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Arguments(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetArguments(this: &Self::This, arguments: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Account(this: &Self::This) -> ::windows_core::Result<FsrmAccountType>;
    fn SetAccount(this: &Self::This, account: FsrmAccountType) -> ::windows_core::Result<()>;
    fn WorkingDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetWorkingDirectory(this: &Self::This, workingdirectory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MonitorCommand(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMonitorCommand(this: &Self::This, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn KillTimeOut(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetKillTimeOut(this: &Self::This, minutes: i32) -> ::windows_core::Result<()>;
    fn LogResult(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogResult(this: &Self::This, logresults: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmActionCommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmActionCommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExecutablePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executablepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecutablePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(executablepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExecutablePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executablepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExecutablePath(this, ::core::mem::transmute(&executablepath)).into())
        }
        unsafe extern "system" fn Arguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arguments: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Arguments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(arguments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arguments: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetArguments(this, ::core::mem::transmute(&arguments)).into())
        }
        unsafe extern "system" fn Account<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Account(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(account, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccount(this, ::core::mem::transmute_copy(&account)).into())
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, workingdirectory: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WorkingDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(workingdirectory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, workingdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkingDirectory(this, ::core::mem::transmute(&workingdirectory)).into())
        }
        unsafe extern "system" fn MonitorCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, monitorcommand: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MonitorCommand(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(monitorcommand, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMonitorCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMonitorCommand(this, ::core::mem::transmute_copy(&monitorcommand)).into())
        }
        unsafe extern "system" fn KillTimeOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KillTimeOut(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKillTimeOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKillTimeOut(this, ::core::mem::transmute_copy(&minutes)).into())
        }
        unsafe extern "system" fn LogResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logresults: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LogResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(logresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLogResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logresults: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogResult(this, ::core::mem::transmute_copy(&logresults)).into())
        }
        IFsrmActionCommand_Vtbl {
            base__: <IFsrmAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExecutablePath: ExecutablePath::<Identity, Impl, OFFSET>,
            SetExecutablePath: SetExecutablePath::<Identity, Impl, OFFSET>,
            Arguments: Arguments::<Identity, Impl, OFFSET>,
            SetArguments: SetArguments::<Identity, Impl, OFFSET>,
            Account: Account::<Identity, Impl, OFFSET>,
            SetAccount: SetAccount::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            MonitorCommand: MonitorCommand::<Identity, Impl, OFFSET>,
            SetMonitorCommand: SetMonitorCommand::<Identity, Impl, OFFSET>,
            KillTimeOut: KillTimeOut::<Identity, Impl, OFFSET>,
            SetKillTimeOut: SetKillTimeOut::<Identity, Impl, OFFSET>,
            LogResult: LogResult::<Identity, Impl, OFFSET>,
            SetLogResult: SetLogResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionEmail_Impl: ::windows_core::BaseImpl + IFsrmAction_Impl {
    fn MailFrom(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailFrom(this: &Self::This, mailfrom: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MailReplyTo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailReplyTo(this: &Self::This, mailreplyto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MailTo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailTo(this: &Self::This, mailto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MailCc(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailCc(this: &Self::This, mailcc: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MailBcc(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailBcc(this: &Self::This, mailbcc: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MailSubject(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailSubject(this: &Self::This, mailsubject: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MessageText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMessageText(this: &Self::This, messagetext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmActionEmail {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmActionEmail {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MailFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailfrom: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailFrom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailfrom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailfrom: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailFrom(this, ::core::mem::transmute(&mailfrom)).into())
        }
        unsafe extern "system" fn MailReplyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailreplyto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailReplyTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailreplyto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailReplyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailreplyto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailReplyTo(this, ::core::mem::transmute(&mailreplyto)).into())
        }
        unsafe extern "system" fn MailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailTo(this, ::core::mem::transmute(&mailto)).into())
        }
        unsafe extern "system" fn MailCc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailcc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailCc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailcc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailCc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailcc: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailCc(this, ::core::mem::transmute(&mailcc)).into())
        }
        unsafe extern "system" fn MailBcc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailbcc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailBcc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailbcc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailBcc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailbcc: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailBcc(this, ::core::mem::transmute(&mailbcc)).into())
        }
        unsafe extern "system" fn MailSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailsubject: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailSubject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailsubject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailsubject: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailSubject(this, ::core::mem::transmute(&mailsubject)).into())
        }
        unsafe extern "system" fn MessageText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagetext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMessageText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagetext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageText(this, ::core::mem::transmute(&messagetext)).into())
        }
        IFsrmActionEmail_Vtbl {
            base__: <IFsrmAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MailFrom: MailFrom::<Identity, Impl, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, Impl, OFFSET>,
            MailReplyTo: MailReplyTo::<Identity, Impl, OFFSET>,
            SetMailReplyTo: SetMailReplyTo::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
            MailCc: MailCc::<Identity, Impl, OFFSET>,
            SetMailCc: SetMailCc::<Identity, Impl, OFFSET>,
            MailBcc: MailBcc::<Identity, Impl, OFFSET>,
            SetMailBcc: SetMailBcc::<Identity, Impl, OFFSET>,
            MailSubject: MailSubject::<Identity, Impl, OFFSET>,
            SetMailSubject: SetMailSubject::<Identity, Impl, OFFSET>,
            MessageText: MessageText::<Identity, Impl, OFFSET>,
            SetMessageText: SetMessageText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionEmail2_Impl: ::windows_core::BaseImpl + IFsrmActionEmail_Impl {
    fn AttachmentFileListSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAttachmentFileListSize(this: &Self::This, attachmentfilelistsize: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmActionEmail2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmActionEmail);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmActionEmail2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AttachmentFileListSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AttachmentFileListSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachmentfilelistsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttachmentFileListSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEmail2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttachmentFileListSize(this, ::core::mem::transmute_copy(&attachmentfilelistsize)).into())
        }
        IFsrmActionEmail2_Vtbl {
            base__: <IFsrmActionEmail as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AttachmentFileListSize: AttachmentFileListSize::<Identity, Impl, OFFSET>,
            SetAttachmentFileListSize: SetAttachmentFileListSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionEventLog_Impl: ::windows_core::BaseImpl + IFsrmAction_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<FsrmEventType>;
    fn SetEventType(this: &Self::This, eventtype: FsrmEventType) -> ::windows_core::Result<()>;
    fn MessageText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMessageText(this: &Self::This, messagetext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmActionEventLog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmActionEventLog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventType(this, ::core::mem::transmute_copy(&eventtype)).into())
        }
        unsafe extern "system" fn MessageText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagetext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMessageText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagetext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageText(this, ::core::mem::transmute(&messagetext)).into())
        }
        IFsrmActionEventLog_Vtbl {
            base__: <IFsrmAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventType: EventType::<Identity, Impl, OFFSET>,
            SetEventType: SetEventType::<Identity, Impl, OFFSET>,
            MessageText: MessageText::<Identity, Impl, OFFSET>,
            SetMessageText: SetMessageText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionReport_Impl: ::windows_core::BaseImpl + IFsrmAction_Impl {
    fn ReportTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetReportTypes(this: &Self::This, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn MailTo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailTo(this: &Self::This, mailto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmActionReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmAction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmActionReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reporttypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReportTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReportTypes(this, ::core::mem::transmute_copy(&reporttypes)).into())
        }
        unsafe extern "system" fn MailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailTo(this, ::core::mem::transmute(&mailto)).into())
        }
        IFsrmActionReport_Vtbl {
            base__: <IFsrmAction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportTypes: ReportTypes::<Identity, Impl, OFFSET>,
            SetReportTypes: SetReportTypes::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmAutoApplyQuota_Impl: ::windows_core::BaseImpl + IFsrmQuotaObject_Impl {
    fn ExcludeFolders(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetExcludeFolders(this: &Self::This, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn CommitAndUpdateDerived(this: &Self::This, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmAutoApplyQuota {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmQuotaObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmAutoApplyQuota {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExcludeFolders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExcludeFolders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(folders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExcludeFolders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExcludeFolders(this, ::core::mem::transmute_copy(&folders)).into())
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitAndUpdateDerived(this, ::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjectsresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmAutoApplyQuota_Vtbl {
            base__: <IFsrmQuotaObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExcludeFolders: ExcludeFolders::<Identity, Impl, OFFSET>,
            SetExcludeFolders: SetExcludeFolders::<Identity, Impl, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassificationManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ClassificationReportFormats(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetClassificationReportFormats(this: &Self::This, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn Logging(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLogging(this: &Self::This, logging: i32) -> ::windows_core::Result<()>;
    fn ClassificationReportMailTo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClassificationReportMailTo(this: &Self::This, mailto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClassificationReportEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetClassificationReportEnabled(this: &Self::This, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ClassificationLastReportPathWithoutExtension(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClassificationLastError(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClassificationRunningStatus(this: &Self::This) -> ::windows_core::Result<FsrmReportRunningStatus>;
    fn EnumPropertyDefinitions(this: &Self::This, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection>;
    fn CreatePropertyDefinition(this: &Self::This) -> ::windows_core::Result<IFsrmPropertyDefinition>;
    fn GetPropertyDefinition(this: &Self::This, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmPropertyDefinition>;
    fn EnumRules(this: &Self::This, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection>;
    fn CreateRule(this: &Self::This, ruletype: FsrmRuleType) -> ::windows_core::Result<IFsrmRule>;
    fn GetRule(this: &Self::This, rulename: &::windows_core::BSTR, ruletype: FsrmRuleType) -> ::windows_core::Result<IFsrmRule>;
    fn EnumModuleDefinitions(this: &Self::This, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection>;
    fn CreateModuleDefinition(this: &Self::This, moduletype: FsrmPipelineModuleType) -> ::windows_core::Result<IFsrmPipelineModuleDefinition>;
    fn GetModuleDefinition(this: &Self::This, modulename: &::windows_core::BSTR, moduletype: FsrmPipelineModuleType) -> ::windows_core::Result<IFsrmPipelineModuleDefinition>;
    fn RunClassification(this: &Self::This, context: FsrmReportGenerationContext, reserved: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn WaitForClassificationCompletion(this: &Self::This, waitseconds: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CancelClassification(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumFileProperties(this: &Self::This, filepath: &::windows_core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<IFsrmCollection>;
    fn GetFileProperty(this: &Self::This, filepath: &::windows_core::BSTR, propertyname: &::windows_core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(this: &Self::This, filepath: &::windows_core::BSTR, propertyname: &::windows_core::BSTR, propertyvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClearFileProperty(this: &Self::This, filepath: &::windows_core::BSTR, property: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmClassificationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmClassificationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClassificationReportFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassificationReportFormats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClassificationReportFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassificationReportFormats(this, ::core::mem::transmute_copy(&formats)).into())
        }
        unsafe extern "system" fn Logging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Logging(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(logging, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLogging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogging(this, ::core::mem::transmute_copy(&logging)).into())
        }
        unsafe extern "system" fn ClassificationReportMailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassificationReportMailTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClassificationReportMailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassificationReportMailTo(this, ::core::mem::transmute(&mailto)).into())
        }
        unsafe extern "system" fn ClassificationReportEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassificationReportEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClassificationReportEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassificationReportEnabled(this, ::core::mem::transmute_copy(&reportenabled)).into())
        }
        unsafe extern "system" fn ClassificationLastReportPathWithoutExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastreportpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassificationLastReportPathWithoutExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastreportpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClassificationLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lasterror: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassificationLastError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lasterror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClassificationRunningStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassificationRunningStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runningstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumPropertyDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumPropertyDefinitions(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePropertyDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyDefinition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyDefinition(this, ::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRules(this, ::core::mem::transmute_copy(&ruletype), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRule(this, ::core::mem::transmute_copy(&ruletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rulename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRule(this, ::core::mem::transmute(&rulename), ::core::mem::transmute_copy(&ruletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumModuleDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumModuleDefinitions(this, ::core::mem::transmute_copy(&moduletype), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateModuleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateModuleDefinition(this, ::core::mem::transmute_copy(&moduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetModuleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modulename: ::std::mem::MaybeUninit<::windows_core::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModuleDefinition(this, ::core::mem::transmute(&modulename), ::core::mem::transmute_copy(&moduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RunClassification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunClassification(this, ::core::mem::transmute_copy(&context), ::core::mem::transmute(&reserved)).into())
        }
        unsafe extern "system" fn WaitForClassificationCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForClassificationCompletion(this, ::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CancelClassification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelClassification(this).into())
        }
        unsafe extern "system" fn EnumFileProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFileProperties(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileProperty(this, ::core::mem::transmute(&filepath), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFileProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileProperty(this, ::core::mem::transmute(&filepath), ::core::mem::transmute(&propertyname), ::core::mem::transmute(&propertyvalue)).into())
        }
        unsafe extern "system" fn ClearFileProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFileProperty(this, ::core::mem::transmute(&filepath), ::core::mem::transmute(&property)).into())
        }
        IFsrmClassificationManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClassificationReportFormats: ClassificationReportFormats::<Identity, Impl, OFFSET>,
            SetClassificationReportFormats: SetClassificationReportFormats::<Identity, Impl, OFFSET>,
            Logging: Logging::<Identity, Impl, OFFSET>,
            SetLogging: SetLogging::<Identity, Impl, OFFSET>,
            ClassificationReportMailTo: ClassificationReportMailTo::<Identity, Impl, OFFSET>,
            SetClassificationReportMailTo: SetClassificationReportMailTo::<Identity, Impl, OFFSET>,
            ClassificationReportEnabled: ClassificationReportEnabled::<Identity, Impl, OFFSET>,
            SetClassificationReportEnabled: SetClassificationReportEnabled::<Identity, Impl, OFFSET>,
            ClassificationLastReportPathWithoutExtension: ClassificationLastReportPathWithoutExtension::<Identity, Impl, OFFSET>,
            ClassificationLastError: ClassificationLastError::<Identity, Impl, OFFSET>,
            ClassificationRunningStatus: ClassificationRunningStatus::<Identity, Impl, OFFSET>,
            EnumPropertyDefinitions: EnumPropertyDefinitions::<Identity, Impl, OFFSET>,
            CreatePropertyDefinition: CreatePropertyDefinition::<Identity, Impl, OFFSET>,
            GetPropertyDefinition: GetPropertyDefinition::<Identity, Impl, OFFSET>,
            EnumRules: EnumRules::<Identity, Impl, OFFSET>,
            CreateRule: CreateRule::<Identity, Impl, OFFSET>,
            GetRule: GetRule::<Identity, Impl, OFFSET>,
            EnumModuleDefinitions: EnumModuleDefinitions::<Identity, Impl, OFFSET>,
            CreateModuleDefinition: CreateModuleDefinition::<Identity, Impl, OFFSET>,
            GetModuleDefinition: GetModuleDefinition::<Identity, Impl, OFFSET>,
            RunClassification: RunClassification::<Identity, Impl, OFFSET>,
            WaitForClassificationCompletion: WaitForClassificationCompletion::<Identity, Impl, OFFSET>,
            CancelClassification: CancelClassification::<Identity, Impl, OFFSET>,
            EnumFileProperties: EnumFileProperties::<Identity, Impl, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, Impl, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, Impl, OFFSET>,
            ClearFileProperty: ClearFileProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassificationManager2_Impl: ::windows_core::BaseImpl + IFsrmClassificationManager_Impl {
    fn ClassifyFiles(this: &Self::This, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmClassificationManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmClassificationManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmClassificationManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClassifyFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClassifyFiles(this, ::core::mem::transmute_copy(&filepaths), ::core::mem::transmute_copy(&propertynames), ::core::mem::transmute_copy(&propertyvalues), ::core::mem::transmute_copy(&options)).into())
        }
        IFsrmClassificationManager2_Vtbl { base__: <IFsrmClassificationManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ClassifyFiles: ClassifyFiles::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassificationRule_Impl: ::windows_core::BaseImpl + IFsrmRule_Impl {
    fn ExecutionOption(this: &Self::This) -> ::windows_core::Result<FsrmExecutionOption>;
    fn SetExecutionOption(this: &Self::This, executionoption: FsrmExecutionOption) -> ::windows_core::Result<()>;
    fn PropertyAffected(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPropertyAffected(this: &Self::This, property: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetValue(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmClassificationRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmRule);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmClassificationRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExecutionOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecutionOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(executionoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExecutionOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExecutionOption(this, ::core::mem::transmute_copy(&executionoption)).into())
        }
        unsafe extern "system" fn PropertyAffected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyAffected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropertyAffected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyAffected(this, ::core::mem::transmute(&property)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&value)).into())
        }
        IFsrmClassificationRule_Vtbl {
            base__: <IFsrmRule as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExecutionOption: ExecutionOption::<Identity, Impl, OFFSET>,
            SetExecutionOption: SetExecutionOption::<Identity, Impl, OFFSET>,
            PropertyAffected: PropertyAffected::<Identity, Impl, OFFSET>,
            SetPropertyAffected: SetPropertyAffected::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassifierModuleDefinition_Impl: ::windows_core::BaseImpl + IFsrmPipelineModuleDefinition_Impl {
    fn PropertiesAffected(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesAffected(this: &Self::This, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn PropertiesUsed(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesUsed(this: &Self::This, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn NeedsExplicitValue(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNeedsExplicitValue(this: &Self::This, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmClassifierModuleDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmPipelineModuleDefinition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmClassifierModuleDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertiesAffected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertiesAffected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertiesaffected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropertiesAffected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertiesAffected(this, ::core::mem::transmute_copy(&propertiesaffected)).into())
        }
        unsafe extern "system" fn PropertiesUsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertiesUsed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertiesused, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropertiesUsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertiesUsed(this, ::core::mem::transmute_copy(&propertiesused)).into())
        }
        unsafe extern "system" fn NeedsExplicitValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NeedsExplicitValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(needsexplicitvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNeedsExplicitValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNeedsExplicitValue(this, ::core::mem::transmute_copy(&needsexplicitvalue)).into())
        }
        IFsrmClassifierModuleDefinition_Vtbl {
            base__: <IFsrmPipelineModuleDefinition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertiesAffected: PropertiesAffected::<Identity, Impl, OFFSET>,
            SetPropertiesAffected: SetPropertiesAffected::<Identity, Impl, OFFSET>,
            PropertiesUsed: PropertiesUsed::<Identity, Impl, OFFSET>,
            SetPropertiesUsed: SetPropertiesUsed::<Identity, Impl, OFFSET>,
            NeedsExplicitValue: NeedsExplicitValue::<Identity, Impl, OFFSET>,
            SetNeedsExplicitValue: SetNeedsExplicitValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassifierModuleImplementation_Impl: ::windows_core::BaseImpl + IFsrmPipelineModuleImplementation_Impl {
    fn LastModified(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn UseRulesAndDefinitions(this: &Self::This, rules: ::core::option::Option<&IFsrmCollection>, propertydefinitions: ::core::option::Option<&IFsrmCollection>) -> ::windows_core::Result<()>;
    fn OnBeginFile(this: &Self::This, propertybag: ::core::option::Option<&IFsrmPropertyBag>, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn DoesPropertyValueApply(this: &Self::This, property: &::windows_core::BSTR, value: &::windows_core::BSTR, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: &::windows_core::GUID, idpropdef: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetPropertyValueToApply(this: &Self::This, property: &::windows_core::BSTR, value: *mut ::windows_core::BSTR, idrule: &::windows_core::GUID, idpropdef: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnEndFile(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmClassifierModuleImplementation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmPipelineModuleImplementation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmClassifierModuleImplementation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LastModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastModified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UseRulesAndDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rules: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseRulesAndDefinitions(this, ::windows_core::from_raw_borrowed(&rules), ::windows_core::from_raw_borrowed(&propertydefinitions)).into())
        }
        unsafe extern "system" fn OnBeginFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginFile(this, ::windows_core::from_raw_borrowed(&propertybag), ::core::mem::transmute_copy(&arrayruleids)).into())
        }
        unsafe extern "system" fn DoesPropertyValueApply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: ::windows_core::GUID, idpropdef: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoesPropertyValueApply(this, ::core::mem::transmute(&property), ::core::mem::transmute(&value), ::core::mem::transmute_copy(&applyvalue), ::core::mem::transmute(&idrule), ::core::mem::transmute(&idpropdef)).into())
        }
        unsafe extern "system" fn GetPropertyValueToApply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, idrule: ::windows_core::GUID, idpropdef: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyValueToApply(this, ::core::mem::transmute(&property), ::core::mem::transmute_copy(&value), ::core::mem::transmute(&idrule), ::core::mem::transmute(&idpropdef)).into())
        }
        unsafe extern "system" fn OnEndFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndFile(this).into())
        }
        IFsrmClassifierModuleImplementation_Vtbl {
            base__: <IFsrmPipelineModuleImplementation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LastModified: LastModified::<Identity, Impl, OFFSET>,
            UseRulesAndDefinitions: UseRulesAndDefinitions::<Identity, Impl, OFFSET>,
            OnBeginFile: OnBeginFile::<Identity, Impl, OFFSET>,
            DoesPropertyValueApply: DoesPropertyValueApply::<Identity, Impl, OFFSET>,
            GetPropertyValueToApply: GetPropertyValueToApply::<Identity, Impl, OFFSET>,
            OnEndFile: OnEndFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn State(this: &Self::This) -> ::windows_core::Result<FsrmCollectionState>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForCompletion(this: &Self::This, waitseconds: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetById(this: &Self::This, id: &::windows_core::GUID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForCompletion(this, ::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::windows_core::GUID, entry: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetById(this, ::core::mem::transmute(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            GetById: GetById::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmCommittableCollection_Impl: ::windows_core::BaseImpl + IFsrmMutableCollection_Impl {
    fn Commit(this: &Self::This, options: FsrmCommitOptions) -> ::windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmCommittableCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmMutableCollection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCommittableCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmCommittableCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmCommittableCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Commit(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmCommittableCollection_Vtbl { base__: <IFsrmMutableCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Commit: Commit::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmDerivedObjectsResult_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DerivedObjects(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
    fn Results(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmDerivedObjectsResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmDerivedObjectsResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DerivedObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, derivedobjects: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DerivedObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjects, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Results<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, results: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Results(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmDerivedObjectsResult_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DerivedObjects: DerivedObjects::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmExportImport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ExportFileGroups(this: &Self::This, filepath: &::windows_core::BSTR, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportFileGroups(this: &Self::This, filepath: &::windows_core::BSTR, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn ExportFileScreenTemplates(this: &Self::This, filepath: &::windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportFileScreenTemplates(this: &Self::This, filepath: &::windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn ExportQuotaTemplates(this: &Self::This, filepath: &::windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportQuotaTemplates(this: &Self::This, filepath: &::windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmExportImport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmExportImport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExportFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportFileGroups(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&filegroupnamessafearray), ::core::mem::transmute(&remotehost)).into())
        }
        unsafe extern "system" fn ImportFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportFileGroups(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&filegroupnamessafearray), ::core::mem::transmute(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportFileScreenTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportFileScreenTemplates(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)).into())
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, templates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportFileScreenTemplates(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(templates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportQuotaTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportQuotaTemplates(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)).into())
        }
        unsafe extern "system" fn ImportQuotaTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, templates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportQuotaTemplates(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(templates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmExportImport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExportFileGroups: ExportFileGroups::<Identity, Impl, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, Impl, OFFSET>,
            ExportFileScreenTemplates: ExportFileScreenTemplates::<Identity, Impl, OFFSET>,
            ImportFileScreenTemplates: ImportFileScreenTemplates::<Identity, Impl, OFFSET>,
            ExportQuotaTemplates: ExportQuotaTemplates::<Identity, Impl, OFFSET>,
            ImportQuotaTemplates: ImportQuotaTemplates::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileCondition_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<FsrmFileConditionType>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IFsrmFileCondition_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileConditionProperty_Impl: ::windows_core::BaseImpl + IFsrmFileCondition_Impl {
    fn PropertyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPropertyName(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PropertyId(this: &Self::This) -> ::windows_core::Result<FsrmFileSystemPropertyId>;
    fn SetPropertyId(this: &Self::This, newval: FsrmFileSystemPropertyId) -> ::windows_core::Result<()>;
    fn Operator(this: &Self::This) -> ::windows_core::Result<FsrmPropertyConditionType>;
    fn SetOperator(this: &Self::This, newval: FsrmPropertyConditionType) -> ::windows_core::Result<()>;
    fn ValueType(this: &Self::This) -> ::windows_core::Result<FsrmPropertyValueType>;
    fn SetValueType(this: &Self::This, newval: FsrmPropertyValueType) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(this: &Self::This, newval: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileConditionProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmFileCondition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileConditionProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyName(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn PropertyId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropertyId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyId(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Operator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOperator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOperator(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ValueType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValueType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueType(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&newval)).into())
        }
        IFsrmFileConditionProperty_Vtbl {
            base__: <IFsrmFileCondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertyName: PropertyName::<Identity, Impl, OFFSET>,
            SetPropertyName: SetPropertyName::<Identity, Impl, OFFSET>,
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            SetPropertyId: SetPropertyId::<Identity, Impl, OFFSET>,
            Operator: Operator::<Identity, Impl, OFFSET>,
            SetOperator: SetOperator::<Identity, Impl, OFFSET>,
            ValueType: ValueType::<Identity, Impl, OFFSET>,
            SetValueType: SetValueType::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileGroup_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Members(this: &Self::This) -> ::windows_core::Result<IFsrmMutableCollection>;
    fn SetMembers(this: &Self::This, members: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows_core::Result<()>;
    fn NonMembers(this: &Self::This) -> ::windows_core::Result<IFsrmMutableCollection>;
    fn SetNonMembers(this: &Self::This, nonmembers: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, members: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Members(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(members, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, members: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMembers(this, ::windows_core::from_raw_borrowed(&members)).into())
        }
        unsafe extern "system" fn NonMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nonmembers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NonMembers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nonmembers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNonMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nonmembers: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNonMembers(this, ::windows_core::from_raw_borrowed(&nonmembers)).into())
        }
        IFsrmFileGroup_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            SetMembers: SetMembers::<Identity, Impl, OFFSET>,
            NonMembers: NonMembers::<Identity, Impl, OFFSET>,
            SetNonMembers: SetNonMembers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileGroupImported_Impl: ::windows_core::BaseImpl + IFsrmFileGroup_Impl {
    fn OverwriteOnCommit(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(this: &Self::This, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileGroupImported {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmFileGroup);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupImported_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileGroupImported {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OverwriteOnCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupImported_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OverwriteOnCommit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overwrite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupImported_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverwriteOnCommit(this, ::core::mem::transmute_copy(&overwrite)).into())
        }
        IFsrmFileGroupImported_Vtbl {
            base__: <IFsrmFileGroup as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OverwriteOnCommit: OverwriteOnCommit::<Identity, Impl, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileGroupManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CreateFileGroup(this: &Self::This) -> ::windows_core::Result<IFsrmFileGroup>;
    fn GetFileGroup(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileGroup>;
    fn EnumFileGroups(this: &Self::This, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn ExportFileGroups(this: &Self::This, filegroupnamesarray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImportFileGroups(this: &Self::This, serializedfilegroups: &::windows_core::BSTR, filegroupnamesarray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileGroupManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileGroupManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFileGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileGroup(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFileGroups(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Variant::VARIANT, serializedfilegroups: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExportFileGroups(this, ::core::mem::transmute_copy(&filegroupnamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serializedfilegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serializedfilegroups: ::std::mem::MaybeUninit<::windows_core::BSTR>, filegroupnamesarray: *const super::super::System::Variant::VARIANT, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportFileGroups(this, ::core::mem::transmute(&serializedfilegroups), ::core::mem::transmute_copy(&filegroupnamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileGroupManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFileGroup: CreateFileGroup::<Identity, Impl, OFFSET>,
            GetFileGroup: GetFileGroup::<Identity, Impl, OFFSET>,
            EnumFileGroups: EnumFileGroups::<Identity, Impl, OFFSET>,
            ExportFileGroups: ExportFileGroups::<Identity, Impl, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileManagementJob_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamespaceRoots(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(this: &Self::This, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn OperationType(this: &Self::This) -> ::windows_core::Result<FsrmFileManagementType>;
    fn SetOperationType(this: &Self::This, operationtype: FsrmFileManagementType) -> ::windows_core::Result<()>;
    fn ExpirationDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetExpirationDirectory(this: &Self::This, expirationdirectory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CustomAction(this: &Self::This) -> ::windows_core::Result<IFsrmActionCommand>;
    fn Notifications(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Logging(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLogging(this: &Self::This, loggingflags: i32) -> ::windows_core::Result<()>;
    fn ReportEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportEnabled(this: &Self::This, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Formats(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(this: &Self::This, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn MailTo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailTo(this: &Self::This, mailto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DaysSinceFileCreated(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDaysSinceFileCreated(this: &Self::This, dayssincecreation: i32) -> ::windows_core::Result<()>;
    fn DaysSinceFileLastAccessed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDaysSinceFileLastAccessed(this: &Self::This, dayssinceaccess: i32) -> ::windows_core::Result<()>;
    fn DaysSinceFileLastModified(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDaysSinceFileLastModified(this: &Self::This, dayssincemodify: i32) -> ::windows_core::Result<()>;
    fn PropertyConditions(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
    fn FromDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetFromDate(this: &Self::This, fromdate: f64) -> ::windows_core::Result<()>;
    fn Task(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTask(this: &Self::This, taskname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Parameters(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(this: &Self::This, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RunningStatus(this: &Self::This) -> ::windows_core::Result<FsrmReportRunningStatus>;
    fn LastError(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastReportPathWithoutExtension(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastRun(this: &Self::This) -> ::windows_core::Result<f64>;
    fn FileNamePattern(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFileNamePattern(this: &Self::This, filenamepattern: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Run(this: &Self::This, context: FsrmReportGenerationContext) -> ::windows_core::Result<()>;
    fn WaitForCompletion(this: &Self::This, waitseconds: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddNotification(this: &Self::This, days: i32) -> ::windows_core::Result<()>;
    fn DeleteNotification(this: &Self::This, days: i32) -> ::windows_core::Result<()>;
    fn ModifyNotification(this: &Self::This, days: i32, newdays: i32) -> ::windows_core::Result<()>;
    fn CreateNotificationAction(this: &Self::This, days: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction>;
    fn EnumNotificationActions(this: &Self::This, days: i32) -> ::windows_core::Result<IFsrmCollection>;
    fn CreatePropertyCondition(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmPropertyCondition>;
    fn CreateCustomAction(this: &Self::This) -> ::windows_core::Result<IFsrmActionCommand>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileManagementJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileManagementJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn NamespaceRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamespaceRoots(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceroots, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamespaceRoots(this, ::core::mem::transmute_copy(&namespaceroots)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn OperationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OperationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(operationtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOperationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOperationType(this, ::core::mem::transmute_copy(&operationtype)).into())
        }
        unsafe extern "system" fn ExpirationDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expirationdirectory: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpirationDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expirationdirectory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExpirationDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expirationdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExpirationDirectory(this, ::core::mem::transmute(&expirationdirectory)).into())
        }
        unsafe extern "system" fn CustomAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, action: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CustomAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Notifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Notifications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notifications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Logging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Logging(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(loggingflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLogging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogging(this, ::core::mem::transmute_copy(&loggingflags)).into())
        }
        unsafe extern "system" fn ReportEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReportEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReportEnabled(this, ::core::mem::transmute_copy(&reportenabled)).into())
        }
        unsafe extern "system" fn Formats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Formats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormats(this, ::core::mem::transmute_copy(&formats)).into())
        }
        unsafe extern "system" fn MailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailTo(this, ::core::mem::transmute(&mailto)).into())
        }
        unsafe extern "system" fn DaysSinceFileCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DaysSinceFileCreated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dayssincecreation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDaysSinceFileCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysSinceFileCreated(this, ::core::mem::transmute_copy(&dayssincecreation)).into())
        }
        unsafe extern "system" fn DaysSinceFileLastAccessed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DaysSinceFileLastAccessed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dayssinceaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDaysSinceFileLastAccessed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysSinceFileLastAccessed(this, ::core::mem::transmute_copy(&dayssinceaccess)).into())
        }
        unsafe extern "system" fn DaysSinceFileLastModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DaysSinceFileLastModified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dayssincemodify, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDaysSinceFileLastModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaysSinceFileLastModified(this, ::core::mem::transmute_copy(&dayssincemodify)).into())
        }
        unsafe extern "system" fn PropertyConditions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyconditions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyConditions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyconditions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FromDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FromDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fromdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFromDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFromDate(this, ::core::mem::transmute_copy(&fromdate)).into())
        }
        unsafe extern "system" fn Task<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Task(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(taskname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTask(this, ::core::mem::transmute(&taskname)).into())
        }
        unsafe extern "system" fn Parameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute_copy(&parameters)).into())
        }
        unsafe extern "system" fn RunningStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunningStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runningstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lasterror: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lasterror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastReportPathWithoutExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastReportPathWithoutExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastRun(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastrun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FileNamePattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filenamepattern: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileNamePattern(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filenamepattern, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFileNamePattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filenamepattern: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileNamePattern(this, ::core::mem::transmute(&filenamepattern)).into())
        }
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Run(this, ::core::mem::transmute_copy(&context)).into())
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForCompletion(this, ::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn AddNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNotification(this, ::core::mem::transmute_copy(&days)).into())
        }
        unsafe extern "system" fn DeleteNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteNotification(this, ::core::mem::transmute_copy(&days)).into())
        }
        unsafe extern "system" fn ModifyNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyNotification(this, ::core::mem::transmute_copy(&days), ::core::mem::transmute_copy(&newdays)).into())
        }
        unsafe extern "system" fn CreateNotificationAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNotificationAction(this, ::core::mem::transmute_copy(&days), ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumNotificationActions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, days: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumNotificationActions(this, ::core::mem::transmute_copy(&days)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertycondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyCondition(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertycondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCustomAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileManagementJob_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, Impl, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            OperationType: OperationType::<Identity, Impl, OFFSET>,
            SetOperationType: SetOperationType::<Identity, Impl, OFFSET>,
            ExpirationDirectory: ExpirationDirectory::<Identity, Impl, OFFSET>,
            SetExpirationDirectory: SetExpirationDirectory::<Identity, Impl, OFFSET>,
            CustomAction: CustomAction::<Identity, Impl, OFFSET>,
            Notifications: Notifications::<Identity, Impl, OFFSET>,
            Logging: Logging::<Identity, Impl, OFFSET>,
            SetLogging: SetLogging::<Identity, Impl, OFFSET>,
            ReportEnabled: ReportEnabled::<Identity, Impl, OFFSET>,
            SetReportEnabled: SetReportEnabled::<Identity, Impl, OFFSET>,
            Formats: Formats::<Identity, Impl, OFFSET>,
            SetFormats: SetFormats::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
            DaysSinceFileCreated: DaysSinceFileCreated::<Identity, Impl, OFFSET>,
            SetDaysSinceFileCreated: SetDaysSinceFileCreated::<Identity, Impl, OFFSET>,
            DaysSinceFileLastAccessed: DaysSinceFileLastAccessed::<Identity, Impl, OFFSET>,
            SetDaysSinceFileLastAccessed: SetDaysSinceFileLastAccessed::<Identity, Impl, OFFSET>,
            DaysSinceFileLastModified: DaysSinceFileLastModified::<Identity, Impl, OFFSET>,
            SetDaysSinceFileLastModified: SetDaysSinceFileLastModified::<Identity, Impl, OFFSET>,
            PropertyConditions: PropertyConditions::<Identity, Impl, OFFSET>,
            FromDate: FromDate::<Identity, Impl, OFFSET>,
            SetFromDate: SetFromDate::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            SetTask: SetTask::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            RunningStatus: RunningStatus::<Identity, Impl, OFFSET>,
            LastError: LastError::<Identity, Impl, OFFSET>,
            LastReportPathWithoutExtension: LastReportPathWithoutExtension::<Identity, Impl, OFFSET>,
            LastRun: LastRun::<Identity, Impl, OFFSET>,
            FileNamePattern: FileNamePattern::<Identity, Impl, OFFSET>,
            SetFileNamePattern: SetFileNamePattern::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            AddNotification: AddNotification::<Identity, Impl, OFFSET>,
            DeleteNotification: DeleteNotification::<Identity, Impl, OFFSET>,
            ModifyNotification: ModifyNotification::<Identity, Impl, OFFSET>,
            CreateNotificationAction: CreateNotificationAction::<Identity, Impl, OFFSET>,
            EnumNotificationActions: EnumNotificationActions::<Identity, Impl, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, Impl, OFFSET>,
            CreateCustomAction: CreateCustomAction::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileManagementJobManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn EnumFileManagementJobs(this: &Self::This, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection>;
    fn CreateFileManagementJob(this: &Self::This) -> ::windows_core::Result<IFsrmFileManagementJob>;
    fn GetFileManagementJob(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileManagementJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileManagementJobManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileManagementJobManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActionVariables<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionVariables(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variables, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionVariableDescriptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descriptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFileManagementJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFileManagementJobs(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filemanagementjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileManagementJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileManagementJob(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filemanagementjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileManagementJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileManagementJob(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filemanagementjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileManagementJobManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActionVariables: ActionVariables::<Identity, Impl, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, Impl, OFFSET>,
            EnumFileManagementJobs: EnumFileManagementJobs::<Identity, Impl, OFFSET>,
            CreateFileManagementJob: CreateFileManagementJob::<Identity, Impl, OFFSET>,
            GetFileManagementJob: GetFileManagementJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreen_Impl: ::windows_core::BaseImpl + IFsrmFileScreenBase_Impl {
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SourceTemplateName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MatchesSourceTemplate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UserSid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserAccount(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ApplyTemplate(this: &Self::This, filescreentemplatename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreen {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmFileScreenBase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreen {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SourceTemplateName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceTemplateName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplatename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MatchesSourceTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchesSourceTemplate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserSid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usersid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useraccount: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserAccount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(useraccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplyTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyTemplate(this, ::core::mem::transmute(&filescreentemplatename)).into())
        }
        IFsrmFileScreen_Vtbl {
            base__: <IFsrmFileScreenBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            SourceTemplateName: SourceTemplateName::<Identity, Impl, OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Identity, Impl, OFFSET>,
            UserSid: UserSid::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenBase_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn BlockedFileGroups(this: &Self::This) -> ::windows_core::Result<IFsrmMutableCollection>;
    fn SetBlockedFileGroups(this: &Self::This, blocklist: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows_core::Result<()>;
    fn FileScreenFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFileScreenFlags(this: &Self::This, filescreenflags: i32) -> ::windows_core::Result<()>;
    fn CreateAction(this: &Self::This, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction>;
    fn EnumActions(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreenBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreenBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BlockedFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blocklist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockedFileGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blocklist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBlockedFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blocklist: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlockedFileGroups(this, ::windows_core::from_raw_borrowed(&blocklist)).into())
        }
        unsafe extern "system" fn FileScreenFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileScreenFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFileScreenFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileScreenFlags(this, ::core::mem::transmute_copy(&filescreenflags)).into())
        }
        unsafe extern "system" fn CreateAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAction(this, ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumActions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumActions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileScreenBase_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BlockedFileGroups: BlockedFileGroups::<Identity, Impl, OFFSET>,
            SetBlockedFileGroups: SetBlockedFileGroups::<Identity, Impl, OFFSET>,
            FileScreenFlags: FileScreenFlags::<Identity, Impl, OFFSET>,
            SetFileScreenFlags: SetFileScreenFlags::<Identity, Impl, OFFSET>,
            CreateAction: CreateAction::<Identity, Impl, OFFSET>,
            EnumActions: EnumActions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenException_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AllowedFileGroups(this: &Self::This) -> ::windows_core::Result<IFsrmMutableCollection>;
    fn SetAllowedFileGroups(this: &Self::This, allowlist: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreenException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreenException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllowedFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowedFileGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allowlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowedFileGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowlist: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowedFileGroups(this, ::windows_core::from_raw_borrowed(&allowlist)).into())
        }
        IFsrmFileScreenException_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            AllowedFileGroups: AllowedFileGroups::<Identity, Impl, OFFSET>,
            SetAllowedFileGroups: SetAllowedFileGroups::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateFileScreen(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileScreen>;
    fn GetFileScreen(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileScreen>;
    fn EnumFileScreens(this: &Self::This, path: &::windows_core::BSTR, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenException(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileScreenException>;
    fn GetFileScreenException(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileScreenException>;
    fn EnumFileScreenExceptions(this: &Self::This, path: &::windows_core::BSTR, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenCollection(this: &Self::This) -> ::windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreenManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreenManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActionVariables<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionVariables(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variables, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionVariableDescriptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descriptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileScreen(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileScreen(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFileScreens<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmEnumOptions, filescreens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFileScreens(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileScreenException<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileScreenException(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenexception, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileScreenException<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileScreenException(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenexception, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFileScreenExceptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFileScreenExceptions(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenexceptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileScreenCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileScreenCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileScreenManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActionVariables: ActionVariables::<Identity, Impl, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, Impl, OFFSET>,
            CreateFileScreen: CreateFileScreen::<Identity, Impl, OFFSET>,
            GetFileScreen: GetFileScreen::<Identity, Impl, OFFSET>,
            EnumFileScreens: EnumFileScreens::<Identity, Impl, OFFSET>,
            CreateFileScreenException: CreateFileScreenException::<Identity, Impl, OFFSET>,
            GetFileScreenException: GetFileScreenException::<Identity, Impl, OFFSET>,
            EnumFileScreenExceptions: EnumFileScreenExceptions::<Identity, Impl, OFFSET>,
            CreateFileScreenCollection: CreateFileScreenCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenTemplate_Impl: ::windows_core::BaseImpl + IFsrmFileScreenBase_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CopyTemplate(this: &Self::This, filescreentemplatename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CommitAndUpdateDerived(this: &Self::This, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreenTemplate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmFileScreenBase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreenTemplate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn CopyTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTemplate(this, ::core::mem::transmute(&filescreentemplatename)).into())
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitAndUpdateDerived(this, ::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjectsresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileScreenTemplate_Vtbl {
            base__: <IFsrmFileScreenBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            CopyTemplate: CopyTemplate::<Identity, Impl, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenTemplateImported_Impl: ::windows_core::BaseImpl + IFsrmFileScreenTemplate_Impl {
    fn OverwriteOnCommit(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(this: &Self::This, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreenTemplateImported {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmFileScreenTemplate);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreenTemplateImported {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OverwriteOnCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OverwriteOnCommit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overwrite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverwriteOnCommit(this, ::core::mem::transmute_copy(&overwrite)).into())
        }
        IFsrmFileScreenTemplateImported_Vtbl {
            base__: <IFsrmFileScreenTemplate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OverwriteOnCommit: OverwriteOnCommit::<Identity, Impl, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenTemplateManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(this: &Self::This) -> ::windows_core::Result<IFsrmFileScreenTemplate>;
    fn GetTemplate(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmFileScreenTemplate>;
    fn EnumTemplates(this: &Self::This, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(this: &Self::This, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImportTemplates(this: &Self::This, serializedfilescreentemplates: &::windows_core::BSTR, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmFileScreenTemplateManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmFileScreenTemplateManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTemplate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTemplate(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumTemplates(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT, serializedfilescreentemplates: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExportTemplates(this, ::core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serializedfilescreentemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::std::mem::MaybeUninit<::windows_core::BSTR>, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportTemplates(this, ::core::mem::transmute(&serializedfilescreentemplates), ::core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmFileScreenTemplateManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTemplate: CreateTemplate::<Identity, Impl, OFFSET>,
            GetTemplate: GetTemplate::<Identity, Impl, OFFSET>,
            EnumTemplates: EnumTemplates::<Identity, Impl, OFFSET>,
            ExportTemplates: ExportTemplates::<Identity, Impl, OFFSET>,
            ImportTemplates: ImportTemplates::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmMutableCollection_Impl: ::windows_core::BaseImpl + IFsrmCollection_Impl {
    fn Add(this: &Self::This, item: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, index: i32) -> ::windows_core::Result<()>;
    fn RemoveById(this: &Self::This, id: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IFsrmMutableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmMutableCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmCollection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmMutableCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&item)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn RemoveById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveById(this, ::core::mem::transmute(&id)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmMutableCollection_Vtbl {
            base__: <IFsrmCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveById: RemoveById::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmObject_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, description: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&description)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        IFsrmObject_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPathMapper_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetSharePathsForLocalPath(this: &Self::This, localpath: &::windows_core::BSTR) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPathMapper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPathMapper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPathMapper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSharePathsForLocalPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPathMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSharePathsForLocalPath(this, ::core::mem::transmute(&localpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharepaths, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmPathMapper_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSharePathsForLocalPath: GetSharePathsForLocalPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPipelineModuleConnector_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ModuleImplementation(this: &Self::This) -> ::windows_core::Result<IFsrmPipelineModuleImplementation>;
    fn ModuleName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HostingUserAccount(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HostingProcessPid(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Bind(this: &Self::This, moduledefinition: ::core::option::Option<&IFsrmPipelineModuleDefinition>, moduleimplementation: ::core::option::Option<&IFsrmPipelineModuleImplementation>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPipelineModuleConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPipelineModuleConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ModuleImplementation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleImplementation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pipelinemoduleimplementation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModuleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(username, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HostingUserAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useraccount: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostingUserAccount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(useraccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HostingProcessPid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostingProcessPid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduledefinition: *mut ::core::ffi::c_void, moduleimplementation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bind(this, ::windows_core::from_raw_borrowed(&moduledefinition), ::windows_core::from_raw_borrowed(&moduleimplementation)).into())
        }
        IFsrmPipelineModuleConnector_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ModuleImplementation: ModuleImplementation::<Identity, Impl, OFFSET>,
            ModuleName: ModuleName::<Identity, Impl, OFFSET>,
            HostingUserAccount: HostingUserAccount::<Identity, Impl, OFFSET>,
            HostingProcessPid: HostingProcessPid::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPipelineModuleDefinition_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn ModuleClsid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModuleClsid(this: &Self::This, moduleclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Company(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCompany(this: &Self::This, company: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVersion(this: &Self::This, version: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ModuleType(this: &Self::This) -> ::windows_core::Result<FsrmPipelineModuleType>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn NeedsFileContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNeedsFileContent(this: &Self::This, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Account(this: &Self::This) -> ::windows_core::Result<FsrmAccountType>;
    fn SetAccount(this: &Self::This, retrievalaccount: FsrmAccountType) -> ::windows_core::Result<()>;
    fn SupportedExtensions(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetSupportedExtensions(this: &Self::This, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn Parameters(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(this: &Self::This, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPipelineModuleDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPipelineModuleDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ModuleClsid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleClsid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduleclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetModuleClsid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModuleClsid(this, ::core::mem::transmute(&moduleclsid)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Company<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, company: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Company(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(company, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompany<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, company: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompany(this, ::core::mem::transmute(&company)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute(&version)).into())
        }
        unsafe extern "system" fn ModuleType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn NeedsFileContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, needsfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NeedsFileContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(needsfilecontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNeedsFileContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNeedsFileContent(this, ::core::mem::transmute_copy(&needsfilecontent)).into())
        }
        unsafe extern "system" fn Account<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Account(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retrievalaccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccount(this, ::core::mem::transmute_copy(&retrievalaccount)).into())
        }
        unsafe extern "system" fn SupportedExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedextensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSupportedExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSupportedExtensions(this, ::core::mem::transmute_copy(&supportedextensions)).into())
        }
        unsafe extern "system" fn Parameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute_copy(&parameters)).into())
        }
        IFsrmPipelineModuleDefinition_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ModuleClsid: ModuleClsid::<Identity, Impl, OFFSET>,
            SetModuleClsid: SetModuleClsid::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            SetCompany: SetCompany::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            ModuleType: ModuleType::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            NeedsFileContent: NeedsFileContent::<Identity, Impl, OFFSET>,
            SetNeedsFileContent: SetNeedsFileContent::<Identity, Impl, OFFSET>,
            Account: Account::<Identity, Impl, OFFSET>,
            SetAccount: SetAccount::<Identity, Impl, OFFSET>,
            SupportedExtensions: SupportedExtensions::<Identity, Impl, OFFSET>,
            SetSupportedExtensions: SetSupportedExtensions::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPipelineModuleImplementation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OnLoad(this: &Self::This, moduledefinition: ::core::option::Option<&IFsrmPipelineModuleDefinition>) -> ::windows_core::Result<IFsrmPipelineModuleConnector>;
    fn OnUnload(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPipelineModuleImplementation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPipelineModuleImplementation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduledefinition: *mut ::core::ffi::c_void, moduleconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnLoad(this, ::windows_core::from_raw_borrowed(&moduledefinition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduleconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUnload(this).into())
        }
        IFsrmPipelineModuleImplementation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmProperty_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Sources(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyFlags(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Sources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmProperty_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            Sources: Sources::<Identity, Impl, OFFSET>,
            PropertyFlags: PropertyFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyBag_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RelativePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RelativeNamespaceRoot(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeIndex(this: &Self::This) -> ::windows_core::Result<u32>;
    fn FileId(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ParentDirectoryId(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Size(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SizeAllocated(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CreationTime(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn LastAccessTime(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn LastModificationTime(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Attributes(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OwnerSid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FilePropertyNames(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Messages(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyBagFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFileProperty(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(this: &Self::This, name: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddMessage(this: &Self::This, message: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetFileStreamInterface(this: &Self::This, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPropertyBag {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPropertyBag {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RelativePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RelativePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volumename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volumename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RelativeNamespaceRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RelativeNamespaceRoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relativenamespaceroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volumeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FileId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParentDirectoryId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParentDirectoryId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parentdirectoryid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SizeAllocated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeAllocated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sizeallocated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(creationtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastAccessTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastAccessTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastaccesstime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastModificationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastModificationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodificationtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OwnerSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerSid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ownersid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FilePropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FilePropertyNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filepropertynames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Messages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Messages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyBagFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyBagFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, fileproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileProperty(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFileProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMessage(this, ::core::mem::transmute(&message)).into())
        }
        unsafe extern "system" fn GetFileStreamInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileStreamInterface(this, ::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&interfacetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstreaminterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmPropertyBag_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            RelativePath: RelativePath::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            RelativeNamespaceRoot: RelativeNamespaceRoot::<Identity, Impl, OFFSET>,
            VolumeIndex: VolumeIndex::<Identity, Impl, OFFSET>,
            FileId: FileId::<Identity, Impl, OFFSET>,
            ParentDirectoryId: ParentDirectoryId::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SizeAllocated: SizeAllocated::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            LastAccessTime: LastAccessTime::<Identity, Impl, OFFSET>,
            LastModificationTime: LastModificationTime::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            OwnerSid: OwnerSid::<Identity, Impl, OFFSET>,
            FilePropertyNames: FilePropertyNames::<Identity, Impl, OFFSET>,
            Messages: Messages::<Identity, Impl, OFFSET>,
            PropertyBagFlags: PropertyBagFlags::<Identity, Impl, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, Impl, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            GetFileStreamInterface: GetFileStreamInterface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyBag2_Impl: ::windows_core::BaseImpl + IFsrmPropertyBag_Impl {
    fn GetFieldValue(this: &Self::This, field: FsrmPropertyBagField) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetUntrustedInFileProperties(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPropertyBag2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmPropertyBag);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPropertyBag2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFieldValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFieldValue(this, ::core::mem::transmute_copy(&field)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUntrustedInFileProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUntrustedInFileProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(props, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmPropertyBag2_Vtbl {
            base__: <IFsrmPropertyBag as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFieldValue: GetFieldValue::<Identity, Impl, OFFSET>,
            GetUntrustedInFileProperties: GetUntrustedInFileProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyCondition_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<FsrmPropertyConditionType>;
    fn SetType(this: &Self::This, r#type: FsrmPropertyConditionType) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetValue(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPropertyCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPropertyCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IFsrmPropertyCondition_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyDefinition_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<FsrmPropertyDefinitionType>;
    fn SetType(this: &Self::This, r#type: FsrmPropertyDefinitionType) -> ::windows_core::Result<()>;
    fn PossibleValues(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPossibleValues(this: &Self::This, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn ValueDescriptions(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetValueDescriptions(this: &Self::This, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn Parameters(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(this: &Self::This, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPropertyDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPropertyDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn PossibleValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PossibleValues(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(possiblevalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPossibleValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPossibleValues(this, ::core::mem::transmute_copy(&possiblevalues)).into())
        }
        unsafe extern "system" fn ValueDescriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueDescriptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuedescriptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValueDescriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueDescriptions(this, ::core::mem::transmute_copy(&valuedescriptions)).into())
        }
        unsafe extern "system" fn Parameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute_copy(&parameters)).into())
        }
        IFsrmPropertyDefinition_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            PossibleValues: PossibleValues::<Identity, Impl, OFFSET>,
            SetPossibleValues: SetPossibleValues::<Identity, Impl, OFFSET>,
            ValueDescriptions: ValueDescriptions::<Identity, Impl, OFFSET>,
            SetValueDescriptions: SetValueDescriptions::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyDefinition2_Impl: ::windows_core::BaseImpl + IFsrmPropertyDefinition_Impl {
    fn PropertyDefinitionFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AppliesTo(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ValueDefinitions(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPropertyDefinition2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmPropertyDefinition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPropertyDefinition2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertyDefinitionFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyDefinitionFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinitionflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn AppliesTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppliesTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(appliesto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValueDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuedefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueDefinitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuedefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmPropertyDefinition2_Vtbl {
            base__: <IFsrmPropertyDefinition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertyDefinitionFlags: PropertyDefinitionFlags::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            AppliesTo: AppliesTo::<Identity, Impl, OFFSET>,
            ValueDefinitions: ValueDefinitions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyDefinitionValue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UniqueID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmPropertyDefinitionValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmPropertyDefinitionValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UniqueID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uniqueid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UniqueID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uniqueid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmPropertyDefinitionValue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            UniqueID: UniqueID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuota_Impl: ::windows_core::BaseImpl + IFsrmQuotaObject_Impl {
    fn QuotaUsed(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn QuotaPeakUsage(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn QuotaPeakUsageTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ResetPeakUsage(this: &Self::This) -> ::windows_core::Result<()>;
    fn RefreshUsageProperties(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuota {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmQuotaObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuota {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QuotaUsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuotaUsed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(used, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QuotaPeakUsage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuotaPeakUsage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peakusage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QuotaPeakUsageTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuotaPeakUsageTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peakusagedatetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResetPeakUsage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetPeakUsage(this).into())
        }
        unsafe extern "system" fn RefreshUsageProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshUsageProperties(this).into())
        }
        IFsrmQuota_Vtbl {
            base__: <IFsrmQuotaObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QuotaUsed: QuotaUsed::<Identity, Impl, OFFSET>,
            QuotaPeakUsage: QuotaPeakUsage::<Identity, Impl, OFFSET>,
            QuotaPeakUsageTime: QuotaPeakUsageTime::<Identity, Impl, OFFSET>,
            ResetPeakUsage: ResetPeakUsage::<Identity, Impl, OFFSET>,
            RefreshUsageProperties: RefreshUsageProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaBase_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn QuotaLimit(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetQuotaLimit(this: &Self::This, quotalimit: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn QuotaFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetQuotaFlags(this: &Self::This, quotaflags: i32) -> ::windows_core::Result<()>;
    fn Thresholds(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddThreshold(this: &Self::This, threshold: i32) -> ::windows_core::Result<()>;
    fn DeleteThreshold(this: &Self::This, threshold: i32) -> ::windows_core::Result<()>;
    fn ModifyThreshold(this: &Self::This, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()>;
    fn CreateThresholdAction(this: &Self::This, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction>;
    fn EnumThresholdActions(this: &Self::This, threshold: i32) -> ::windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QuotaLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuotaLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotalimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuotaLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotalimit: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuotaLimit(this, ::core::mem::transmute(&quotalimit)).into())
        }
        unsafe extern "system" fn QuotaFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuotaFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotaflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuotaFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuotaFlags(this, ::core::mem::transmute_copy(&quotaflags)).into())
        }
        unsafe extern "system" fn Thresholds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Thresholds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(thresholds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddThreshold(this, ::core::mem::transmute_copy(&threshold)).into())
        }
        unsafe extern "system" fn DeleteThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteThreshold(this, ::core::mem::transmute_copy(&threshold)).into())
        }
        unsafe extern "system" fn ModifyThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyThreshold(this, ::core::mem::transmute_copy(&threshold), ::core::mem::transmute_copy(&newthreshold)).into())
        }
        unsafe extern "system" fn CreateThresholdAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateThresholdAction(this, ::core::mem::transmute_copy(&threshold), ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumThresholdActions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumThresholdActions(this, ::core::mem::transmute_copy(&threshold)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmQuotaBase_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QuotaLimit: QuotaLimit::<Identity, Impl, OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Identity, Impl, OFFSET>,
            QuotaFlags: QuotaFlags::<Identity, Impl, OFFSET>,
            SetQuotaFlags: SetQuotaFlags::<Identity, Impl, OFFSET>,
            Thresholds: Thresholds::<Identity, Impl, OFFSET>,
            AddThreshold: AddThreshold::<Identity, Impl, OFFSET>,
            DeleteThreshold: DeleteThreshold::<Identity, Impl, OFFSET>,
            ModifyThreshold: ModifyThreshold::<Identity, Impl, OFFSET>,
            CreateThresholdAction: CreateThresholdAction::<Identity, Impl, OFFSET>,
            EnumThresholdActions: EnumThresholdActions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateQuota(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmQuota>;
    fn CreateAutoApplyQuota(this: &Self::This, quotatemplatename: &::windows_core::BSTR, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmAutoApplyQuota>;
    fn GetQuota(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmQuota>;
    fn GetAutoApplyQuota(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmAutoApplyQuota>;
    fn GetRestrictiveQuota(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmQuota>;
    fn EnumQuotas(this: &Self::This, path: &::windows_core::BSTR, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn EnumAutoApplyQuotas(this: &Self::This, path: &::windows_core::BSTR, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn EnumEffectiveQuotas(this: &Self::This, path: &::windows_core::BSTR, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn Scan(this: &Self::This, strpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CreateQuotaCollection(this: &Self::This) -> ::windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActionVariables<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionVariables(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variables, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionVariableDescriptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descriptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQuota(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAutoApplyQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::std::mem::MaybeUninit<::windows_core::BSTR>, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAutoApplyQuota(this, ::core::mem::transmute(&quotatemplatename), ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQuota(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAutoApplyQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutoApplyQuota(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRestrictiveQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictiveQuota(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumQuotas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumQuotas(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumAutoApplyQuotas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAutoApplyQuotas(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumEffectiveQuotas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumEffectiveQuotas(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Scan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scan(this, ::core::mem::transmute(&strpath)).into())
        }
        unsafe extern "system" fn CreateQuotaCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQuotaCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmQuotaManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActionVariables: ActionVariables::<Identity, Impl, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, Impl, OFFSET>,
            CreateQuota: CreateQuota::<Identity, Impl, OFFSET>,
            CreateAutoApplyQuota: CreateAutoApplyQuota::<Identity, Impl, OFFSET>,
            GetQuota: GetQuota::<Identity, Impl, OFFSET>,
            GetAutoApplyQuota: GetAutoApplyQuota::<Identity, Impl, OFFSET>,
            GetRestrictiveQuota: GetRestrictiveQuota::<Identity, Impl, OFFSET>,
            EnumQuotas: EnumQuotas::<Identity, Impl, OFFSET>,
            EnumAutoApplyQuotas: EnumAutoApplyQuotas::<Identity, Impl, OFFSET>,
            EnumEffectiveQuotas: EnumEffectiveQuotas::<Identity, Impl, OFFSET>,
            Scan: Scan::<Identity, Impl, OFFSET>,
            CreateQuotaCollection: CreateQuotaCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaManagerEx_Impl: ::windows_core::BaseImpl + IFsrmQuotaManager_Impl {
    fn IsAffectedByQuota(this: &Self::This, path: &::windows_core::BSTR, options: FsrmEnumOptions) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaManagerEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmQuotaManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManagerEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaManagerEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsAffectedByQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaManagerEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, options: FsrmEnumOptions, affected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAffectedByQuota(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(affected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmQuotaManagerEx_Vtbl { base__: <IFsrmQuotaManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsAffectedByQuota: IsAffectedByQuota::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaObject_Impl: ::windows_core::BaseImpl + IFsrmQuotaBase_Impl {
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserSid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserAccount(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SourceTemplateName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MatchesSourceTemplate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ApplyTemplate(this: &Self::This, quotatemplatename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmQuotaBase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserSid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usersid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useraccount: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserAccount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(useraccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SourceTemplateName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotatemplatename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceTemplateName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplatename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MatchesSourceTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchesSourceTemplate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplyTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyTemplate(this, ::core::mem::transmute(&quotatemplatename)).into())
        }
        IFsrmQuotaObject_Vtbl {
            base__: <IFsrmQuotaBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            UserSid: UserSid::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            SourceTemplateName: SourceTemplateName::<Identity, Impl, OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Identity, Impl, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaTemplate_Impl: ::windows_core::BaseImpl + IFsrmQuotaBase_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CopyTemplate(this: &Self::This, quotatemplatename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CommitAndUpdateDerived(this: &Self::This, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaTemplate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmQuotaBase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaTemplate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn CopyTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTemplate(this, ::core::mem::transmute(&quotatemplatename)).into())
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitAndUpdateDerived(this, ::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjectsresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmQuotaTemplate_Vtbl {
            base__: <IFsrmQuotaBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            CopyTemplate: CopyTemplate::<Identity, Impl, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaTemplateImported_Impl: ::windows_core::BaseImpl + IFsrmQuotaTemplate_Impl {
    fn OverwriteOnCommit(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(this: &Self::This, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaTemplateImported {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmQuotaTemplate);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaTemplateImported {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OverwriteOnCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OverwriteOnCommit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overwrite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverwriteOnCommit(this, ::core::mem::transmute_copy(&overwrite)).into())
        }
        IFsrmQuotaTemplateImported_Vtbl {
            base__: <IFsrmQuotaTemplate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OverwriteOnCommit: OverwriteOnCommit::<Identity, Impl, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaTemplateManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(this: &Self::This) -> ::windows_core::Result<IFsrmQuotaTemplate>;
    fn GetTemplate(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmQuotaTemplate>;
    fn EnumTemplates(this: &Self::This, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(this: &Self::This, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImportTemplates(this: &Self::This, serializedquotatemplates: &::windows_core::BSTR, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmQuotaTemplateManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmQuotaTemplateManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTemplate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTemplate(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumTemplates(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT, serializedquotatemplates: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExportTemplates(this, ::core::mem::transmute_copy(&quotatemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serializedquotatemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::std::mem::MaybeUninit<::windows_core::BSTR>, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportTemplates(this, ::core::mem::transmute(&serializedquotatemplates), ::core::mem::transmute_copy(&quotatemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmQuotaTemplateManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTemplate: CreateTemplate::<Identity, Impl, OFFSET>,
            GetTemplate: GetTemplate::<Identity, Impl, OFFSET>,
            EnumTemplates: EnumTemplates::<Identity, Impl, OFFSET>,
            ExportTemplates: ExportTemplates::<Identity, Impl, OFFSET>,
            ImportTemplates: ImportTemplates::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<FsrmReportType>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, description: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LastGeneratedFileNamePrefix(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFilter(this: &Self::This, filter: FsrmReportFilter) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFilter(this: &Self::This, filter: FsrmReportFilter, filtervalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reporttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&description)).into())
        }
        unsafe extern "system" fn LastGeneratedFileNamePrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastGeneratedFileNamePrefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefix, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilter(this, ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filtervalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilter(this, ::core::mem::transmute_copy(&filter), ::core::mem::transmute(&filtervalue)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IFsrmReport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LastGeneratedFileNamePrefix: LastGeneratedFileNamePrefix::<Identity, Impl, OFFSET>,
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReportJob_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn Task(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTask(this: &Self::This, taskname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamespaceRoots(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(this: &Self::This, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn Formats(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(this: &Self::This, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn MailTo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailTo(this: &Self::This, mailto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RunningStatus(this: &Self::This) -> ::windows_core::Result<FsrmReportRunningStatus>;
    fn LastRun(this: &Self::This) -> ::windows_core::Result<f64>;
    fn LastError(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastGeneratedInDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnumReports(this: &Self::This) -> ::windows_core::Result<IFsrmCollection>;
    fn CreateReport(this: &Self::This, reporttype: FsrmReportType) -> ::windows_core::Result<IFsrmReport>;
    fn Run(this: &Self::This, context: FsrmReportGenerationContext) -> ::windows_core::Result<()>;
    fn WaitForCompletion(this: &Self::This, waitseconds: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmReportJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmReportJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Task<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Task(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(taskname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTask(this, ::core::mem::transmute(&taskname)).into())
        }
        unsafe extern "system" fn NamespaceRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamespaceRoots(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceroots, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamespaceRoots(this, ::core::mem::transmute_copy(&namespaceroots)).into())
        }
        unsafe extern "system" fn Formats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Formats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormats(this, ::core::mem::transmute_copy(&formats)).into())
        }
        unsafe extern "system" fn MailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailTo(this, ::core::mem::transmute(&mailto)).into())
        }
        unsafe extern "system" fn RunningStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunningStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runningstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastRun(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastrun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lasterror: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lasterror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastGeneratedInDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastGeneratedInDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumReports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reports: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumReports(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reports, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReport(this, ::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(report, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Run(this, ::core::mem::transmute_copy(&context)).into())
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForCompletion(this, ::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IFsrmReportJob_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Task: Task::<Identity, Impl, OFFSET>,
            SetTask: SetTask::<Identity, Impl, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, Impl, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, Impl, OFFSET>,
            Formats: Formats::<Identity, Impl, OFFSET>,
            SetFormats: SetFormats::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
            RunningStatus: RunningStatus::<Identity, Impl, OFFSET>,
            LastRun: LastRun::<Identity, Impl, OFFSET>,
            LastError: LastError::<Identity, Impl, OFFSET>,
            LastGeneratedInDirectory: LastGeneratedInDirectory::<Identity, Impl, OFFSET>,
            EnumReports: EnumReports::<Identity, Impl, OFFSET>,
            CreateReport: CreateReport::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReportManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EnumReportJobs(this: &Self::This, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection>;
    fn CreateReportJob(this: &Self::This) -> ::windows_core::Result<IFsrmReportJob>;
    fn GetReportJob(this: &Self::This, taskname: &::windows_core::BSTR) -> ::windows_core::Result<IFsrmReportJob>;
    fn GetOutputDirectory(this: &Self::This, context: FsrmReportGenerationContext) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOutputDirectory(this: &Self::This, context: FsrmReportGenerationContext, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsFilterValidForReportType(this: &Self::This, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDefaultFilter(this: &Self::This, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDefaultFilter(this: &Self::This, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetReportSizeLimit(this: &Self::This, limit: FsrmReportLimit) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetReportSizeLimit(this: &Self::This, limit: FsrmReportLimit, limitvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmReportManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmReportManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumReportJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumReportJobs(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateReportJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReportJob(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReportJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReportJob(this, ::core::mem::transmute(&taskname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputDirectory(this, ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutputDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputDirectory(this, ::core::mem::transmute_copy(&context), ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn IsFilterValidForReportType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFilterValidForReportType(this, ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultFilter(this, ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filtervalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultFilter(this, ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter), ::core::mem::transmute(&filtervalue)).into())
        }
        unsafe extern "system" fn GetReportSizeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReportSizeLimit(this, ::core::mem::transmute_copy(&limit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(limitvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReportSizeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReportSizeLimit(this, ::core::mem::transmute_copy(&limit), ::core::mem::transmute(&limitvalue)).into())
        }
        IFsrmReportManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumReportJobs: EnumReportJobs::<Identity, Impl, OFFSET>,
            CreateReportJob: CreateReportJob::<Identity, Impl, OFFSET>,
            GetReportJob: GetReportJob::<Identity, Impl, OFFSET>,
            GetOutputDirectory: GetOutputDirectory::<Identity, Impl, OFFSET>,
            SetOutputDirectory: SetOutputDirectory::<Identity, Impl, OFFSET>,
            IsFilterValidForReportType: IsFilterValidForReportType::<Identity, Impl, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, Impl, OFFSET>,
            SetDefaultFilter: SetDefaultFilter::<Identity, Impl, OFFSET>,
            GetReportSizeLimit: GetReportSizeLimit::<Identity, Impl, OFFSET>,
            SetReportSizeLimit: SetReportSizeLimit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReportScheduler_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn VerifyNamespaces(this: &Self::This, namespacessafearray: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CreateScheduleTask(this: &Self::This, taskname: &::windows_core::BSTR, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ModifyScheduleTask(this: &Self::This, taskname: &::windows_core::BSTR, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteScheduleTask(this: &Self::This, taskname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmReportScheduler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmReportScheduler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VerifyNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VerifyNamespaces(this, ::core::mem::transmute_copy(&namespacessafearray)).into())
        }
        unsafe extern "system" fn CreateScheduleTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateScheduleTask(this, ::core::mem::transmute(&taskname), ::core::mem::transmute_copy(&namespacessafearray), ::core::mem::transmute(&serializedtask)).into())
        }
        unsafe extern "system" fn ModifyScheduleTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyScheduleTask(this, ::core::mem::transmute(&taskname), ::core::mem::transmute_copy(&namespacessafearray), ::core::mem::transmute(&serializedtask)).into())
        }
        unsafe extern "system" fn DeleteScheduleTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteScheduleTask(this, ::core::mem::transmute(&taskname)).into())
        }
        IFsrmReportScheduler_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VerifyNamespaces: VerifyNamespaces::<Identity, Impl, OFFSET>,
            CreateScheduleTask: CreateScheduleTask::<Identity, Impl, OFFSET>,
            ModifyScheduleTask: ModifyScheduleTask::<Identity, Impl, OFFSET>,
            DeleteScheduleTask: DeleteScheduleTask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmRule_Impl: ::windows_core::BaseImpl + IFsrmObject_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RuleType(this: &Self::This) -> ::windows_core::Result<FsrmRuleType>;
    fn ModuleDefinitionName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModuleDefinitionName(this: &Self::This, moduledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamespaceRoots(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(this: &Self::This, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RuleFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRuleFlags(this: &Self::This, ruleflags: i32) -> ::windows_core::Result<()>;
    fn Parameters(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(this: &Self::This, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn LastModified(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn RuleType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RuleType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ruletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModuleDefinitionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleDefinitionName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinitionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetModuleDefinitionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModuleDefinitionName(this, ::core::mem::transmute(&moduledefinitionname)).into())
        }
        unsafe extern "system" fn NamespaceRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamespaceRoots(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceroots, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamespaceRoots(this, ::core::mem::transmute_copy(&namespaceroots)).into())
        }
        unsafe extern "system" fn RuleFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RuleFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ruleflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRuleFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRuleFlags(this, ::core::mem::transmute_copy(&ruleflags)).into())
        }
        unsafe extern "system" fn Parameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute_copy(&parameters)).into())
        }
        unsafe extern "system" fn LastModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastModified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmRule_Vtbl {
            base__: <IFsrmObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            RuleType: RuleType::<Identity, Impl, OFFSET>,
            ModuleDefinitionName: ModuleDefinitionName::<Identity, Impl, OFFSET>,
            SetModuleDefinitionName: SetModuleDefinitionName::<Identity, Impl, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, Impl, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, Impl, OFFSET>,
            RuleFlags: RuleFlags::<Identity, Impl, OFFSET>,
            SetRuleFlags: SetRuleFlags::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            LastModified: LastModified::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmSetting_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SmtpServer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSmtpServer(this: &Self::This, smtpserver: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MailFrom(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMailFrom(this: &Self::This, mailfrom: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AdminEmail(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAdminEmail(this: &Self::This, adminemail: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisableCommandLine(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableCommandLine(this: &Self::This, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EnableScreeningAudit(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableScreeningAudit(this: &Self::This, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EmailTest(this: &Self::This, mailto: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetActionRunLimitInterval(this: &Self::This, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows_core::Result<()>;
    fn GetActionRunLimitInterval(this: &Self::This, actiontype: FsrmActionType) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmSetting {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmSetting {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SmtpServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smtpserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmtpServer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smtpserver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSmtpServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smtpserver: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSmtpServer(this, ::core::mem::transmute(&smtpserver)).into())
        }
        unsafe extern "system" fn MailFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailfrom: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MailFrom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailfrom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMailFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailfrom: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMailFrom(this, ::core::mem::transmute(&mailfrom)).into())
        }
        unsafe extern "system" fn AdminEmail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adminemail: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminEmail(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(adminemail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAdminEmail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adminemail: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdminEmail(this, ::core::mem::transmute(&adminemail)).into())
        }
        unsafe extern "system" fn DisableCommandLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disablecommandline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisableCommandLine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disablecommandline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisableCommandLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisableCommandLine(this, ::core::mem::transmute_copy(&disablecommandline)).into())
        }
        unsafe extern "system" fn EnableScreeningAudit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnableScreeningAudit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enablescreeningaudit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableScreeningAudit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableScreeningAudit(this, ::core::mem::transmute_copy(&enablescreeningaudit)).into())
        }
        unsafe extern "system" fn EmailTest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmailTest(this, ::core::mem::transmute(&mailto)).into())
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionRunLimitInterval(this, ::core::mem::transmute_copy(&actiontype), ::core::mem::transmute_copy(&delaytimeminutes)).into())
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActionRunLimitInterval(this, ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(delaytimeminutes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsrmSetting_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SmtpServer: SmtpServer::<Identity, Impl, OFFSET>,
            SetSmtpServer: SetSmtpServer::<Identity, Impl, OFFSET>,
            MailFrom: MailFrom::<Identity, Impl, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, Impl, OFFSET>,
            AdminEmail: AdminEmail::<Identity, Impl, OFFSET>,
            SetAdminEmail: SetAdminEmail::<Identity, Impl, OFFSET>,
            DisableCommandLine: DisableCommandLine::<Identity, Impl, OFFSET>,
            SetDisableCommandLine: SetDisableCommandLine::<Identity, Impl, OFFSET>,
            EnableScreeningAudit: EnableScreeningAudit::<Identity, Impl, OFFSET>,
            SetEnableScreeningAudit: SetEnableScreeningAudit::<Identity, Impl, OFFSET>,
            EmailTest: EmailTest::<Identity, Impl, OFFSET>,
            SetActionRunLimitInterval: SetActionRunLimitInterval::<Identity, Impl, OFFSET>,
            GetActionRunLimitInterval: GetActionRunLimitInterval::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmStorageModuleDefinition_Impl: ::windows_core::BaseImpl + IFsrmPipelineModuleDefinition_Impl {
    fn Capabilities(this: &Self::This) -> ::windows_core::Result<FsrmStorageModuleCaps>;
    fn SetCapabilities(this: &Self::This, capabilities: FsrmStorageModuleCaps) -> ::windows_core::Result<()>;
    fn StorageType(this: &Self::This) -> ::windows_core::Result<FsrmStorageModuleType>;
    fn SetStorageType(this: &Self::This, storagetype: FsrmStorageModuleType) -> ::windows_core::Result<()>;
    fn UpdatesFileContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUpdatesFileContent(this: &Self::This, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmStorageModuleDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmPipelineModuleDefinition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmStorageModuleDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Capabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Capabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCapabilities(this, ::core::mem::transmute_copy(&capabilities)).into())
        }
        unsafe extern "system" fn StorageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StorageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStorageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStorageType(this, ::core::mem::transmute_copy(&storagetype)).into())
        }
        unsafe extern "system" fn UpdatesFileContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdatesFileContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(updatesfilecontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUpdatesFileContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpdatesFileContent(this, ::core::mem::transmute_copy(&updatesfilecontent)).into())
        }
        IFsrmStorageModuleDefinition_Vtbl {
            base__: <IFsrmPipelineModuleDefinition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            SetCapabilities: SetCapabilities::<Identity, Impl, OFFSET>,
            StorageType: StorageType::<Identity, Impl, OFFSET>,
            SetStorageType: SetStorageType::<Identity, Impl, OFFSET>,
            UpdatesFileContent: UpdatesFileContent::<Identity, Impl, OFFSET>,
            SetUpdatesFileContent: SetUpdatesFileContent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmStorageModuleImplementation_Impl: ::windows_core::BaseImpl + IFsrmPipelineModuleImplementation_Impl {
    fn UseDefinitions(this: &Self::This, propertydefinitions: ::core::option::Option<&IFsrmCollection>) -> ::windows_core::Result<()>;
    fn LoadProperties(this: &Self::This, propertybag: ::core::option::Option<&IFsrmPropertyBag>) -> ::windows_core::Result<()>;
    fn SaveProperties(this: &Self::This, propertybag: ::core::option::Option<&IFsrmPropertyBag>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsrmStorageModuleImplementation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsrmPipelineModuleImplementation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsrmStorageModuleImplementation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UseDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseDefinitions(this, ::windows_core::from_raw_borrowed(&propertydefinitions)).into())
        }
        unsafe extern "system" fn LoadProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadProperties(this, ::windows_core::from_raw_borrowed(&propertybag)).into())
        }
        unsafe extern "system" fn SaveProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveProperties(this, ::windows_core::from_raw_borrowed(&propertybag)).into())
        }
        IFsrmStorageModuleImplementation_Vtbl {
            base__: <IFsrmPipelineModuleImplementation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UseDefinitions: UseDefinitions::<Identity, Impl, OFFSET>,
            LoadProperties: LoadProperties::<Identity, Impl, OFFSET>,
            SaveProperties: SaveProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
