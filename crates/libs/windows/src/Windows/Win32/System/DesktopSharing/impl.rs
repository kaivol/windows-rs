#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIApplication_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Windows(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIWindowList>;
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Shared(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShared(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Windows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwindowlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Windows(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindowlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shared<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shared(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetShared<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetShared(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIApplication_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Windows: Windows::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Shared: Shared::<Identity, Impl, OFFSET>,
            SetShared: SetShared::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIApplicationFilter_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Applications(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIApplicationList>;
    fn Windows(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIWindowList>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIApplicationFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIApplicationFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Applications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplications: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Applications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papplications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Windows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwindows: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Windows(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindows, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&newval)).into())
        }
        IRDPSRAPIApplicationFilter_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Applications: Applications::<Identity, Impl, OFFSET>,
            Windows: Windows::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIApplicationList_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, item: i32) -> ::windows_core::Result<IRDPSRAPIApplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIApplicationList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIApplicationList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, papplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIApplicationList_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIAttendee_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RemoteName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ControlLevel(this: &Self::This) -> ::windows_core::Result<CTRL_LEVEL>;
    fn SetControlLevel(this: &Self::This, pnewval: CTRL_LEVEL) -> ::windows_core::Result<()>;
    fn Invitation(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIInvitation>;
    fn TerminateConnection(this: &Self::This) -> ::windows_core::Result<()>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ConnectivityInfo(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIAttendee {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIAttendee {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ControlLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut CTRL_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetControlLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: CTRL_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControlLevel(this, ::core::mem::transmute_copy(&pnewval)).into())
        }
        unsafe extern "system" fn Invitation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Invitation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TerminateConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TerminateConnection(this).into())
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectivityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectivityInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIAttendee_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            RemoteName: RemoteName::<Identity, Impl, OFFSET>,
            ControlLevel: ControlLevel::<Identity, Impl, OFFSET>,
            SetControlLevel: SetControlLevel::<Identity, Impl, OFFSET>,
            Invitation: Invitation::<Identity, Impl, OFFSET>,
            TerminateConnection: TerminateConnection::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            ConnectivityInfo: ConnectivityInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIAttendeeDisconnectInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Attendee(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIAttendee>;
    fn Reason(this: &Self::This) -> ::windows_core::Result<ATTENDEE_DISCONNECT_REASON>;
    fn Code(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIAttendeeDisconnectInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIAttendeeDisconnectInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Attendee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attendee(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Reason(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preason, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Code<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Code(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Attendee: Attendee::<Identity, Impl, OFFSET>,
            Reason: Reason::<Identity, Impl, OFFSET>,
            Code: Code::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIAttendeeManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, id: i32) -> ::windows_core::Result<IRDPSRAPIAttendee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIAttendeeManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIAttendeeManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIAttendeeManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPIAudioStream_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<i64>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetBuffer(this: &Self::This, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows_core::Result<()>;
    fn FreeBuffer(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRDPSRAPIAudioStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIAudioStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Initialize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnperiodinhundrednsintervals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&ptimestamp)).into())
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBuffer(this).into())
        }
        IRDPSRAPIAudioStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRDPSRAPIClipboardUseEvents_Impl: ::windows_core::BaseImpl {
    fn OnPasteFromClipboard(this: &Self::This, clipboardformat: u32, pattendee: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IRDPSRAPIClipboardUseEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIClipboardUseEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPasteFromClipboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipboardformat: u32, pattendee: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnPasteFromClipboard(this, ::core::mem::transmute_copy(&clipboardformat), ::windows_core::from_raw_borrowed(&pattendee)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIClipboardUseEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPasteFromClipboard: OnPasteFromClipboard::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPIDebug_Impl: ::windows_core::BaseImpl {
    fn SetCLXCmdLine(this: &Self::This, clxcmdline: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CLXCmdLine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IRDPSRAPIDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCLXCmdLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clxcmdline: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLXCmdLine(this, ::core::mem::transmute(&clxcmdline)).into())
        }
        unsafe extern "system" fn CLXCmdLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclxcmdline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CLXCmdLine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclxcmdline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIDebug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCLXCmdLine: SetCLXCmdLine::<Identity, Impl, OFFSET>,
            CLXCmdLine: CLXCmdLine::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIFrameBuffer_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Width(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Height(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Bpp(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetFrameBufferBits(this: &Self::This, x: i32, y: i32, width: i32, heigth: i32) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIFrameBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIFrameBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Width<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Width(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plheight: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Height(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bpp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbpp: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bpp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbpp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFrameBufferBits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameBufferBits(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&heigth)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIFrameBuffer_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Bpp: Bpp::<Identity, Impl, OFFSET>,
            GetFrameBufferBits: GetFrameBufferBits::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIInvitation_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ConnectionString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GroupName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Password(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AttendeeLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAttendeeLimit(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn Revoked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRevoked(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIInvitation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIInvitation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectionString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GroupName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Password<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Password(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AttendeeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AttendeeLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttendeeLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttendeeLimit(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Revoked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Revoked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRevoked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRevoked(this, ::core::mem::transmute_copy(&newval)).into())
        }
        IRDPSRAPIInvitation_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectionString: ConnectionString::<Identity, Impl, OFFSET>,
            GroupName: GroupName::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            AttendeeLimit: AttendeeLimit::<Identity, Impl, OFFSET>,
            SetAttendeeLimit: SetAttendeeLimit::<Identity, Impl, OFFSET>,
            Revoked: Revoked::<Identity, Impl, OFFSET>,
            SetRevoked: SetRevoked::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIInvitationManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, item: &super::Variant::VARIANT) -> ::windows_core::Result<IRDPSRAPIInvitation>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CreateInvitation(this: &Self::This, bstrauthstring: &::windows_core::BSTR, bstrgroupname: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, attendeelimit: i32) -> ::windows_core::Result<IRDPSRAPIInvitation>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIInvitationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIInvitationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: super::Variant::VARIANT, ppinvitation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinvitation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInvitation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrauthstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, attendeelimit: i32, ppinvitation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInvitation(this, ::core::mem::transmute(&bstrauthstring), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&attendeelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinvitation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIInvitationManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            CreateInvitation: CreateInvitation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPIPerfCounterLogger_Impl: ::windows_core::BaseImpl {
    fn LogValue(this: &Self::This, lvalue: i64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRDPSRAPIPerfCounterLogger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIPerfCounterLogger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LogValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvalue: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogValue(this, ::core::mem::transmute_copy(&lvalue)).into())
        }
        IRDPSRAPIPerfCounterLogger_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LogValue: LogValue::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPIPerfCounterLoggingManager_Impl: ::windows_core::BaseImpl {
    fn CreateLogger(this: &Self::This, bstrcountername: &::windows_core::BSTR) -> ::windows_core::Result<IRDPSRAPIPerfCounterLogger>;
}
impl ::windows_core::Iids for IRDPSRAPIPerfCounterLoggingManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIPerfCounterLoggingManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLogger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcountername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pplogger: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLogger(this, ::core::mem::transmute(&bstrcountername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplogger, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIPerfCounterLoggingManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateLogger: CreateLogger::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPISessionProperties_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Property(this: &Self::This, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn put_Property(this: &Self::This, propertyname: &::windows_core::BSTR, newval: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPISessionProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPISessionProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Property<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Property(this, ::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Property<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, newval: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Property(this, ::core::mem::transmute(&propertyname), ::core::mem::transmute(&newval)).into())
        }
        IRDPSRAPISessionProperties_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Property: get_Property::<Identity, Impl, OFFSET>,
            put_Property: put_Property::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPISharingSession_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Open(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetColorDepth(this: &Self::This, colordepth: i32) -> ::windows_core::Result<()>;
    fn ColorDepth(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<IRDPSRAPISessionProperties>;
    fn Attendees(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIVirtualChannelManager>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn ConnectToClient(this: &Self::This, bstrconnectionstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDesktopSharedRect(this: &Self::This, left: i32, top: i32, right: i32, bottom: i32) -> ::windows_core::Result<()>;
    fn GetDesktopSharedRect(this: &Self::This, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPISharingSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPISharingSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn SetColorDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colordepth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorDepth(this, ::core::mem::transmute_copy(&colordepth)).into())
        }
        unsafe extern "system" fn ColorDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolordepth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColorDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcolordepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attendees<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attendees(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Invitations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Invitations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplicationFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VirtualChannelManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn ConnectToClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectToClient(this, ::core::mem::transmute(&bstrconnectionstring)).into())
        }
        unsafe extern "system" fn SetDesktopSharedRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesktopSharedRect(this, ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into())
        }
        unsafe extern "system" fn GetDesktopSharedRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesktopSharedRect(this, ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into())
        }
        IRDPSRAPISharingSession_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetColorDepth: SetColorDepth::<Identity, Impl, OFFSET>,
            ColorDepth: ColorDepth::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Attendees: Attendees::<Identity, Impl, OFFSET>,
            Invitations: Invitations::<Identity, Impl, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, Impl, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            ConnectToClient: ConnectToClient::<Identity, Impl, OFFSET>,
            SetDesktopSharedRect: SetDesktopSharedRect::<Identity, Impl, OFFSET>,
            GetDesktopSharedRect: GetDesktopSharedRect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPISharingSession2_Impl: ::windows_core::BaseImpl + IRDPSRAPISharingSession_Impl {
    fn ConnectUsingTransportStream(this: &Self::This, pstream: ::core::option::Option<&IRDPSRAPITransportStream>, bstrgroup: &::windows_core::BSTR, bstrauthenticatedattendeename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FrameBuffer(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIFrameBuffer>;
    fn SendControlLevelChangeResponse(this: &Self::This, pattendee: ::core::option::Option<&IRDPSRAPIAttendee>, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPISharingSession2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRDPSRAPISharingSession);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPISharingSession2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectUsingTransportStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, bstrgroup: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrauthenticatedattendeename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectUsingTransportStream(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute(&bstrgroup), ::core::mem::transmute(&bstrauthenticatedattendeename)).into())
        }
        unsafe extern "system" fn FrameBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FrameBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattendee: *mut ::core::ffi::c_void, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendControlLevelChangeResponse(this, ::windows_core::from_raw_borrowed(&pattendee), ::core::mem::transmute_copy(&requestedlevel), ::core::mem::transmute_copy(&reasoncode)).into())
        }
        IRDPSRAPISharingSession2_Vtbl {
            base__: <IRDPSRAPISharingSession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectUsingTransportStream: ConnectUsingTransportStream::<Identity, Impl, OFFSET>,
            FrameBuffer: FrameBuffer::<Identity, Impl, OFFSET>,
            SendControlLevelChangeResponse: SendControlLevelChangeResponse::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPITcpConnectionInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Protocol(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LocalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LocalIP(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PeerPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PeerIP(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPITcpConnectionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPITcpConnectionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprotocol: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprotocol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalIP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsrlocalip: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalIP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsrlocalip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeerPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeerPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeerIP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrip: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeerIP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPITcpConnectionInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            LocalPort: LocalPort::<Identity, Impl, OFFSET>,
            LocalIP: LocalIP::<Identity, Impl, OFFSET>,
            PeerPort: PeerPort::<Identity, Impl, OFFSET>,
            PeerIP: PeerIP::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPITransportStream_Impl: ::windows_core::BaseImpl {
    fn AllocBuffer(this: &Self::This, maxpayload: i32) -> ::windows_core::Result<IRDPSRAPITransportStreamBuffer>;
    fn FreeBuffer(this: &Self::This, pbuffer: ::core::option::Option<&IRDPSRAPITransportStreamBuffer>) -> ::windows_core::Result<()>;
    fn WriteBuffer(this: &Self::This, pbuffer: ::core::option::Option<&IRDPSRAPITransportStreamBuffer>) -> ::windows_core::Result<()>;
    fn ReadBuffer(this: &Self::This, pbuffer: ::core::option::Option<&IRDPSRAPITransportStreamBuffer>) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, pcallbacks: ::core::option::Option<&IRDPSRAPITransportStreamEvents>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRDPSRAPITransportStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPITransportStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllocBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxpayload: i32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocBuffer(this, ::core::mem::transmute_copy(&maxpayload)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBuffer(this, ::windows_core::from_raw_borrowed(&pbuffer)).into())
        }
        unsafe extern "system" fn WriteBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteBuffer(this, ::windows_core::from_raw_borrowed(&pbuffer)).into())
        }
        unsafe extern "system" fn ReadBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadBuffer(this, ::windows_core::from_raw_borrowed(&pbuffer)).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallbacks: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&pcallbacks)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IRDPSRAPITransportStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllocBuffer: AllocBuffer::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, Impl, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPITransportStreamBuffer_Impl: ::windows_core::BaseImpl {
    fn Storage(this: &Self::This) -> ::windows_core::Result<*mut u8>;
    fn StorageSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PayloadSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPayloadSize(this: &Self::This, lval: i32) -> ::windows_core::Result<()>;
    fn PayloadOffset(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPayloadOffset(this: &Self::This, lretval: i32) -> ::windows_core::Result<()>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFlags(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn Context(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetContext(this: &Self::This, pcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRDPSRAPITransportStreamBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPITransportStreamBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Storage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbstorage: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Storage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StorageSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxstore: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StorageSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PayloadSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PayloadSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPayloadSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPayloadSize(this, ::core::mem::transmute_copy(&lval)).into())
        }
        unsafe extern "system" fn PayloadOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PayloadOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPayloadOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lretval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPayloadOffset(this, ::core::mem::transmute_copy(&lretval)).into())
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn Context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Context(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContext(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        IRDPSRAPITransportStreamBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Storage: Storage::<Identity, Impl, OFFSET>,
            StorageSize: StorageSize::<Identity, Impl, OFFSET>,
            PayloadSize: PayloadSize::<Identity, Impl, OFFSET>,
            SetPayloadSize: SetPayloadSize::<Identity, Impl, OFFSET>,
            PayloadOffset: PayloadOffset::<Identity, Impl, OFFSET>,
            SetPayloadOffset: SetPayloadOffset::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRDPSRAPITransportStreamEvents_Impl: ::windows_core::BaseImpl {
    fn OnWriteCompleted(this: &Self::This, pbuffer: ::core::option::Option<&IRDPSRAPITransportStreamBuffer>);
    fn OnReadCompleted(this: &Self::This, pbuffer: ::core::option::Option<&IRDPSRAPITransportStreamBuffer>);
    fn OnStreamClosed(this: &Self::This, hrreason: ::windows_core::HRESULT);
}
impl ::windows_core::Iids for IRDPSRAPITransportStreamEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPITransportStreamEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnWriteCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWriteCompleted(this, ::windows_core::from_raw_borrowed(&pbuffer)))
        }
        unsafe extern "system" fn OnReadCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReadCompleted(this, ::windows_core::from_raw_borrowed(&pbuffer)))
        }
        unsafe extern "system" fn OnStreamClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrreason: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStreamClosed(this, ::core::mem::transmute_copy(&hrreason)))
        }
        IRDPSRAPITransportStreamEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnWriteCompleted: OnWriteCompleted::<Identity, Impl, OFFSET>,
            OnReadCompleted: OnReadCompleted::<Identity, Impl, OFFSET>,
            OnStreamClosed: OnStreamClosed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIViewer_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Connect(this: &Self::This, bstrconnectionstring: &::windows_core::BSTR, bstrname: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Attendees(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIVirtualChannelManager>;
    fn SetSmartSizing(this: &Self::This, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SmartSizing(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RequestControl(this: &Self::This, ctrllevel: CTRL_LEVEL) -> ::windows_core::Result<()>;
    fn SetDisconnectedText(this: &Self::This, bstrdisconnectedtext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisconnectedText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestColorDepthChange(this: &Self::This, bpp: i32) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<IRDPSRAPISessionProperties>;
    fn StartReverseConnectListener(this: &Self::This, bstrconnectionstring: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIViewer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIViewer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute(&bstrconnectionstring), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn Attendees<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attendees(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Invitations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Invitations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplicationFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VirtualChannelManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSmartSizing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSmartSizing(this, ::core::mem::transmute_copy(&vbsmartsizing)).into())
        }
        unsafe extern "system" fn SmartSizing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbsmartsizing: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmartSizing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbsmartsizing, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestControl(this, ::core::mem::transmute_copy(&ctrllevel)).into())
        }
        unsafe extern "system" fn SetDisconnectedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdisconnectedtext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisconnectedText(this, ::core::mem::transmute(&bstrdisconnectedtext)).into())
        }
        unsafe extern "system" fn DisconnectedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisconnectedtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisconnectedText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisconnectedtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestColorDepthChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpp: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestColorDepthChange(this, ::core::mem::transmute_copy(&bpp)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartReverseConnectListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIViewer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrreverseconnectstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartReverseConnectListener(this, ::core::mem::transmute(&bstrconnectionstring), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreverseconnectstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIViewer_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Attendees: Attendees::<Identity, Impl, OFFSET>,
            Invitations: Invitations::<Identity, Impl, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, Impl, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, Impl, OFFSET>,
            SetSmartSizing: SetSmartSizing::<Identity, Impl, OFFSET>,
            SmartSizing: SmartSizing::<Identity, Impl, OFFSET>,
            RequestControl: RequestControl::<Identity, Impl, OFFSET>,
            SetDisconnectedText: SetDisconnectedText::<Identity, Impl, OFFSET>,
            DisconnectedText: DisconnectedText::<Identity, Impl, OFFSET>,
            RequestColorDepthChange: RequestColorDepthChange::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            StartReverseConnectListener: StartReverseConnectListener::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIVirtualChannel_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SendData(this: &Self::This, bstrdata: &::windows_core::BSTR, lattendeeid: i32, channelsendflags: u32) -> ::windows_core::Result<()>;
    fn SetAccess(this: &Self::This, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<CHANNEL_PRIORITY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIVirtualChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIVirtualChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendData(this, ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&channelsendflags)).into())
        }
        unsafe extern "system" fn SetAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccess(this, ::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&accesstype)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIVirtualChannel_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendData: SendData::<Identity, Impl, OFFSET>,
            SetAccess: SetAccess::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIVirtualChannelManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, item: &super::Variant::VARIANT) -> ::windows_core::Result<IRDPSRAPIVirtualChannel>;
    fn CreateVirtualChannel(this: &Self::This, bstrchannelname: &::windows_core::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> ::windows_core::Result<IRDPSRAPIVirtualChannel>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIVirtualChannelManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIVirtualChannelManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: super::Variant::VARIANT, pchannel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchannel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrchannelname: ::std::mem::MaybeUninit<::windows_core::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVirtualChannel(this, ::core::mem::transmute(&bstrchannelname), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&channelflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchannel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIVirtualChannelManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIWindow_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Application(this: &Self::This) -> ::windows_core::Result<IRDPSRAPIApplication>;
    fn Shared(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShared(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Show(this: &Self::This) -> ::windows_core::Result<()>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIWindow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIWindow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Application<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Application(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shared<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shared(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetShared<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetShared(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this).into())
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIWindow_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Application: Application::<Identity, Impl, OFFSET>,
            Shared: Shared::<Identity, Impl, OFFSET>,
            SetShared: SetShared::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIWindowList_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, item: i32) -> ::windows_core::Result<IRDPSRAPIWindow>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRDPSRAPIWindowList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindowList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPSRAPIWindowList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindowList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPSRAPIWindowList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, pwindow: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRDPSRAPIWindowList_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRDPViewerInputSink_Impl: ::windows_core::BaseImpl {
    fn SendMouseButtonEvent(this: &Self::This, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> ::windows_core::Result<()>;
    fn SendMouseMoveEvent(this: &Self::This, xpos: u32, ypos: u32) -> ::windows_core::Result<()>;
    fn SendMouseWheelEvent(this: &Self::This, wheelrotation: u16) -> ::windows_core::Result<()>;
    fn SendKeyboardEvent(this: &Self::This, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SendSyncEvent(this: &Self::This, syncflags: u32) -> ::windows_core::Result<()>;
    fn BeginTouchFrame(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddTouchInput(this: &Self::This, contactid: u32, event: u32, x: i32, y: i32) -> ::windows_core::Result<()>;
    fn EndTouchFrame(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRDPViewerInputSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRDPViewerInputSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendMouseButtonEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMouseButtonEvent(this, ::core::mem::transmute_copy(&buttontype), ::core::mem::transmute_copy(&vbbuttondown), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into())
        }
        unsafe extern "system" fn SendMouseMoveEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpos: u32, ypos: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMouseMoveEvent(this, ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into())
        }
        unsafe extern "system" fn SendMouseWheelEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wheelrotation: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMouseWheelEvent(this, ::core::mem::transmute_copy(&wheelrotation)).into())
        }
        unsafe extern "system" fn SendKeyboardEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendKeyboardEvent(this, ::core::mem::transmute_copy(&codetype), ::core::mem::transmute_copy(&keycode), ::core::mem::transmute_copy(&vbkeyup), ::core::mem::transmute_copy(&vbrepeat), ::core::mem::transmute_copy(&vbextended)).into())
        }
        unsafe extern "system" fn SendSyncEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendSyncEvent(this, ::core::mem::transmute_copy(&syncflags)).into())
        }
        unsafe extern "system" fn BeginTouchFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginTouchFrame(this).into())
        }
        unsafe extern "system" fn AddTouchInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTouchInput(this, ::core::mem::transmute_copy(&contactid), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn EndTouchFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRDPViewerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndTouchFrame(this).into())
        }
        IRDPViewerInputSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendMouseButtonEvent: SendMouseButtonEvent::<Identity, Impl, OFFSET>,
            SendMouseMoveEvent: SendMouseMoveEvent::<Identity, Impl, OFFSET>,
            SendMouseWheelEvent: SendMouseWheelEvent::<Identity, Impl, OFFSET>,
            SendKeyboardEvent: SendKeyboardEvent::<Identity, Impl, OFFSET>,
            SendSyncEvent: SendSyncEvent::<Identity, Impl, OFFSET>,
            BeginTouchFrame: BeginTouchFrame::<Identity, Impl, OFFSET>,
            AddTouchInput: AddTouchInput::<Identity, Impl, OFFSET>,
            EndTouchFrame: EndTouchFrame::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _IRDPSessionEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _IRDPSessionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _IRDPSessionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _IRDPSessionEvents {
    const VTABLE: Self::Vtable = { _IRDPSessionEvents_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
