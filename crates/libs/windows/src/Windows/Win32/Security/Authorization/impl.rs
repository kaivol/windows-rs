#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplication_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(this: &Self::This, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AuthzInterfaceClsid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAuthzInterfaceClsid(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVersion(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GenerateAudits(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(this: &Self::This, bprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ApplyStoreSacl(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(this: &Self::This, bprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PolicyAdministrators(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReaders(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministrator(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministrator(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReader(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReader(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Scopes(this: &Self::This) -> ::windows_core::Result<IAzScopes>;
    fn OpenScope(this: &Self::This, bstrscopename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzScope>;
    fn CreateScope(this: &Self::This, bstrscopename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzScope>;
    fn DeleteScope(this: &Self::This, bstrscopename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Operations(this: &Self::This) -> ::windows_core::Result<IAzOperations>;
    fn OpenOperation(this: &Self::This, bstroperationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzOperation>;
    fn CreateOperation(this: &Self::This, bstroperationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzOperation>;
    fn DeleteOperation(this: &Self::This, bstroperationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Tasks(this: &Self::This) -> ::windows_core::Result<IAzTasks>;
    fn OpenTask(this: &Self::This, bstrtaskname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn CreateTask(this: &Self::This, bstrtaskname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn DeleteTask(this: &Self::This, bstrtaskname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ApplicationGroups(this: &Self::This) -> ::windows_core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Roles(this: &Self::This) -> ::windows_core::Result<IAzRoles>;
    fn OpenRole(this: &Self::This, bstrrolename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn CreateRole(this: &Self::This, bstrrolename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn DeleteRole(this: &Self::This, bstrrolename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn InitializeClientContextFromToken(this: &Self::This, ulltokenhandle: u64, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzClientContext>;
    fn AddPropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn InitializeClientContextFromName(this: &Self::This, clientname: &::windows_core::BSTR, domainname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzClientContext>;
    fn DelegatedPolicyUsers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUser(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUser(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn InitializeClientContextFromStringSid(this: &Self::This, sidstring: &::windows_core::BSTR, loptions: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzClientContext>;
    fn PolicyAdministratorsName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReadersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministratorName(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministratorName(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReaderName(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReaderName(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DelegatedPolicyUsersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUserName(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUserName(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationData(this, ::core::mem::transmute(&bstrapplicationdata)).into())
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthzInterfaceClsid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthzInterfaceClsid(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateAudits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGenerateAudits(this, ::core::mem::transmute_copy(&bprop)).into())
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplyStoreSacl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplyStoreSacl(this, ::core::mem::transmute_copy(&bprop)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyAdministrators(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyReaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyAdministrator(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyAdministrator(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyReader(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyReader(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Scopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscopecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scopes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscopecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenScope(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScope(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteScope(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Operations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperationcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenOperation(this, ::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateOperation(this, ::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteOperation(this, ::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tasks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenTask(this, ::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTask(this, ::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTask(this, ::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Roles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Roles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprolecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenRole(this, ::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRole(this, ::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRole(this, ::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitializeClientContextFromToken(this, ::core::mem::transmute_copy(&ulltokenhandle), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn InitializeClientContextFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientname: ::std::mem::MaybeUninit<::windows_core::BSTR>, domainname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitializeClientContextFromName(this, ::core::mem::transmute(&clientname), ::core::mem::transmute(&domainname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DelegatedPolicyUsers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDelegatedPolicyUser(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDelegatedPolicyUser(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sidstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: i32, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitializeClientContextFromStringSid(this, ::core::mem::transmute(&sidstring), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyAdministratorsName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyReadersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyAdministratorName(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyAdministratorName(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyReaderName(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyReaderName(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DelegatedPolicyUsersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDelegatedPolicyUserName(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDelegatedPolicyUserName(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        IAzApplication_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            AuthzInterfaceClsid: AuthzInterfaceClsid::<Identity, Impl, OFFSET>,
            SetAuthzInterfaceClsid: SetAuthzInterfaceClsid::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, Impl, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, Impl, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, Impl, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            Scopes: Scopes::<Identity, Impl, OFFSET>,
            OpenScope: OpenScope::<Identity, Impl, OFFSET>,
            CreateScope: CreateScope::<Identity, Impl, OFFSET>,
            DeleteScope: DeleteScope::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            OpenOperation: OpenOperation::<Identity, Impl, OFFSET>,
            CreateOperation: CreateOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            OpenTask: OpenTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Roles: Roles::<Identity, Impl, OFFSET>,
            OpenRole: OpenRole::<Identity, Impl, OFFSET>,
            CreateRole: CreateRole::<Identity, Impl, OFFSET>,
            DeleteRole: DeleteRole::<Identity, Impl, OFFSET>,
            InitializeClientContextFromToken: InitializeClientContextFromToken::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            InitializeClientContextFromName: InitializeClientContextFromName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            InitializeClientContextFromStringSid: InitializeClientContextFromStringSid::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplication2_Impl: ::windows_core::BaseImpl + IAzApplication_Impl {
    fn InitializeClientContextFromToken2(this: &Self::This, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzClientContext2>;
    fn InitializeClientContext2(this: &Self::This, identifyingstring: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzClientContext2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplication2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzApplication);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplication2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeClientContextFromToken2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitializeClientContextFromToken2(this, ::core::mem::transmute_copy(&ultokenhandlelowpart), ::core::mem::transmute_copy(&ultokenhandlehighpart), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeClientContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifyingstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitializeClientContext2(this, ::core::mem::transmute(&identifyingstring), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzApplication2_Vtbl {
            base__: <IAzApplication as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeClientContextFromToken2: InitializeClientContextFromToken2::<Identity, Impl, OFFSET>,
            InitializeClientContext2: InitializeClientContext2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplication3_Impl: ::windows_core::BaseImpl + IAzApplication2_Impl {
    fn ScopeExists(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn OpenScope2(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzScope2>;
    fn CreateScope2(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzScope2>;
    fn DeleteScope2(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleDefinitions(this: &Self::This) -> ::windows_core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(this: &Self::This, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(this: &Self::This, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(this: &Self::This, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleAssignments(this: &Self::This) -> ::windows_core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(this: &Self::This, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(this: &Self::This, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(this: &Self::This, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRulesEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBizRulesEnabled(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplication3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzApplication2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplication3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScopeExists<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScopeExists(this, ::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbexist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenScope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenScope2(this, ::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateScope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScope2(this, ::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteScope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteScope2(this, ::core::mem::transmute(&bstrscopename)).into())
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleDefinitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRoleDefinition(this, ::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenRoleDefinition(this, ::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRoleDefinition(this, ::core::mem::transmute(&bstrroledefinitionname)).into())
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleAssignments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRoleAssignment(this, ::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenRoleAssignment(this, ::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRoleAssignment(this, ::core::mem::transmute(&bstrroleassignmentname)).into())
        }
        unsafe extern "system" fn BizRulesEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRulesEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRulesEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRulesEnabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        IAzApplication3_Vtbl {
            base__: <IAzApplication2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ScopeExists: ScopeExists::<Identity, Impl, OFFSET>,
            OpenScope2: OpenScope2::<Identity, Impl, OFFSET>,
            CreateScope2: CreateScope2::<Identity, Impl, OFFSET>,
            DeleteScope2: DeleteScope2::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, Impl, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, Impl, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, Impl, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, Impl, OFFSET>,
            BizRulesEnabled: BizRulesEnabled::<Identity, Impl, OFFSET>,
            SetBizRulesEnabled: SetBizRulesEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplicationGroup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetType(this: &Self::This, lprop: i32) -> ::windows_core::Result<()>;
    fn LdapQuery(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLdapQuery(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AppMembers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AppNonMembers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Members(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NonMembers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddAppMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteAppMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddAppNonMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteAppNonMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddNonMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteNonMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddMemberName(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMemberName(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddNonMemberName(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteNonMemberName(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MembersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NonMembersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplicationGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplicationGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&lprop)).into())
        }
        unsafe extern "system" fn LdapQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LdapQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLdapQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLdapQuery(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppMembers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppNonMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppNonMembers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Members(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NonMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NonMembers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAppMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAppMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddAppNonMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAppNonMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteAppNonMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAppNonMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddNonMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNonMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteNonMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteNonMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMemberName(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMemberName(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddNonMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNonMemberName(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteNonMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteNonMemberName(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn MembersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MembersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NonMembersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NonMembersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzApplicationGroup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            LdapQuery: LdapQuery::<Identity, Impl, OFFSET>,
            SetLdapQuery: SetLdapQuery::<Identity, Impl, OFFSET>,
            AppMembers: AppMembers::<Identity, Impl, OFFSET>,
            AppNonMembers: AppNonMembers::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            NonMembers: NonMembers::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            AddAppMember: AddAppMember::<Identity, Impl, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, Impl, OFFSET>,
            AddAppNonMember: AddAppNonMember::<Identity, Impl, OFFSET>,
            DeleteAppNonMember: DeleteAppNonMember::<Identity, Impl, OFFSET>,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            AddNonMember: AddNonMember::<Identity, Impl, OFFSET>,
            DeleteNonMember: DeleteNonMember::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            AddMemberName: AddMemberName::<Identity, Impl, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, Impl, OFFSET>,
            AddNonMemberName: AddNonMemberName::<Identity, Impl, OFFSET>,
            DeleteNonMemberName: DeleteNonMemberName::<Identity, Impl, OFFSET>,
            MembersName: MembersName::<Identity, Impl, OFFSET>,
            NonMembersName: NonMembersName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplicationGroup2_Impl: ::windows_core::BaseImpl + IAzApplicationGroup_Impl {
    fn BizRule(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRule(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleLanguage(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleImportedPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleImportedPath(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleAssignments(this: &Self::This, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplicationGroup2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzApplicationGroup);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplicationGroup2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BizRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRule(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRuleLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRuleLanguage(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRuleImportedPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRuleImportedPath(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleAssignments(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzApplicationGroup2_Vtbl {
            base__: <IAzApplicationGroup as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BizRule: BizRule::<Identity, Impl, OFFSET>,
            SetBizRule: SetBizRule::<Identity, Impl, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, Impl, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, Impl, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, Impl, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplicationGroups_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplicationGroups {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplicationGroups {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzApplicationGroups_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplications_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzApplications {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzApplications {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzApplications_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzAuthorizationStore_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(this: &Self::This, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DomainTimeout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDomainTimeout(this: &Self::This, lprop: i32) -> ::windows_core::Result<()>;
    fn ScriptEngineTimeout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetScriptEngineTimeout(this: &Self::This, lprop: i32) -> ::windows_core::Result<()>;
    fn MaxScriptEngines(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxScriptEngines(this: &Self::This, lprop: i32) -> ::windows_core::Result<()>;
    fn GenerateAudits(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(this: &Self::This, bprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(this: &Self::This, lpropid: AZ_PROP_CONSTANTS, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PolicyAdministrators(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReaders(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministrator(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministrator(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReader(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReader(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn UpdateCache(this: &Self::This, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Applications(this: &Self::This) -> ::windows_core::Result<IAzApplications>;
    fn OpenApplication(this: &Self::This, bstrapplicationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplication>;
    fn CreateApplication(this: &Self::This, bstrapplicationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplication>;
    fn DeleteApplication(this: &Self::This, bstrapplicationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ApplicationGroups(this: &Self::This) -> ::windows_core::Result<IAzApplicationGroups>;
    fn CreateApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn OpenApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DelegatedPolicyUsers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUser(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUser(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TargetMachine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ApplyStoreSacl(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(this: &Self::This, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PolicyAdministratorsName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReadersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministratorName(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministratorName(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReaderName(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReaderName(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DelegatedPolicyUsersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUserName(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUserName(this: &Self::This, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CloseApplication(this: &Self::This, bstrapplicationname: &::windows_core::BSTR, lflag: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzAuthorizationStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzAuthorizationStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationData(this, ::core::mem::transmute(&bstrapplicationdata)).into())
        }
        unsafe extern "system" fn DomainTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDomainTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDomainTimeout(this, ::core::mem::transmute_copy(&lprop)).into())
        }
        unsafe extern "system" fn ScriptEngineTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScriptEngineTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScriptEngineTimeout(this, ::core::mem::transmute_copy(&lprop)).into())
        }
        unsafe extern "system" fn MaxScriptEngines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxScriptEngines(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxScriptEngines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxScriptEngines(this, ::core::mem::transmute_copy(&lprop)).into())
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateAudits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGenerateAudits(this, ::core::mem::transmute_copy(&bprop)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyAdministrators(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyReaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyAdministrator(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyAdministrator(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyReader(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyReader(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrpolicyurl), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn UpdateCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateCache(this, ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Applications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Applications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenApplication(this, ::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplication(this, ::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteApplication(this, ::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DelegatedPolicyUsers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDelegatedPolicyUser(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDelegatedPolicyUser(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn TargetMachine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetMachine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtargetmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplyStoreSacl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbapplystoresacl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplyStoreSacl(this, ::core::mem::transmute_copy(&bapplystoresacl)).into())
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyAdministratorsName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyReadersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyAdministratorName(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyAdministratorName(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyReaderName(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyReaderName(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DelegatedPolicyUsersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDelegatedPolicyUserName(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDelegatedPolicyUserName(this, ::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn CloseApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflag: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseApplication(this, ::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute_copy(&lflag)).into())
        }
        IAzAuthorizationStore_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            DomainTimeout: DomainTimeout::<Identity, Impl, OFFSET>,
            SetDomainTimeout: SetDomainTimeout::<Identity, Impl, OFFSET>,
            ScriptEngineTimeout: ScriptEngineTimeout::<Identity, Impl, OFFSET>,
            SetScriptEngineTimeout: SetScriptEngineTimeout::<Identity, Impl, OFFSET>,
            MaxScriptEngines: MaxScriptEngines::<Identity, Impl, OFFSET>,
            SetMaxScriptEngines: SetMaxScriptEngines::<Identity, Impl, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, Impl, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            UpdateCache: UpdateCache::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Applications: Applications::<Identity, Impl, OFFSET>,
            OpenApplication: OpenApplication::<Identity, Impl, OFFSET>,
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            DeleteApplication: DeleteApplication::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            TargetMachine: TargetMachine::<Identity, Impl, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, Impl, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            CloseApplication: CloseApplication::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzAuthorizationStore2_Impl: ::windows_core::BaseImpl + IAzAuthorizationStore_Impl {
    fn OpenApplication2(this: &Self::This, bstrapplicationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplication2>;
    fn CreateApplication2(this: &Self::This, bstrapplicationname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplication2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzAuthorizationStore2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzAuthorizationStore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzAuthorizationStore2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenApplication2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenApplication2(this, ::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateApplication2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplication2(this, ::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzAuthorizationStore2_Vtbl {
            base__: <IAzAuthorizationStore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenApplication2: OpenApplication2::<Identity, Impl, OFFSET>,
            CreateApplication2: CreateApplication2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzAuthorizationStore3_Impl: ::windows_core::BaseImpl + IAzAuthorizationStore2_Impl {
    fn IsUpdateNeeded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BizruleGroupSupported(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UpgradeStoresFunctionalLevel(this: &Self::This, lfunctionallevel: i32) -> ::windows_core::Result<()>;
    fn IsFunctionalLevelUpgradeSupported(this: &Self::This, lfunctionallevel: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSchemaVersion(this: &Self::This, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzAuthorizationStore3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzAuthorizationStore2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzAuthorizationStore3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsUpdateNeeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUpdateNeeded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisupdateneeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BizruleGroupSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizruleGroupSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpgradeStoresFunctionalLevel(this, ::core::mem::transmute_copy(&lfunctionallevel)).into())
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFunctionalLevelUpgradeSupported(this, ::core::mem::transmute_copy(&lfunctionallevel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSchemaVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSchemaVersion(this, ::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into())
        }
        IAzAuthorizationStore3_Vtbl {
            base__: <IAzAuthorizationStore2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsUpdateNeeded: IsUpdateNeeded::<Identity, Impl, OFFSET>,
            BizruleGroupSupported: BizruleGroupSupported::<Identity, Impl, OFFSET>,
            UpgradeStoresFunctionalLevel: UpgradeStoresFunctionalLevel::<Identity, Impl, OFFSET>,
            IsFunctionalLevelUpgradeSupported: IsFunctionalLevelUpgradeSupported::<Identity, Impl, OFFSET>,
            GetSchemaVersion: GetSchemaVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzBizRuleContext_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetBusinessRuleResult(this: &Self::This, bresult: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBusinessRuleString(this: &Self::This, bstrbusinessrulestring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BusinessRuleString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetParameter(this: &Self::This, bstrparametername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzBizRuleContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzBizRuleContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBusinessRuleResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBusinessRuleResult(this, ::core::mem::transmute_copy(&bresult)).into())
        }
        unsafe extern "system" fn SetBusinessRuleString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBusinessRuleString(this, ::core::mem::transmute(&bstrbusinessrulestring)).into())
        }
        unsafe extern "system" fn BusinessRuleString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BusinessRuleString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbusinessrulestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarparametervalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameter(this, ::core::mem::transmute(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarparametervalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzBizRuleContext_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBusinessRuleResult: SetBusinessRuleResult::<Identity, Impl, OFFSET>,
            SetBusinessRuleString: SetBusinessRuleString::<Identity, Impl, OFFSET>,
            BusinessRuleString: BusinessRuleString::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzBizRuleInterfaces_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AddInterface(this: &Self::This, bstrinterfacename: &::windows_core::BSTR, linterfaceflag: i32, varinterface: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddInterfaces(this: &Self::This, varinterfacenames: &super::super::System::Variant::VARIANT, varinterfaceflags: &super::super::System::Variant::VARIANT, varinterfaces: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetInterfaceValue(this: &Self::This, bstrinterfacename: &::windows_core::BSTR, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, bstrinterfacename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzBizRuleInterfaces {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzBizRuleInterfaces {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, linterfaceflag: i32, varinterface: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddInterface(this, ::core::mem::transmute(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute(&varinterface)).into())
        }
        unsafe extern "system" fn AddInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinterfacenames: super::super::System::Variant::VARIANT, varinterfaceflags: super::super::System::Variant::VARIANT, varinterfaces: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddInterfaces(this, ::core::mem::transmute(&varinterfacenames), ::core::mem::transmute(&varinterfaceflags), ::core::mem::transmute(&varinterfaces)).into())
        }
        unsafe extern "system" fn GetInterfaceValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterfaceValue(this, ::core::mem::transmute(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&bstrinterfacename)).into())
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAll(this).into())
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzBizRuleInterfaces_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddInterface: AddInterface::<Identity, Impl, OFFSET>,
            AddInterfaces: AddInterfaces::<Identity, Impl, OFFSET>,
            GetInterfaceValue: GetInterfaceValue::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzBizRuleParameters_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AddParameter(this: &Self::This, bstrparametername: &::windows_core::BSTR, varparametervalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddParameters(this: &Self::This, varparameternames: &super::super::System::Variant::VARIANT, varparametervalues: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetParameterValue(this: &Self::This, bstrparametername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Remove(this: &Self::This, varparametername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzBizRuleParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzBizRuleParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>, varparametervalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddParameter(this, ::core::mem::transmute(&bstrparametername), ::core::mem::transmute(&varparametervalue)).into())
        }
        unsafe extern "system" fn AddParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varparameternames: super::super::System::Variant::VARIANT, varparametervalues: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddParameters(this, ::core::mem::transmute(&varparameternames), ::core::mem::transmute(&varparametervalues)).into())
        }
        unsafe extern "system" fn GetParameterValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarparametervalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameterValue(this, ::core::mem::transmute(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarparametervalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&varparametername)).into())
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAll(this).into())
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzBizRuleParameters_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddParameter: AddParameter::<Identity, Impl, OFFSET>,
            AddParameters: AddParameters::<Identity, Impl, OFFSET>,
            GetParameterValue: GetParameterValue::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzClientContext_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AccessCheck(this: &Self::This, bstrobjectname: &::windows_core::BSTR, varscopenames: &super::super::System::Variant::VARIANT, varoperations: &super::super::System::Variant::VARIANT, varparameternames: &super::super::System::Variant::VARIANT, varparametervalues: &super::super::System::Variant::VARIANT, varinterfacenames: &super::super::System::Variant::VARIANT, varinterfaceflags: &super::super::System::Variant::VARIANT, varinterfaces: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetBusinessRuleString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserDn(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserSamCompat(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserDisplay(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserCanonical(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserUpn(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserDnsSamCompat(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetRoles(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RoleForAccessCheck(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRoleForAccessCheck(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzClientContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzClientContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccessCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varscopenames: super::super::System::Variant::VARIANT, varoperations: super::super::System::Variant::VARIANT, varparameternames: super::super::System::Variant::VARIANT, varparametervalues: super::super::System::Variant::VARIANT, varinterfacenames: super::super::System::Variant::VARIANT, varinterfaceflags: super::super::System::Variant::VARIANT, varinterfaces: super::super::System::Variant::VARIANT, pvarresults: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccessCheck(this, ::core::mem::transmute(&bstrobjectname), ::core::mem::transmute(&varscopenames), ::core::mem::transmute(&varoperations), ::core::mem::transmute(&varparameternames), ::core::mem::transmute(&varparametervalues), ::core::mem::transmute(&varinterfacenames), ::core::mem::transmute(&varinterfaceflags), ::core::mem::transmute(&varinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBusinessRuleString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBusinessRuleString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbusinessrulestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserDn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserDn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserSamCompat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserSamCompat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserDisplay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserCanonical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserCanonical(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserUpn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserUpn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserDnsSamCompat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserDnsSamCompat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRoles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarrolenames: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRoles(this, ::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarrolenames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoleForAccessCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleForAccessCheck(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRoleForAccessCheck(this, ::core::mem::transmute(&bstrprop)).into())
        }
        IAzClientContext_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccessCheck: AccessCheck::<Identity, Impl, OFFSET>,
            GetBusinessRuleString: GetBusinessRuleString::<Identity, Impl, OFFSET>,
            UserDn: UserDn::<Identity, Impl, OFFSET>,
            UserSamCompat: UserSamCompat::<Identity, Impl, OFFSET>,
            UserDisplay: UserDisplay::<Identity, Impl, OFFSET>,
            UserGuid: UserGuid::<Identity, Impl, OFFSET>,
            UserCanonical: UserCanonical::<Identity, Impl, OFFSET>,
            UserUpn: UserUpn::<Identity, Impl, OFFSET>,
            UserDnsSamCompat: UserDnsSamCompat::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRoles: GetRoles::<Identity, Impl, OFFSET>,
            RoleForAccessCheck: RoleForAccessCheck::<Identity, Impl, OFFSET>,
            SetRoleForAccessCheck: SetRoleForAccessCheck::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzClientContext2_Impl: ::windows_core::BaseImpl + IAzClientContext_Impl {
    fn GetAssignedScopesPage(this: &Self::This, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Variant::VARIANT, pvarscopenames: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddRoles(this: &Self::This, varroles: &super::super::System::Variant::VARIANT, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddApplicationGroups(this: &Self::This, varapplicationgroups: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddStringSids(this: &Self::This, varstringsids: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetLDAPQueryDN(this: &Self::This, bstrldapquerydn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LDAPQueryDN(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzClientContext2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzClientContext);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzClientContext2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAssignedScopesPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Variant::VARIANT, pvarscopenames: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAssignedScopesPage(this, ::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&pagesize), ::core::mem::transmute_copy(&pvarcursor), ::core::mem::transmute_copy(&pvarscopenames)).into())
        }
        unsafe extern "system" fn AddRoles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varroles: super::super::System::Variant::VARIANT, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRoles(this, ::core::mem::transmute(&varroles), ::core::mem::transmute(&bstrscopename)).into())
        }
        unsafe extern "system" fn AddApplicationGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varapplicationgroups: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddApplicationGroups(this, ::core::mem::transmute(&varapplicationgroups)).into())
        }
        unsafe extern "system" fn AddStringSids<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varstringsids: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStringSids(this, ::core::mem::transmute(&varstringsids)).into())
        }
        unsafe extern "system" fn SetLDAPQueryDN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLDAPQueryDN(this, ::core::mem::transmute(&bstrldapquerydn)).into())
        }
        unsafe extern "system" fn LDAPQueryDN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LDAPQueryDN(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrldapquerydn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzClientContext2_Vtbl {
            base__: <IAzClientContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAssignedScopesPage: GetAssignedScopesPage::<Identity, Impl, OFFSET>,
            AddRoles: AddRoles::<Identity, Impl, OFFSET>,
            AddApplicationGroups: AddApplicationGroups::<Identity, Impl, OFFSET>,
            AddStringSids: AddStringSids::<Identity, Impl, OFFSET>,
            SetLDAPQueryDN: SetLDAPQueryDN::<Identity, Impl, OFFSET>,
            LDAPQueryDN: LDAPQueryDN::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzClientContext3_Impl: ::windows_core::BaseImpl + IAzClientContext2_Impl {
    fn AccessCheck2(this: &Self::This, bstrobjectname: &::windows_core::BSTR, bstrscopename: &::windows_core::BSTR, loperation: i32) -> ::windows_core::Result<u32>;
    fn IsInRoleAssignment(this: &Self::This, bstrscopename: &::windows_core::BSTR, bstrrolename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetOperations(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzOperations>;
    fn GetTasks(this: &Self::This, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzTasks>;
    fn BizRuleParameters(this: &Self::This) -> ::windows_core::Result<IAzBizRuleParameters>;
    fn BizRuleInterfaces(this: &Self::This) -> ::windows_core::Result<IAzBizRuleInterfaces>;
    fn GetGroups(this: &Self::This, bstrscopename: &::windows_core::BSTR, uloptions: &AZ_PROP_CONSTANTS) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Sids(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzClientContext3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzClientContext2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzClientContext3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccessCheck2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccessCheck2(this, ::core::mem::transmute(&bstrobjectname), ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&loperation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsInRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbisinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInRoleAssignment(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&bstrrolename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOperations(this, ::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperationcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTasks(this, ::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BizRuleParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRuleParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbizruleparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BizRuleInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRuleInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbizruleinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, uloptions: u32, pgrouparray: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGroups(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&uloptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrouparray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Sids<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstringsidarray: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sids(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringsidarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzClientContext3_Vtbl {
            base__: <IAzClientContext2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccessCheck2: AccessCheck2::<Identity, Impl, OFFSET>,
            IsInRoleAssignment: IsInRoleAssignment::<Identity, Impl, OFFSET>,
            GetOperations: GetOperations::<Identity, Impl, OFFSET>,
            GetTasks: GetTasks::<Identity, Impl, OFFSET>,
            BizRuleParameters: BizRuleParameters::<Identity, Impl, OFFSET>,
            BizRuleInterfaces: BizRuleInterfaces::<Identity, Impl, OFFSET>,
            GetGroups: GetGroups::<Identity, Impl, OFFSET>,
            Sids: Sids::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzNameResolver_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn NameFromSid(this: &Self::This, bstrsid: &::windows_core::BSTR, psidtype: *mut i32, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamesFromSids(this: &Self::This, vsids: &super::super::System::Variant::VARIANT, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzNameResolver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzNameResolver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NameFromSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psidtype: *mut i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NameFromSid(this, ::core::mem::transmute(&bstrsid), ::core::mem::transmute_copy(&psidtype), ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn NamesFromSids<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vsids: super::super::System::Variant::VARIANT, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NamesFromSids(this, ::core::mem::transmute(&vsids), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames)).into())
        }
        IAzNameResolver_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NameFromSid: NameFromSid::<Identity, Impl, OFFSET>,
            NamesFromSids: NamesFromSids::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzObjectPicker_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetPrincipals(this: &Self::This, hparentwnd: super::super::Foundation::HWND, bstrtitle: &::windows_core::BSTR, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT, pvsids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzObjectPicker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzObjectPicker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrincipals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT, pvsids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrincipals(this, ::core::mem::transmute_copy(&hparentwnd), ::core::mem::transmute(&bstrtitle), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames), ::core::mem::transmute_copy(&pvsids)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzObjectPicker_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrincipals: GetPrincipals::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzOperation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(this: &Self::This, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OperationID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetOperationID(this: &Self::This, lprop: i32) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationData(this, ::core::mem::transmute(&bstrapplicationdata)).into())
        }
        unsafe extern "system" fn OperationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OperationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOperationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOperationID(this, ::core::mem::transmute_copy(&lprop)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        IAzOperation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            OperationID: OperationID::<Identity, Impl, OFFSET>,
            SetOperationID: SetOperationID::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzOperation2_Impl: ::windows_core::BaseImpl + IAzOperation_Impl {
    fn RoleAssignments(this: &Self::This, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzOperation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzOperation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzOperation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleAssignments(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzOperation2_Vtbl { base__: <IAzOperation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzOperations_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzOperations_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzPrincipalLocator_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn NameResolver(this: &Self::This) -> ::windows_core::Result<IAzNameResolver>;
    fn ObjectPicker(this: &Self::This) -> ::windows_core::Result<IAzObjectPicker>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzPrincipalLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzPrincipalLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NameResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnameresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NameResolver(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnameresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ObjectPicker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectPicker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectpicker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzPrincipalLocator_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NameResolver: NameResolver::<Identity, Impl, OFFSET>,
            ObjectPicker: ObjectPicker::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRole_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(this: &Self::This, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddAppMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteAppMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddTask(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteTask(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddOperation(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteOperation(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMember(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AppMembers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Members(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Operations(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Tasks(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddMemberName(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMemberName(this: &Self::This, bstrprop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MembersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzRole {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzRole {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationData(this, ::core::mem::transmute(&bstrapplicationdata)).into())
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAppMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAppMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTask(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTask(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOperation(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteOperation(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMember(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppMembers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Members(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Operations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tasks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMemberName(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMemberName(this, ::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn MembersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MembersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzRole_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            AddAppMember: AddAppMember::<Identity, Impl, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, Impl, OFFSET>,
            AddTask: AddTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            AddOperation: AddOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AppMembers: AppMembers::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            AddMemberName: AddMemberName::<Identity, Impl, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, Impl, OFFSET>,
            MembersName: MembersName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleAssignment_Impl: ::windows_core::BaseImpl + IAzRole_Impl {
    fn AddRoleDefinition(this: &Self::This, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteRoleDefinition(this: &Self::This, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleDefinitions(this: &Self::This) -> ::windows_core::Result<IAzRoleDefinitions>;
    fn Scope(this: &Self::This) -> ::windows_core::Result<IAzScope>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzRoleAssignment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzRole);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzRoleAssignment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRoleDefinition(this, ::core::mem::transmute(&bstrroledefinition)).into())
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRoleDefinition(this, ::core::mem::transmute(&bstrroledefinition)).into())
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleDefinitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzRoleAssignment_Vtbl {
            base__: <IAzRole as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRoleDefinition: AddRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleAssignments_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzRoleAssignments {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzRoleAssignments {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzRoleAssignments_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleDefinition_Impl: ::windows_core::BaseImpl + IAzTask_Impl {
    fn RoleAssignments(this: &Self::This, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
    fn AddRoleDefinition(this: &Self::This, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteRoleDefinition(this: &Self::This, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleDefinitions(this: &Self::This) -> ::windows_core::Result<IAzRoleDefinitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzRoleDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzTask);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzRoleDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleAssignments(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRoleDefinition(this, ::core::mem::transmute(&bstrroledefinition)).into())
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRoleDefinition(this, ::core::mem::transmute(&bstrroledefinition)).into())
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleDefinitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzRoleDefinition_Vtbl {
            base__: <IAzTask as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            AddRoleDefinition: AddRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleDefinitions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzRoleDefinitions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzRoleDefinitions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzRoleDefinitions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoles_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzRoles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzRoles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzRoles_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzScope_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(this: &Self::This, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PolicyAdministrators(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReaders(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministrator(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministrator(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReader(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReader(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ApplicationGroups(this: &Self::This) -> ::windows_core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Roles(this: &Self::This) -> ::windows_core::Result<IAzRoles>;
    fn OpenRole(this: &Self::This, bstrrolename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn CreateRole(this: &Self::This, bstrrolename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn DeleteRole(this: &Self::This, bstrrolename: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Tasks(this: &Self::This) -> ::windows_core::Result<IAzTasks>;
    fn OpenTask(this: &Self::This, bstrtaskname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn CreateTask(this: &Self::This, bstrtaskname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn DeleteTask(this: &Self::This, bstrtaskname: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CanBeDelegated(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn BizrulesWritable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn PolicyAdministratorsName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReadersName(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministratorName(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministratorName(this: &Self::This, bstradmin: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReaderName(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReaderName(this: &Self::This, bstrreader: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzScope {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzScope {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationData(this, ::core::mem::transmute(&bstrapplicationdata)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyAdministrators(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyReaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyAdministrator(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyAdministrator(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyReader(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyReader(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteApplicationGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Roles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Roles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprolecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenRole(this, ::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRole(this, ::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRole(this, ::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tasks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenTask(this, ::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTask(this, ::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTask(this, ::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn CanBeDelegated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanBeDelegated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BizrulesWritable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizrulesWritable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyAdministratorsName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PolicyReadersName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyAdministratorName(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyAdministratorName(this, ::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPolicyReaderName(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePolicyReaderName(this, ::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into())
        }
        IAzScope_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Roles: Roles::<Identity, Impl, OFFSET>,
            OpenRole: OpenRole::<Identity, Impl, OFFSET>,
            CreateRole: CreateRole::<Identity, Impl, OFFSET>,
            DeleteRole: DeleteRole::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            OpenTask: OpenTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            CanBeDelegated: CanBeDelegated::<Identity, Impl, OFFSET>,
            BizrulesWritable: BizrulesWritable::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzScope2_Impl: ::windows_core::BaseImpl + IAzScope_Impl {
    fn RoleDefinitions(this: &Self::This) -> ::windows_core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(this: &Self::This, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(this: &Self::This, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(this: &Self::This, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleAssignments(this: &Self::This) -> ::windows_core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(this: &Self::This, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(this: &Self::This, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(this: &Self::This, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzScope2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzScope);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzScope2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleDefinitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRoleDefinition(this, ::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenRoleDefinition(this, ::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRoleDefinition(this, ::core::mem::transmute(&bstrroledefinitionname)).into())
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleAssignments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRoleAssignment(this, ::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenRoleAssignment(this, ::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRoleAssignment(this, ::core::mem::transmute(&bstrroleassignmentname)).into())
        }
        IAzScope2_Vtbl {
            base__: <IAzScope as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, Impl, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, Impl, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, Impl, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzScopes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzScopes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzScopes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzScopes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzTask_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(this: &Self::This, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRule(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRule(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleLanguage(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleImportedPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleImportedPath(this: &Self::This, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsRoleDefinition(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsRoleDefinition(this: &Self::This, fprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Operations(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Tasks(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddOperation(this: &Self::This, bstrop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteOperation(this: &Self::This, bstrop: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddTask(this: &Self::This, bstrtask: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteTask(this: &Self::This, bstrtask: &::windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Writable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(this: &Self::This, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(this: &Self::This, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(this: &Self::This, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzTask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzTask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationData(this, ::core::mem::transmute(&bstrapplicationdata)).into())
        }
        unsafe extern "system" fn BizRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRule(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRuleLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRuleLanguage(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BizRuleImportedPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBizRuleImportedPath(this, ::core::mem::transmute(&bstrprop)).into())
        }
        unsafe extern "system" fn IsRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRoleDefinition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsRoleDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsRoleDefinition(this, ::core::mem::transmute_copy(&fprop)).into())
        }
        unsafe extern "system" fn Operations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tasks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOperation(this, ::core::mem::transmute(&bstrop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteOperation(this, ::core::mem::transmute(&bstrop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtask: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTask(this, ::core::mem::transmute(&bstrtask), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtask: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTask(this, ::core::mem::transmute(&bstrtask), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Writable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyItem(this, ::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into())
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Submit(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into())
        }
        IAzTask_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            BizRule: BizRule::<Identity, Impl, OFFSET>,
            SetBizRule: SetBizRule::<Identity, Impl, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, Impl, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, Impl, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, Impl, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, Impl, OFFSET>,
            IsRoleDefinition: IsRoleDefinition::<Identity, Impl, OFFSET>,
            SetIsRoleDefinition: SetIsRoleDefinition::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            AddOperation: AddOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            AddTask: AddTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzTask2_Impl: ::windows_core::BaseImpl + IAzTask_Impl {
    fn RoleAssignments(this: &Self::This, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzTask2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAzTask);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzTask2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTask2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoleAssignments(this, ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzTask2_Vtbl { base__: <IAzTask as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzTasks_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAzTasks {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAzTasks {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAzTasks_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
