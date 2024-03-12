pub trait IDtcLuConfigure_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, puclupair: *const u8, cblupair: u32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, puclupair: *const u8, cblupair: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuConfigure {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuConfigure_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuConfigure {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuConfigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuConfigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)).into())
        }
        IDtcLuConfigure_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcLuRecovery_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDtcLuRecovery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecovery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecovery {
    const VTABLE: Self::Vtable = { IDtcLuRecovery_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDtcLuRecoveryFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, puclupair: *const u8, cblupair: u32) -> ::windows_core::Result<IDtcLuRecovery>;
}
impl ::windows_core::Iids for IDtcLuRecoveryFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecoveryFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecovery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDtcLuRecoveryFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
pub trait IDtcLuRecoveryInitiatedByDtc_Impl: ::windows_core::BaseImpl {
    fn GetWork(this: &Self::This, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuRecoveryInitiatedByDtc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecoveryInitiatedByDtc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWork(this, ::core::mem::transmute_copy(&pwork), ::core::mem::transmute_copy(&ppv)).into())
        }
        IDtcLuRecoveryInitiatedByDtc_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetWork: GetWork::<Identity, Impl, OFFSET> }
    };
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWork_Impl: ::windows_core::BaseImpl {
    fn HandleCheckLuStatus(this: &Self::This, lrecoveryseqnum: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuRecoveryInitiatedByDtcStatusWork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecoveryInitiatedByDtcStatusWork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleCheckLuStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleCheckLuStatus(this, ::core::mem::transmute_copy(&lrecoveryseqnum)).into())
        }
        IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleCheckLuStatus: HandleCheckLuStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRecoveryInitiatedByDtcTransWork_Impl: ::windows_core::BaseImpl {
    fn GetLogNameSizes(this: &Self::This, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows_core::Result<()>;
    fn GetOurXln(this: &Self::This, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::Result<()>;
    fn HandleConfirmationFromOurXln(this: &Self::This, confirmation: DTCLUXLNCONFIRMATION) -> ::windows_core::Result<()>;
    fn HandleTheirXlnResponse(this: &Self::This, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> ::windows_core::Result<()>;
    fn HandleErrorFromOurXln(this: &Self::This, error: DTCLUXLNERROR) -> ::windows_core::Result<()>;
    fn CheckForCompareStates(this: &Self::This, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetOurTransIdSize(this: &Self::This, pcbourtransid: *mut u32) -> ::windows_core::Result<()>;
    fn GetOurCompareStates(this: &Self::This, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> ::windows_core::Result<()>;
    fn HandleTheirCompareStatesResponse(this: &Self::This, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> ::windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(this: &Self::This, error: DTCLUCOMPARESTATESERROR) -> ::windows_core::Result<()>;
    fn ConversationLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRecoverySeqNum(this: &Self::This, plrecoveryseqnum: *mut i32) -> ::windows_core::Result<()>;
    fn ObsoleteRecoverySeqNum(this: &Self::This, lnewrecoveryseqnum: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcLuRecoveryInitiatedByDtcTransWork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecoveryInitiatedByDtcTransWork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLogNameSizes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLogNameSizes(this, ::core::mem::transmute_copy(&pcbourlogname), ::core::mem::transmute_copy(&pcbremotelogname)).into())
        }
        unsafe extern "system" fn GetOurXln<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOurXln(this, ::core::mem::transmute_copy(&pxln), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&pdwprotocol)).into())
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleConfirmationFromOurXln(this, ::core::mem::transmute_copy(&confirmation)).into())
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleTheirXlnResponse(this, ::core::mem::transmute_copy(&xln), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&cbremotelogname), ::core::mem::transmute_copy(&dwprotocol), ::core::mem::transmute_copy(&pconfirmation)).into())
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: DTCLUXLNERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleErrorFromOurXln(this, ::core::mem::transmute_copy(&error)).into())
        }
        unsafe extern "system" fn CheckForCompareStates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckForCompareStates(this, ::core::mem::transmute_copy(&fcomparestates)).into())
        }
        unsafe extern "system" fn GetOurTransIdSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOurTransIdSize(this, ::core::mem::transmute_copy(&pcbourtransid)).into())
        }
        unsafe extern "system" fn GetOurCompareStates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOurCompareStates(this, ::core::mem::transmute_copy(&pourtransid), ::core::mem::transmute_copy(&pcomparestate)).into())
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleTheirCompareStatesResponse(this, ::core::mem::transmute_copy(&comparestate), ::core::mem::transmute_copy(&pconfirmation)).into())
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleErrorFromOurCompareStates(this, ::core::mem::transmute_copy(&error)).into())
        }
        unsafe extern "system" fn ConversationLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConversationLost(this).into())
        }
        unsafe extern "system" fn GetRecoverySeqNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRecoverySeqNum(this, ::core::mem::transmute_copy(&plrecoveryseqnum)).into())
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ObsoleteRecoverySeqNum(this, ::core::mem::transmute_copy(&lnewrecoveryseqnum)).into())
        }
        IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLogNameSizes: GetLogNameSizes::<Identity, Impl, OFFSET>,
            GetOurXln: GetOurXln::<Identity, Impl, OFFSET>,
            HandleConfirmationFromOurXln: HandleConfirmationFromOurXln::<Identity, Impl, OFFSET>,
            HandleTheirXlnResponse: HandleTheirXlnResponse::<Identity, Impl, OFFSET>,
            HandleErrorFromOurXln: HandleErrorFromOurXln::<Identity, Impl, OFFSET>,
            CheckForCompareStates: CheckForCompareStates::<Identity, Impl, OFFSET>,
            GetOurTransIdSize: GetOurTransIdSize::<Identity, Impl, OFFSET>,
            GetOurCompareStates: GetOurCompareStates::<Identity, Impl, OFFSET>,
            HandleTheirCompareStatesResponse: HandleTheirCompareStatesResponse::<Identity, Impl, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, Impl, OFFSET>,
            ConversationLost: ConversationLost::<Identity, Impl, OFFSET>,
            GetRecoverySeqNum: GetRecoverySeqNum::<Identity, Impl, OFFSET>,
            ObsoleteRecoverySeqNum: ObsoleteRecoverySeqNum::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcLuRecoveryInitiatedByLu_Impl: ::windows_core::BaseImpl {
    fn GetObjectToHandleWorkFromLu(this: &Self::This) -> ::windows_core::Result<IDtcLuRecoveryInitiatedByLuWork>;
}
impl ::windows_core::Iids for IDtcLuRecoveryInitiatedByLu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecoveryInitiatedByLu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectToHandleWorkFromLu(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDtcLuRecoveryInitiatedByLu_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectToHandleWorkFromLu: GetObjectToHandleWorkFromLu::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcLuRecoveryInitiatedByLuWork_Impl: ::windows_core::BaseImpl {
    fn HandleTheirXln(this: &Self::This, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> ::windows_core::Result<()>;
    fn GetOurLogNameSize(this: &Self::This, pcbourlogname: *mut u32) -> ::windows_core::Result<()>;
    fn GetOurXln(this: &Self::This, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::Result<()>;
    fn HandleConfirmationOfOurXln(this: &Self::This, confirmation: DTCLUXLNCONFIRMATION) -> ::windows_core::Result<()>;
    fn HandleTheirCompareStates(this: &Self::This, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> ::windows_core::Result<()>;
    fn HandleConfirmationOfOurCompareStates(this: &Self::This, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> ::windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(this: &Self::This, error: DTCLUCOMPARESTATESERROR) -> ::windows_core::Result<()>;
    fn ConversationLost(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuRecoveryInitiatedByLuWork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRecoveryInitiatedByLuWork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleTheirXln<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleTheirXln(this, ::core::mem::transmute_copy(&lrecoveryseqnum), ::core::mem::transmute_copy(&xln), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&cbremotelogname), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&cbourlogname), ::core::mem::transmute_copy(&dwprotocol), ::core::mem::transmute_copy(&presponse)).into())
        }
        unsafe extern "system" fn GetOurLogNameSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOurLogNameSize(this, ::core::mem::transmute_copy(&pcbourlogname)).into())
        }
        unsafe extern "system" fn GetOurXln<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOurXln(this, ::core::mem::transmute_copy(&pxln), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&pdwprotocol)).into())
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleConfirmationOfOurXln(this, ::core::mem::transmute_copy(&confirmation)).into())
        }
        unsafe extern "system" fn HandleTheirCompareStates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleTheirCompareStates(this, ::core::mem::transmute_copy(&premotetransid), ::core::mem::transmute_copy(&cbremotetransid), ::core::mem::transmute_copy(&comparestate), ::core::mem::transmute_copy(&presponse), ::core::mem::transmute_copy(&pcomparestate)).into())
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleConfirmationOfOurCompareStates(this, ::core::mem::transmute_copy(&confirmation)).into())
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleErrorFromOurCompareStates(this, ::core::mem::transmute_copy(&error)).into())
        }
        unsafe extern "system" fn ConversationLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConversationLost(this).into())
        }
        IDtcLuRecoveryInitiatedByLuWork_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleTheirXln: HandleTheirXln::<Identity, Impl, OFFSET>,
            GetOurLogNameSize: GetOurLogNameSize::<Identity, Impl, OFFSET>,
            GetOurXln: GetOurXln::<Identity, Impl, OFFSET>,
            HandleConfirmationOfOurXln: HandleConfirmationOfOurXln::<Identity, Impl, OFFSET>,
            HandleTheirCompareStates: HandleTheirCompareStates::<Identity, Impl, OFFSET>,
            HandleConfirmationOfOurCompareStates: HandleConfirmationOfOurCompareStates::<Identity, Impl, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, Impl, OFFSET>,
            ConversationLost: ConversationLost::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRmEnlistment_Impl: ::windows_core::BaseImpl {
    fn Unplug(this: &Self::This, fconversationlost: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn BackedOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn Committed(this: &Self::This) -> ::windows_core::Result<()>;
    fn Forget(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestCommit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcLuRmEnlistment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRmEnlistment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Unplug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unplug(this, ::core::mem::transmute_copy(&fconversationlost)).into())
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackedOut(this).into())
        }
        unsafe extern "system" fn BackOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackOut(this).into())
        }
        unsafe extern "system" fn Committed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Committed(this).into())
        }
        unsafe extern "system" fn Forget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forget(this).into())
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestCommit(this).into())
        }
        IDtcLuRmEnlistment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Unplug: Unplug::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcLuRmEnlistmentFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, puclupair: *mut u8, cblupair: u32, pitransaction: ::core::option::Option<&ITransaction>, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::core::option::Option<&IDtcLuRmEnlistmentSink>, pprmenlistment: *mut ::core::option::Option<IDtcLuRmEnlistment>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuRmEnlistmentFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRmEnlistmentFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: *mut ::core::ffi::c_void, pprmenlistment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair), ::windows_core::from_raw_borrowed(&pitransaction), ::core::mem::transmute_copy(&ptransid), ::core::mem::transmute_copy(&cbtransid), ::windows_core::from_raw_borrowed(&prmenlistmentsink), ::core::mem::transmute_copy(&pprmenlistment)).into())
        }
        IDtcLuRmEnlistmentFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
