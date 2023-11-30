#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICloneViewHelper_Impl: ::windows_core::BaseImpl {
    fn GetConnectedIDs(this: &Self::This, wszadaptorname: &::windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveTopology(this: &Self::This, wszadaptorname: &::windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows_core::Result<()>;
    fn SetActiveTopology(this: &Self::This, wszadaptorname: &::windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, ffinalcall: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICloneViewHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICloneViewHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICloneViewHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectedIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICloneViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszadaptorname: ::windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectedIDs(this, ::core::mem::transmute(&wszadaptorname), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pulid), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn GetActiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICloneViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszadaptorname: ::windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActiveTopology(this, ::core::mem::transmute(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pultargetid)).into())
        }
        unsafe extern "system" fn SetActiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICloneViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszadaptorname: ::windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveTopology(this, ::core::mem::transmute(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pultargetid)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICloneViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffinalcall: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&ffinalcall)).into())
        }
        ICloneViewHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectedIDs: GetConnectedIDs::<Identity, Impl, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, Impl, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IViewHelper_Impl: ::windows_core::BaseImpl {
    fn GetConnectedIDs(this: &Self::This, wszadaptorname: &::windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveTopology(this: &Self::This, wszadaptorname: &::windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows_core::Result<()>;
    fn SetActiveTopology(this: &Self::This, wszadaptorname: &::windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetConfiguration(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<u32>;
    fn GetProceedOnNewConfiguration(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IViewHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectedIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszadaptorname: ::windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectedIDs(this, ::core::mem::transmute(&wszadaptorname), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pulid), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn GetActiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszadaptorname: ::windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActiveTopology(this, ::core::mem::transmute(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pultargetid)).into())
        }
        unsafe extern "system" fn SetActiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszadaptorname: ::windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveTopology(this, ::core::mem::transmute(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pultargetid)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn SetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetConfiguration(this, ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProceedOnNewConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProceedOnNewConfiguration(this).into())
        }
        IViewHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectedIDs: GetConnectedIDs::<Identity, Impl, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, Impl, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            GetProceedOnNewConfiguration: GetProceedOnNewConfiguration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
