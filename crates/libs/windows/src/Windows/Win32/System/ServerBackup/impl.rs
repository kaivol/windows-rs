pub trait IWsbApplicationAsync_Impl: ::windows_core::BaseImpl {
    fn QueryStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWsbApplicationAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWsbApplicationAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        IWsbApplicationAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWsbApplicationBackupSupport_Impl: ::windows_core::BaseImpl {
    fn CheckConsistency(this: &Self::This, wszwritermetadata: &::windows_core::PCWSTR, wszcomponentname: &::windows_core::PCWSTR, wszcomponentlogicalpath: &::windows_core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const ::windows_core::PCWSTR, rgwszsnapshotvolumepath: *const ::windows_core::PCWSTR) -> ::windows_core::Result<IWsbApplicationAsync>;
}
impl ::windows_core::Iids for IWsbApplicationBackupSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationBackupSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWsbApplicationBackupSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckConsistency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationBackupSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcomponentlogicalpath: ::windows_core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const ::windows_core::PCWSTR, rgwszsnapshotvolumepath: *const ::windows_core::PCWSTR, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckConsistency(this, ::core::mem::transmute(&wszwritermetadata), ::core::mem::transmute(&wszcomponentname), ::core::mem::transmute(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&cvolumes), ::core::mem::transmute_copy(&rgwszsourcevolumepath), ::core::mem::transmute_copy(&rgwszsnapshotvolumepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWsbApplicationBackupSupport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CheckConsistency: CheckConsistency::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWsbApplicationRestoreSupport_Impl: ::windows_core::BaseImpl {
    fn PreRestore(this: &Self::This, wszwritermetadata: &::windows_core::PCWSTR, wszcomponentname: &::windows_core::PCWSTR, wszcomponentlogicalpath: &::windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows_core::Result<()>;
    fn PostRestore(this: &Self::This, wszwritermetadata: &::windows_core::PCWSTR, wszcomponentname: &::windows_core::PCWSTR, wszcomponentlogicalpath: &::windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows_core::Result<()>;
    fn OrderComponents(this: &Self::This, ccomponents: u32, rgcomponentname: *const ::windows_core::PCWSTR, rgcomponentlogicalpaths: *const ::windows_core::PCWSTR, prgcomponentname: *mut *mut ::windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn IsRollForwardSupported(this: &Self::This) -> ::windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWsbApplicationRestoreSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWsbApplicationRestoreSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PreRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcomponentlogicalpath: ::windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreRestore(this, ::core::mem::transmute(&wszwritermetadata), ::core::mem::transmute(&wszcomponentname), ::core::mem::transmute(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into())
        }
        unsafe extern "system" fn PostRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcomponentlogicalpath: ::windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostRestore(this, ::core::mem::transmute(&wszwritermetadata), ::core::mem::transmute(&wszcomponentname), ::core::mem::transmute(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into())
        }
        unsafe extern "system" fn OrderComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccomponents: u32, rgcomponentname: *const ::windows_core::PCWSTR, rgcomponentlogicalpaths: *const ::windows_core::PCWSTR, prgcomponentname: *mut *mut ::windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OrderComponents(this, ::core::mem::transmute_copy(&ccomponents), ::core::mem::transmute_copy(&rgcomponentname), ::core::mem::transmute_copy(&rgcomponentlogicalpaths), ::core::mem::transmute_copy(&prgcomponentname), ::core::mem::transmute_copy(&prgcomponentlogicalpath)).into())
        }
        unsafe extern "system" fn IsRollForwardSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrollforwardsupported: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRollForwardSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbrollforwardsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWsbApplicationRestoreSupport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PreRestore: PreRestore::<Identity, Impl, OFFSET>,
            PostRestore: PostRestore::<Identity, Impl, OFFSET>,
            OrderComponents: OrderComponents::<Identity, Impl, OFFSET>,
            IsRollForwardSupported: IsRollForwardSupported::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