pub trait IDtcLuRmEnlistmentSink_Impl: ::windows_core::BaseImpl {
    fn AckUnplug(this: &Self::This) -> ::windows_core::Result<()>;
    fn TmDown(this: &Self::This) -> ::windows_core::Result<()>;
    fn SessionLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackedOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn Committed(this: &Self::This) -> ::windows_core::Result<()>;
    fn Forget(this: &Self::This) -> ::windows_core::Result<()>;
    fn Prepare(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestCommit(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuRmEnlistmentSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuRmEnlistmentSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AckUnplug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AckUnplug(this).into())
        }
        unsafe extern "system" fn TmDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TmDown(this).into())
        }
        unsafe extern "system" fn SessionLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SessionLost(this).into())
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackedOut(this).into())
        }
        unsafe extern "system" fn BackOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackOut(this).into())
        }
        unsafe extern "system" fn Committed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Committed(this).into())
        }
        unsafe extern "system" fn Forget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forget(this).into())
        }
        unsafe extern "system" fn Prepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Prepare(this).into())
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestCommit(this).into())
        }
        IDtcLuRmEnlistmentSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AckUnplug: AckUnplug::<Identity, Impl, OFFSET>,
            TmDown: TmDown::<Identity, Impl, OFFSET>,
            SessionLost: SessionLost::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuSubordinateDtc_Impl: ::windows_core::BaseImpl {
    fn Unplug(this: &Self::This, fconversationlost: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn BackedOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn Committed(this: &Self::This) -> ::windows_core::Result<()>;
    fn Forget(this: &Self::This) -> ::windows_core::Result<()>;
    fn Prepare(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestCommit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcLuSubordinateDtc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuSubordinateDtc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Unplug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unplug(this, ::core::mem::transmute_copy(&fconversationlost)).into())
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackedOut(this).into())
        }
        unsafe extern "system" fn BackOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackOut(this).into())
        }
        unsafe extern "system" fn Committed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Committed(this).into())
        }
        unsafe extern "system" fn Forget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forget(this).into())
        }
        unsafe extern "system" fn Prepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Prepare(this).into())
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestCommit(this).into())
        }
        IDtcLuSubordinateDtc_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Unplug: Unplug::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcLuSubordinateDtcFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, puclupair: *mut u8, cblupair: u32, punktransactionouter: ::core::option::Option<&::windows_core::IUnknown>, isolevel: i32, isoflags: u32, poptions: ::core::option::Option<&ITransactionOptions>, pptransaction: *mut ::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::core::option::Option<&IDtcLuSubordinateDtcSink>, ppsubordinatedtc: *mut ::core::option::Option<IDtcLuSubordinateDtc>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuSubordinateDtcFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuSubordinateDtcFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: *mut ::core::ffi::c_void, ppsubordinatedtc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::Create(
                    this,
                    ::core::mem::transmute_copy(&puclupair),
                    ::core::mem::transmute_copy(&cblupair),
                    ::windows_core::from_raw_borrowed(&punktransactionouter),
                    ::core::mem::transmute_copy(&isolevel),
                    ::core::mem::transmute_copy(&isoflags),
                    ::windows_core::from_raw_borrowed(&poptions),
                    ::core::mem::transmute_copy(&pptransaction),
                    ::core::mem::transmute_copy(&ptransid),
                    ::core::mem::transmute_copy(&cbtransid),
                    ::windows_core::from_raw_borrowed(&psubordinatedtcsink),
                    ::core::mem::transmute_copy(&ppsubordinatedtc),
                )
                .into()
            })
        }
        IDtcLuSubordinateDtcFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
pub trait IDtcLuSubordinateDtcSink_Impl: ::windows_core::BaseImpl {
    fn AckUnplug(this: &Self::This) -> ::windows_core::Result<()>;
    fn TmDown(this: &Self::This) -> ::windows_core::Result<()>;
    fn SessionLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackedOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn Committed(this: &Self::This) -> ::windows_core::Result<()>;
    fn Forget(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestCommit(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcLuSubordinateDtcSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcLuSubordinateDtcSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AckUnplug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AckUnplug(this).into())
        }
        unsafe extern "system" fn TmDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TmDown(this).into())
        }
        unsafe extern "system" fn SessionLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SessionLost(this).into())
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackedOut(this).into())
        }
        unsafe extern "system" fn BackOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackOut(this).into())
        }
        unsafe extern "system" fn Committed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Committed(this).into())
        }
        unsafe extern "system" fn Forget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forget(this).into())
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestCommit(this).into())
        }
        IDtcLuSubordinateDtcSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AckUnplug: AckUnplug::<Identity, Impl, OFFSET>,
            TmDown: TmDown::<Identity, Impl, OFFSET>,
            SessionLost: SessionLost::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig_Impl: ::windows_core::BaseImpl {
    fn GetAnyNetworkAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAnyNetworkAccess(this: &Self::This, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetNetworkAdministrationAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkAdministrationAccess(this: &Self::This, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetNetworkTransactionAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTransactionAccess(this: &Self::This, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetNetworkClientAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkClientAccess(this: &Self::This, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetNetworkTIPAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTIPAccess(this: &Self::This, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetXAAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetXAAccess(this: &Self::This, bxaaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RestartDtcService(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcNetworkAccessConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcNetworkAccessConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAnyNetworkAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAnyNetworkAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbanynetworkaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAnyNetworkAccess(this, ::core::mem::transmute_copy(&banynetworkaccess)).into())
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkAdministrationAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworkadministrationaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkAdministrationAccess(this, ::core::mem::transmute_copy(&bnetworkadministrationaccess)).into())
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkTransactionAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworktransactionaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkTransactionAccess(this, ::core::mem::transmute_copy(&bnetworktransactionaccess)).into())
        }
        unsafe extern "system" fn GetNetworkClientAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkClientAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworkclientaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkClientAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkClientAccess(this, ::core::mem::transmute_copy(&bnetworkclientaccess)).into())
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkTIPAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworktipaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkTIPAccess(this, ::core::mem::transmute_copy(&bnetworktipaccess)).into())
        }
        unsafe extern "system" fn GetXAAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXAAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbxaaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetXAAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXAAccess(this, ::core::mem::transmute_copy(&bxaaccess)).into())
        }
        unsafe extern "system" fn RestartDtcService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestartDtcService(this).into())
        }
        IDtcNetworkAccessConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAnyNetworkAccess: GetAnyNetworkAccess::<Identity, Impl, OFFSET>,
            SetAnyNetworkAccess: SetAnyNetworkAccess::<Identity, Impl, OFFSET>,
            GetNetworkAdministrationAccess: GetNetworkAdministrationAccess::<Identity, Impl, OFFSET>,
            SetNetworkAdministrationAccess: SetNetworkAdministrationAccess::<Identity, Impl, OFFSET>,
            GetNetworkTransactionAccess: GetNetworkTransactionAccess::<Identity, Impl, OFFSET>,
            SetNetworkTransactionAccess: SetNetworkTransactionAccess::<Identity, Impl, OFFSET>,
            GetNetworkClientAccess: GetNetworkClientAccess::<Identity, Impl, OFFSET>,
            SetNetworkClientAccess: SetNetworkClientAccess::<Identity, Impl, OFFSET>,
            GetNetworkTIPAccess: GetNetworkTIPAccess::<Identity, Impl, OFFSET>,
            SetNetworkTIPAccess: SetNetworkTIPAccess::<Identity, Impl, OFFSET>,
            GetXAAccess: GetXAAccess::<Identity, Impl, OFFSET>,
            SetXAAccess: SetXAAccess::<Identity, Impl, OFFSET>,
            RestartDtcService: RestartDtcService::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig2_Impl: ::windows_core::BaseImpl + IDtcNetworkAccessConfig_Impl {
    fn GetNetworkInboundAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetNetworkOutboundAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkInboundAccess(this: &Self::This, binbound: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetNetworkOutboundAccess(this: &Self::This, boutbound: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAuthenticationLevel(this: &Self::This) -> ::windows_core::Result<AUTHENTICATION_LEVEL>;
    fn SetAuthenticationLevel(this: &Self::This, authlevel: AUTHENTICATION_LEVEL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcNetworkAccessConfig2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDtcNetworkAccessConfig);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcNetworkAccessConfig2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNetworkInboundAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkInboundAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbinbound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkOutboundAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboutbound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkInboundAccess(this, ::core::mem::transmute_copy(&binbound)).into())
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkOutboundAccess(this, ::core::mem::transmute_copy(&boutbound)).into())
        }
        unsafe extern "system" fn GetAuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAuthenticationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauthlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationLevel(this, ::core::mem::transmute_copy(&authlevel)).into())
        }
        IDtcNetworkAccessConfig2_Vtbl {
            base__: <IDtcNetworkAccessConfig as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNetworkInboundAccess: GetNetworkInboundAccess::<Identity, Impl, OFFSET>,
            GetNetworkOutboundAccess: GetNetworkOutboundAccess::<Identity, Impl, OFFSET>,
            SetNetworkInboundAccess: SetNetworkInboundAccess::<Identity, Impl, OFFSET>,
            SetNetworkOutboundAccess: SetNetworkOutboundAccess::<Identity, Impl, OFFSET>,
            GetAuthenticationLevel: GetAuthenticationLevel::<Identity, Impl, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig3_Impl: ::windows_core::BaseImpl + IDtcNetworkAccessConfig2_Impl {
    fn GetLUAccess(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetLUAccess(this: &Self::This, bluaccess: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcNetworkAccessConfig3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDtcNetworkAccessConfig2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcNetworkAccessConfig3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLUAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLUAccess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbluaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLUAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLUAccess(this, ::core::mem::transmute_copy(&bluaccess)).into())
        }
        IDtcNetworkAccessConfig3_Vtbl {
            base__: <IDtcNetworkAccessConfig2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLUAccess: GetLUAccess::<Identity, Impl, OFFSET>,
            SetLUAccess: SetLUAccess::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelper_Impl: ::windows_core::BaseImpl {
    fn Close(this: &Self::This, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn TranslateTridToXid(this: &Self::This, pitransaction: ::core::option::Option<&ITransaction>, pguidbqual: *const ::windows_core::GUID, pxid: *mut XID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcToXaHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcToXaHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::core::mem::transmute_copy(&i_fdorecovery)).into())
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitransaction: *mut ::core::ffi::c_void, pguidbqual: *const ::windows_core::GUID, pxid: *mut XID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateTridToXid(this, ::windows_core::from_raw_borrowed(&pitransaction), ::core::mem::transmute_copy(&pguidbqual), ::core::mem::transmute_copy(&pxid)).into())
        }
        IDtcToXaHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Close: Close::<Identity, Impl, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcToXaHelperFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, pszdsn: &::windows_core::PCSTR, pszclientdllname: &::windows_core::PCSTR, pguidrm: *mut ::windows_core::GUID, ppxahelper: *mut ::core::option::Option<IDtcToXaHelper>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcToXaHelperFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcToXaHelperFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdsn: ::windows_core::PCSTR, pszclientdllname: ::windows_core::PCSTR, pguidrm: *mut ::windows_core::GUID, ppxahelper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute(&pszdsn), ::core::mem::transmute(&pszclientdllname), ::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute_copy(&ppxahelper)).into())
        }
        IDtcToXaHelperFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelperSinglePipe_Impl: ::windows_core::BaseImpl {
    fn XARMCreate(this: &Self::This, pszdsn: &::windows_core::PCSTR, pszclientdll: &::windows_core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertTridToXID(this: &Self::This, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> ::windows_core::Result<()>;
    fn EnlistWithRM(this: &Self::This, dwrmcookie: u32, i_pitransaction: ::core::option::Option<&ITransaction>, i_pitransres: ::core::option::Option<&ITransactionResourceAsync>) -> ::windows_core::Result<ITransactionEnlistmentAsync>;
    fn ReleaseRMCookie(this: &Self::This, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDtcToXaHelperSinglePipe {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcToXaHelperSinglePipe {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn XARMCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdsn: ::windows_core::PCSTR, pszclientdll: ::windows_core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::XARMCreate(this, ::core::mem::transmute(&pszdsn), ::core::mem::transmute(&pszclientdll), ::core::mem::transmute_copy(&pdwrmcookie)).into())
        }
        unsafe extern "system" fn ConvertTridToXID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertTridToXID(this, ::core::mem::transmute_copy(&pdwitrans), ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pxid)).into())
        }
        unsafe extern "system" fn EnlistWithRM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: *mut ::core::ffi::c_void, i_pitransres: *mut ::core::ffi::c_void, o_ppitransenslitment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnlistWithRM(this, ::core::mem::transmute_copy(&dwrmcookie), ::windows_core::from_raw_borrowed(&i_pitransaction), ::windows_core::from_raw_borrowed(&i_pitransres)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppitransenslitment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseRMCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseRMCookie(this, ::core::mem::transmute_copy(&i_dwrmcookie), ::core::mem::transmute_copy(&i_fnormal)))
        }
        IDtcToXaHelperSinglePipe_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            XARMCreate: XARMCreate::<Identity, Impl, OFFSET>,
            ConvertTridToXID: ConvertTridToXID::<Identity, Impl, OFFSET>,
            EnlistWithRM: EnlistWithRM::<Identity, Impl, OFFSET>,
            ReleaseRMCookie: ReleaseRMCookie::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDtcToXaMapper_Impl: ::windows_core::BaseImpl {
    fn RequestNewResourceManager(this: &Self::This, pszdsn: &::windows_core::PCSTR, pszclientdllname: &::windows_core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_core::Result<()>;
    fn TranslateTridToXid(this: &Self::This, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> ::windows_core::Result<()>;
    fn EnlistResourceManager(this: &Self::This, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows_core::Result<()>;
    fn ReleaseResourceManager(this: &Self::This, dwrmcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDtcToXaMapper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDtcToXaMapper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestNewResourceManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdsn: ::windows_core::PCSTR, pszclientdllname: ::windows_core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestNewResourceManager(this, ::core::mem::transmute(&pszdsn), ::core::mem::transmute(&pszclientdllname), ::core::mem::transmute_copy(&pdwrmcookie)).into())
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateTridToXid(this, ::core::mem::transmute_copy(&pdwitransaction), ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pxid)).into())
        }
        unsafe extern "system" fn EnlistResourceManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnlistResourceManager(this, ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pdwitransaction)).into())
        }
        unsafe extern "system" fn ReleaseResourceManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseResourceManager(this, ::core::mem::transmute_copy(&dwrmcookie)).into())
        }
        IDtcToXaMapper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestNewResourceManager: RequestNewResourceManager::<Identity, Impl, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, Impl, OFFSET>,
            EnlistResourceManager: EnlistResourceManager::<Identity, Impl, OFFSET>,
            ReleaseResourceManager: ReleaseResourceManager::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGetDispenser_Impl: ::windows_core::BaseImpl {
    fn GetDispenser(this: &Self::This, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGetDispenser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetDispenser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetDispenser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDispenser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetDispenser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDispenser(this, ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        IGetDispenser_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDispenser: GetDispenser::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKernelTransaction_Impl: ::windows_core::BaseImpl {
    fn GetHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKernelTransaction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKernelTransaction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKernelTransaction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKernelTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKernelTransaction_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetHandle: GetHandle::<Identity, Impl, OFFSET> }
    };
}
pub trait ILastResourceManager_Impl: ::windows_core::BaseImpl {
    fn TransactionCommitted(this: &Self::This, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows_core::Result<()>;
    fn RecoveryDone(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILastResourceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILastResourceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILastResourceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransactionCommitted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILastResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransactionCommitted(this, ::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo)).into())
        }
        unsafe extern "system" fn RecoveryDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILastResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecoveryDone(this).into())
        }
        ILastResourceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransactionCommitted: TransactionCommitted::<Identity, Impl, OFFSET>,
            RecoveryDone: RecoveryDone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrepareInfo_Impl: ::windows_core::BaseImpl {
    fn GetPrepareInfoSize(this: &Self::This, pcbprepinfo: *mut u32) -> ::windows_core::Result<()>;
    fn GetPrepareInfo(this: &Self::This, pprepinfo: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrepareInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrepareInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrepareInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrepareInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrepareInfoSize(this, ::core::mem::transmute_copy(&pcbprepinfo)).into())
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrepareInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrepareInfo(this, ::core::mem::transmute_copy(&pprepinfo)).into())
        }
        IPrepareInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, Impl, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrepareInfo2_Impl: ::windows_core::BaseImpl {
    fn GetPrepareInfoSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPrepareInfo(this: &Self::This, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrepareInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrepareInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrepareInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrepareInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrepareInfoSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbprepinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrepareInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrepareInfo(this, ::core::mem::transmute_copy(&cbprepareinfo), ::core::mem::transmute_copy(&pprepinfo)).into())
        }
        IPrepareInfo2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, Impl, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRMHelper_Impl: ::windows_core::BaseImpl {
    fn RMCount(this: &Self::This, dwctotalnumberofrms: u32) -> ::windows_core::Result<()>;
    fn RMInfo(this: &Self::This, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: &::windows_core::PCSTR, pszclosestring: &::windows_core::PCSTR, guidrmrecovery: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRMHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRMHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRMHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RMCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRMHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RMCount(this, ::core::mem::transmute_copy(&dwctotalnumberofrms)).into())
        }
        unsafe extern "system" fn RMInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRMHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: ::windows_core::PCSTR, pszclosestring: ::windows_core::PCSTR, guidrmrecovery: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RMInfo(this, ::core::mem::transmute_copy(&pxa_switch), ::core::mem::transmute_copy(&fcdeclcallingconv), ::core::mem::transmute(&pszopenstring), ::core::mem::transmute(&pszclosestring), ::core::mem::transmute(&guidrmrecovery)).into())
        }
        IRMHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RMCount: RMCount::<Identity, Impl, OFFSET>,
            RMInfo: RMInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IResourceManager_Impl: ::windows_core::BaseImpl {
    fn Enlist(this: &Self::This, ptransaction: ::core::option::Option<&ITransaction>, pres: ::core::option::Option<&ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()>;
    fn Reenlist(this: &Self::This, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows_core::Result<XACTSTAT>;
    fn ReenlistmentComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDistributedTransactionManager(this: &Self::This, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IResourceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResourceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enlist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, pres: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enlist(this, ::windows_core::from_raw_borrowed(&ptransaction), ::windows_core::from_raw_borrowed(&pres), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&ppenlist)).into())
        }
        unsafe extern "system" fn Reenlist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Reenlist(this, ::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo), ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxactstat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReenlistmentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReenlistmentComplete(this).into())
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDistributedTransactionManager(this, ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        IResourceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enlist: Enlist::<Identity, Impl, OFFSET>,
            Reenlist: Reenlist::<Identity, Impl, OFFSET>,
            ReenlistmentComplete: ReenlistmentComplete::<Identity, Impl, OFFSET>,
            GetDistributedTransactionManager: GetDistributedTransactionManager::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IResourceManager2_Impl: ::windows_core::BaseImpl + IResourceManager_Impl {
    fn Enlist2(this: &Self::This, ptransaction: ::core::option::Option<&ITransaction>, presasync: ::core::option::Option<&ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()>;
    fn Reenlist2(this: &Self::This, pxid: *const XID, dwtimeout: u32) -> ::windows_core::Result<XACTSTAT>;
}
impl ::windows_core::Iids for IResourceManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IResourceManager);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResourceManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enlist2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, presasync: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enlist2(this, ::windows_core::from_raw_borrowed(&ptransaction), ::windows_core::from_raw_borrowed(&presasync), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&ppenlist)).into())
        }
        unsafe extern "system" fn Reenlist2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxid: *const XID, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Reenlist2(this, ::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxactstat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IResourceManager2_Vtbl {
            base__: <IResourceManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enlist2: Enlist2::<Identity, Impl, OFFSET>,
            Reenlist2: Reenlist2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IResourceManagerFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, pguidrm: *const ::windows_core::GUID, pszrmname: &::windows_core::PCSTR, piresmgrsink: ::core::option::Option<&IResourceManagerSink>) -> ::windows_core::Result<IResourceManager>;
}
impl ::windows_core::Iids for IResourceManagerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResourceManagerFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows_core::GUID, pszrmname: ::windows_core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, ppresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute(&pszrmname), ::windows_core::from_raw_borrowed(&piresmgrsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresmgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IResourceManagerFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
pub trait IResourceManagerFactory2_Impl: ::windows_core::BaseImpl + IResourceManagerFactory_Impl {
    fn CreateEx(this: &Self::This, pguidrm: *const ::windows_core::GUID, pszrmname: &::windows_core::PCSTR, piresmgrsink: ::core::option::Option<&IResourceManagerSink>, riidrequested: *const ::windows_core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IResourceManagerFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IResourceManagerFactory);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResourceManagerFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows_core::GUID, pszrmname: ::windows_core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, riidrequested: *const ::windows_core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateEx(this, ::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute(&pszrmname), ::windows_core::from_raw_borrowed(&piresmgrsink), ::core::mem::transmute_copy(&riidrequested), ::core::mem::transmute_copy(&ppvresmgr)).into())
        }
        IResourceManagerFactory2_Vtbl { base__: <IResourceManagerFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateEx: CreateEx::<Identity, Impl, OFFSET> }
    };
}
pub trait IResourceManagerRejoinable_Impl: ::windows_core::BaseImpl + IResourceManager2_Impl {
    fn Rejoin(this: &Self::This, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows_core::Result<XACTSTAT>;
}
impl ::windows_core::Iids for IResourceManagerRejoinable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IResourceManager2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerRejoinable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResourceManagerRejoinable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Rejoin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerRejoinable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rejoin(this, ::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo), ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxactstat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IResourceManagerRejoinable_Vtbl { base__: <IResourceManager2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Rejoin: Rejoin::<Identity, Impl, OFFSET> }
    };
}
pub trait IResourceManagerSink_Impl: ::windows_core::BaseImpl {
    fn TMDown(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IResourceManagerSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResourceManagerSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TMDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResourceManagerSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TMDown(this).into())
        }
        IResourceManagerSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TMDown: TMDown::<Identity, Impl, OFFSET> }
    };
}
pub trait ITipHelper_Impl: ::windows_core::BaseImpl {
    fn Pull(this: &Self::This, i_psztxurl: *const u8) -> ::windows_core::Result<ITransaction>;
    fn PullAsync(this: &Self::This, i_psztxurl: *const u8, i_ptippullsink: ::core::option::Option<&ITipPullSink>) -> ::windows_core::Result<ITransaction>;
    fn GetLocalTmUrl(this: &Self::This) -> ::windows_core::Result<*mut u8>;
}
impl ::windows_core::Iids for ITipHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITipHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pull(this, ::core::mem::transmute_copy(&i_psztxurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppitransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PullAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: *mut ::core::ffi::c_void, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PullAsync(this, ::core::mem::transmute_copy(&i_psztxurl), ::windows_core::from_raw_borrowed(&i_ptippullsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppitransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalTmUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalTmUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppszlocaltmurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITipHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Pull: Pull::<Identity, Impl, OFFSET>,
            PullAsync: PullAsync::<Identity, Impl, OFFSET>,
            GetLocalTmUrl: GetLocalTmUrl::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITipPullSink_Impl: ::windows_core::BaseImpl {
    fn PullComplete(this: &Self::This, i_hrpull: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITipPullSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipPullSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITipPullSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PullComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipPullSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_hrpull: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PullComplete(this, ::core::mem::transmute_copy(&i_hrpull)).into())
        }
        ITipPullSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, PullComplete: PullComplete::<Identity, Impl, OFFSET> }
    };
}
pub trait ITipTransaction_Impl: ::windows_core::BaseImpl {
    fn Push(this: &Self::This, i_pszremotetmurl: *const u8) -> ::windows_core::Result<::windows_core::PSTR>;
    fn GetTransactionUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::PSTR>;
}
impl ::windows_core::Iids for ITipTransaction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipTransaction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITipTransaction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Push(this, ::core::mem::transmute_copy(&i_pszremotetmurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppszremotetxurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransactionUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITipTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransactionUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppszlocaltxurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITipTransaction_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Push: Push::<Identity, Impl, OFFSET>,
            GetTransactionUrl: GetTransactionUrl::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITmNodeName_Impl: ::windows_core::BaseImpl {
    fn GetNodeNameSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNodeName(this: &Self::This, cbnodenamebuffersize: u32, pnodenamebuffer: &::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITmNodeName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITmNodeName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITmNodeName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNodeNameSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITmNodeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNodeNameSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbnodenamesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNodeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITmNodeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNodeName(this, ::core::mem::transmute_copy(&cbnodenamebuffersize), ::core::mem::transmute(&pnodenamebuffer)).into())
        }
        ITmNodeName_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNodeNameSize: GetNodeNameSize::<Identity, Impl, OFFSET>,
            GetNodeName: GetNodeName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransaction_Impl: ::windows_core::BaseImpl {
    fn Commit(this: &Self::This, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetTransactionInfo(this: &Self::This, pinfo: *mut XACTTRANSINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransaction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransaction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this, ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into())
        }
        unsafe extern "system" fn GetTransactionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransactionInfo(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        ITransaction_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetTransactionInfo: GetTransactionInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransaction2_Impl: ::windows_core::BaseImpl + ITransactionCloner_Impl {
    fn GetTransactionInfo2(this: &Self::This, pinfo: *mut XACTTRANSINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransaction2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITransactionCloner);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransaction2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransaction2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransactionInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransaction2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransactionInfo2(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        ITransaction2_Vtbl {
            base__: <ITransactionCloner as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransactionInfo2: GetTransactionInfo2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionCloner_Impl: ::windows_core::BaseImpl + ITransaction_Impl {
    fn CloneWithCommitDisabled(this: &Self::This) -> ::windows_core::Result<ITransaction>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransactionCloner {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITransaction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionCloner_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionCloner {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CloneWithCommitDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionCloner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CloneWithCommitDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionCloner_Vtbl {
            base__: <ITransaction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CloneWithCommitDisabled: CloneWithCommitDisabled::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionDispenser_Impl: ::windows_core::BaseImpl {
    fn GetOptionsObject(this: &Self::This) -> ::windows_core::Result<ITransactionOptions>;
    fn BeginTransaction(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, isolevel: i32, isoflags: u32, poptions: ::core::option::Option<&ITransactionOptions>) -> ::windows_core::Result<ITransaction>;
}
impl ::windows_core::Iids for ITransactionDispenser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionDispenser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionDispenser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionDispenser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionsObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionDispenser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::windows_core::from_raw_borrowed(&poptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionDispenser_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionEnlistmentAsync_Impl: ::windows_core::BaseImpl {
    fn PrepareRequestDone(this: &Self::This, hr: ::windows_core::HRESULT, pmk: ::core::option::Option<&super::Com::IMoniker>, pboidreason: *const BOID) -> ::windows_core::Result<()>;
    fn CommitRequestDone(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn AbortRequestDone(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITransactionEnlistmentAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionEnlistmentAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrepareRequestDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, pmk: *mut ::core::ffi::c_void, pboidreason: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareRequestDone(this, ::core::mem::transmute_copy(&hr), ::windows_core::from_raw_borrowed(&pmk), ::core::mem::transmute_copy(&pboidreason)).into())
        }
        unsafe extern "system" fn CommitRequestDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitRequestDone(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn AbortRequestDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortRequestDone(this, ::core::mem::transmute_copy(&hr)).into())
        }
        ITransactionEnlistmentAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrepareRequestDone: PrepareRequestDone::<Identity, Impl, OFFSET>,
            CommitRequestDone: CommitRequestDone::<Identity, Impl, OFFSET>,
            AbortRequestDone: AbortRequestDone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionExport_Impl: ::windows_core::BaseImpl {
    fn Export(this: &Self::This, punktransaction: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
    fn GetTransactionCookie(this: &Self::This, punktransaction: ::core::option::Option<&::windows_core::IUnknown>, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionExport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionExport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionExport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Export<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionExport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Export(this, ::windows_core::from_raw_borrowed(&punktransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbtransactioncookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransactionCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionExport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransactionCookie(this, ::windows_core::from_raw_borrowed(&punktransaction), ::core::mem::transmute_copy(&cbtransactioncookie), ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&pcbused)).into())
        }
        ITransactionExport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Export: Export::<Identity, Impl, OFFSET>,
            GetTransactionCookie: GetTransactionCookie::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionExportFactory_Impl: ::windows_core::BaseImpl {
    fn GetRemoteClassId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Create(this: &Self::This, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> ::windows_core::Result<ITransactionExport>;
}
impl ::windows_core::Iids for ITransactionExportFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionExportFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionExportFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRemoteClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionExportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRemoteClassId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionExportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&cbwhereabouts), ::core::mem::transmute_copy(&rgbwhereabouts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppexport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionExportFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRemoteClassId: GetRemoteClassId::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionImport_Impl: ::windows_core::BaseImpl {
    fn Import(this: &Self::This, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows_core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionImport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionImport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionImport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionImport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows_core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Import(this, ::core::mem::transmute_copy(&cbtransactioncookie), ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&ppvtransaction)).into())
        }
        ITransactionImport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Import: Import::<Identity, Impl, OFFSET> }
    };
}
pub trait ITransactionImportWhereabouts_Impl: ::windows_core::BaseImpl {
    fn GetWhereaboutsSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetWhereabouts(this: &Self::This, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionImportWhereabouts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionImportWhereabouts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionImportWhereabouts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWhereaboutsSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionImportWhereabouts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWhereaboutsSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwhereabouts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWhereabouts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionImportWhereabouts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWhereabouts(this, ::core::mem::transmute_copy(&cbwhereabouts), ::core::mem::transmute_copy(&rgbwhereabouts), ::core::mem::transmute_copy(&pcbused)).into())
        }
        ITransactionImportWhereabouts_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWhereaboutsSize: GetWhereaboutsSize::<Identity, Impl, OFFSET>,
            GetWhereabouts: GetWhereabouts::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionLastEnlistmentAsync_Impl: ::windows_core::BaseImpl {
    fn TransactionOutcome(this: &Self::This, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionLastEnlistmentAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLastEnlistmentAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionLastEnlistmentAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransactionOutcome<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLastEnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransactionOutcome(this, ::core::mem::transmute_copy(&xactstat), ::core::mem::transmute_copy(&pboidreason)).into())
        }
        ITransactionLastEnlistmentAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransactionOutcome: TransactionOutcome::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionLastResourceAsync_Impl: ::windows_core::BaseImpl {
    fn DelegateCommit(this: &Self::This, grfrm: u32) -> ::windows_core::Result<()>;
    fn ForgetRequest(this: &Self::This, pnewuow: *const BOID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionLastResourceAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLastResourceAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionLastResourceAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DelegateCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLastResourceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DelegateCommit(this, ::core::mem::transmute_copy(&grfrm)).into())
        }
        unsafe extern "system" fn ForgetRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLastResourceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForgetRequest(this, ::core::mem::transmute_copy(&pnewuow)).into())
        }
        ITransactionLastResourceAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DelegateCommit: DelegateCommit::<Identity, Impl, OFFSET>,
            ForgetRequest: ForgetRequest::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionOptions_Impl: ::windows_core::BaseImpl {
    fn SetOptions(this: &Self::This, poptions: *const XACTOPT) -> ::windows_core::Result<()>;
    fn GetOptions(this: &Self::This, poptions: *mut XACTOPT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOptions(this, ::core::mem::transmute_copy(&poptions)).into())
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptions(this, ::core::mem::transmute_copy(&poptions)).into())
        }
        ITransactionOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionOutcomeEvents_Impl: ::windows_core::BaseImpl {
    fn Committed(this: &Self::This, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn Aborted(this: &Self::This, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn HeuristicDecision(this: &Self::This, dwdecision: u32, pboidreason: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn Indoubt(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransactionOutcomeEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionOutcomeEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Committed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Committed(this, ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn Aborted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Aborted(this, ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn HeuristicDecision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HeuristicDecision(this, ::core::mem::transmute_copy(&dwdecision), ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn Indoubt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Indoubt(this).into())
        }
        ITransactionOutcomeEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Aborted: Aborted::<Identity, Impl, OFFSET>,
            HeuristicDecision: HeuristicDecision::<Identity, Impl, OFFSET>,
            Indoubt: Indoubt::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionPhase0EnlistmentAsync_Impl: ::windows_core::BaseImpl {
    fn Enable(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForEnlistment(this: &Self::This) -> ::windows_core::Result<()>;
    fn Phase0Done(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unenlist(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetTransaction(this: &Self::This) -> ::windows_core::Result<ITransaction>;
}
impl ::windows_core::Iids for ITransactionPhase0EnlistmentAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionPhase0EnlistmentAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this).into())
        }
        unsafe extern "system" fn WaitForEnlistment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForEnlistment(this).into())
        }
        unsafe extern "system" fn Phase0Done<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Phase0Done(this).into())
        }
        unsafe extern "system" fn Unenlist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unenlist(this).into())
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionPhase0EnlistmentAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enable: Enable::<Identity, Impl, OFFSET>,
            WaitForEnlistment: WaitForEnlistment::<Identity, Impl, OFFSET>,
            Phase0Done: Phase0Done::<Identity, Impl, OFFSET>,
            Unenlist: Unenlist::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionPhase0Factory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, pphase0notify: ::core::option::Option<&ITransactionPhase0NotifyAsync>) -> ::windows_core::Result<ITransactionPhase0EnlistmentAsync>;
}
impl ::windows_core::Iids for ITransactionPhase0Factory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0Factory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionPhase0Factory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphase0notify: *mut ::core::ffi::c_void, ppphase0enlistment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::windows_core::from_raw_borrowed(&pphase0notify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphase0enlistment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionPhase0Factory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionPhase0NotifyAsync_Impl: ::windows_core::BaseImpl {
    fn Phase0Request(this: &Self::This, fabortinghint: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnlistCompleted(this: &Self::This, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransactionPhase0NotifyAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionPhase0NotifyAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Phase0Request<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Phase0Request(this, ::core::mem::transmute_copy(&fabortinghint)).into())
        }
        unsafe extern "system" fn EnlistCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnlistCompleted(this, ::core::mem::transmute_copy(&status)).into())
        }
        ITransactionPhase0NotifyAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Phase0Request: Phase0Request::<Identity, Impl, OFFSET>,
            EnlistCompleted: EnlistCompleted::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionReceiver_Impl: ::windows_core::BaseImpl {
    fn UnmarshalPropagationToken(this: &Self::This, cbtoken: u32, rgbtoken: *const u8) -> ::windows_core::Result<ITransaction>;
    fn GetReturnTokenSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MarshalReturnToken(this: &Self::This, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionReceiver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionReceiver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UnmarshalPropagationToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnmarshalPropagationToken(this, ::core::mem::transmute_copy(&cbtoken), ::core::mem::transmute_copy(&rgbtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReturnTokenSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReturnTokenSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbreturntoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MarshalReturnToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MarshalReturnToken(this, ::core::mem::transmute_copy(&cbreturntoken), ::core::mem::transmute_copy(&rgbreturntoken), ::core::mem::transmute_copy(&pcbused)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        ITransactionReceiver_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UnmarshalPropagationToken: UnmarshalPropagationToken::<Identity, Impl, OFFSET>,
            GetReturnTokenSize: GetReturnTokenSize::<Identity, Impl, OFFSET>,
            MarshalReturnToken: MarshalReturnToken::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionReceiverFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This) -> ::windows_core::Result<ITransactionReceiver>;
}
impl ::windows_core::Iids for ITransactionReceiverFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiverFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionReceiverFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionReceiverFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreceiver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreceiver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionReceiverFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResource_Impl: ::windows_core::BaseImpl {
    fn PrepareRequest(this: &Self::This, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CommitRequest(this: &Self::This, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::Result<()>;
    fn AbortRequest(this: &Self::This, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_core::Result<()>;
    fn TMDown(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransactionResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrepareRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareRequest(this, ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&fwantmoniker), ::core::mem::transmute_copy(&fsinglephase)).into())
        }
        unsafe extern "system" fn CommitRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitRequest(this, ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&pnewuow)).into())
        }
        unsafe extern "system" fn AbortRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortRequest(this, ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow)).into())
        }
        unsafe extern "system" fn TMDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TMDown(this).into())
        }
        ITransactionResource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrepareRequest: PrepareRequest::<Identity, Impl, OFFSET>,
            CommitRequest: CommitRequest::<Identity, Impl, OFFSET>,
            AbortRequest: AbortRequest::<Identity, Impl, OFFSET>,
            TMDown: TMDown::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResourceAsync_Impl: ::windows_core::BaseImpl {
    fn PrepareRequest(this: &Self::This, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CommitRequest(this: &Self::This, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::Result<()>;
    fn AbortRequest(this: &Self::This, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_core::Result<()>;
    fn TMDown(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransactionResourceAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionResourceAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrepareRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareRequest(this, ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&fwantmoniker), ::core::mem::transmute_copy(&fsinglephase)).into())
        }
        unsafe extern "system" fn CommitRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitRequest(this, ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&pnewuow)).into())
        }
        unsafe extern "system" fn AbortRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortRequest(this, ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow)).into())
        }
        unsafe extern "system" fn TMDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TMDown(this).into())
        }
        ITransactionResourceAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrepareRequest: PrepareRequest::<Identity, Impl, OFFSET>,
            CommitRequest: CommitRequest::<Identity, Impl, OFFSET>,
            AbortRequest: AbortRequest::<Identity, Impl, OFFSET>,
            TMDown: TMDown::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionTransmitter_Impl: ::windows_core::BaseImpl {
    fn Set(this: &Self::This, ptransaction: ::core::option::Option<&ITransaction>) -> ::windows_core::Result<()>;
    fn GetPropagationTokenSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MarshalPropagationToken(this: &Self::This, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows_core::Result<()>;
    fn UnmarshalReturnToken(this: &Self::This, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionTransmitter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionTransmitter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::windows_core::from_raw_borrowed(&ptransaction)).into())
        }
        unsafe extern "system" fn GetPropagationTokenSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropagationTokenSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MarshalPropagationToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MarshalPropagationToken(this, ::core::mem::transmute_copy(&cbtoken), ::core::mem::transmute_copy(&rgbtoken), ::core::mem::transmute_copy(&pcbused)).into())
        }
        unsafe extern "system" fn UnmarshalReturnToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnmarshalReturnToken(this, ::core::mem::transmute_copy(&cbreturntoken), ::core::mem::transmute_copy(&rgbreturntoken)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        ITransactionTransmitter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Set: Set::<Identity, Impl, OFFSET>,
            GetPropagationTokenSize: GetPropagationTokenSize::<Identity, Impl, OFFSET>,
            MarshalPropagationToken: MarshalPropagationToken::<Identity, Impl, OFFSET>,
            UnmarshalReturnToken: UnmarshalReturnToken::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionTransmitterFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This) -> ::windows_core::Result<ITransactionTransmitter>;
}
impl ::windows_core::Iids for ITransactionTransmitterFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitterFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionTransmitterFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionTransmitterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptransmitter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransmitter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionTransmitterFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
pub trait ITransactionVoterBallotAsync2_Impl: ::windows_core::BaseImpl {
    fn VoteRequestDone(this: &Self::This, hr: ::windows_core::HRESULT, pboidreason: *const BOID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionVoterBallotAsync2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionVoterBallotAsync2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionVoterBallotAsync2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VoteRequestDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionVoterBallotAsync2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, pboidreason: *const BOID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VoteRequestDone(this, ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&pboidreason)).into())
        }
        ITransactionVoterBallotAsync2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VoteRequestDone: VoteRequestDone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionVoterFactory2_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, ptransaction: ::core::option::Option<&ITransaction>, pvoternotify: ::core::option::Option<&ITransactionVoterNotifyAsync2>) -> ::windows_core::Result<ITransactionVoterBallotAsync2>;
}
impl ::windows_core::Iids for ITransactionVoterFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionVoterFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionVoterFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionVoterFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, pvoternotify: *mut ::core::ffi::c_void, ppvoterballot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::windows_core::from_raw_borrowed(&ptransaction), ::windows_core::from_raw_borrowed(&pvoternotify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvoterballot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionVoterFactory2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionVoterNotifyAsync2_Impl: ::windows_core::BaseImpl + ITransactionOutcomeEvents_Impl {
    fn VoteRequest(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransactionVoterNotifyAsync2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITransactionOutcomeEvents);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionVoterNotifyAsync2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionVoterNotifyAsync2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VoteRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionVoterNotifyAsync2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VoteRequest(this).into())
        }
        ITransactionVoterNotifyAsync2_Vtbl { base__: <ITransactionOutcomeEvents as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, VoteRequest: VoteRequest::<Identity, Impl, OFFSET> }
    };
}
pub trait IXAConfig_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, clsidhelperdll: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXAConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXAConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXAConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXAConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&clsidhelperdll)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXAConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        IXAConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXAObtainRMInfo_Impl: ::windows_core::BaseImpl {
    fn ObtainRMInfo(this: &Self::This, pirmhelper: ::core::option::Option<&IRMHelper>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXAObtainRMInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXAObtainRMInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXAObtainRMInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ObtainRMInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXAObtainRMInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pirmhelper: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ObtainRMInfo(this, ::windows_core::from_raw_borrowed(&pirmhelper)).into())
        }
        IXAObtainRMInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ObtainRMInfo: ObtainRMInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait IXATransLookup_Impl: ::windows_core::BaseImpl {
    fn Lookup(this: &Self::This) -> ::windows_core::Result<ITransaction>;
}
impl ::windows_core::Iids for IXATransLookup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXATransLookup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXATransLookup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXATransLookup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Lookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXATransLookup_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Lookup: Lookup::<Identity, Impl, OFFSET> }
    };
}
pub trait IXATransLookup2_Impl: ::windows_core::BaseImpl {
    fn Lookup(this: &Self::This, pxid: *const XID) -> ::windows_core::Result<ITransaction>;
}
impl ::windows_core::Iids for IXATransLookup2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXATransLookup2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXATransLookup2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXATransLookup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxid: *const XID, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Lookup(this, ::core::mem::transmute_copy(&pxid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXATransLookup2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Lookup: Lookup::<Identity, Impl, OFFSET> }
    };
}
