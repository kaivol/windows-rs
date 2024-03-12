#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IChannelCredentials_Impl: ::windows_core::BaseImpl + super::IDispatch_Impl {
    fn SetWindowsCredential(this: &Self::This, domain: &::windows_core::BSTR, username: &::windows_core::BSTR, password: &::windows_core::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetUserNameCredential(this: &Self::This, username: &::windows_core::BSTR, password: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetClientCertificateFromStore(this: &Self::This, storelocation: &::windows_core::BSTR, storename: &::windows_core::BSTR, findyype: &::windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetClientCertificateFromStoreByName(this: &Self::This, subjectname: &::windows_core::BSTR, storelocation: &::windows_core::BSTR, storename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetClientCertificateFromFile(this: &Self::This, filename: &::windows_core::BSTR, password: &::windows_core::BSTR, keystorageflags: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(this: &Self::This, storelocation: &::windows_core::BSTR, storename: &::windows_core::BSTR, findtype: &::windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(this: &Self::This, subjectname: &::windows_core::BSTR, storelocation: &::windows_core::BSTR, storename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(this: &Self::This, filename: &::windows_core::BSTR, password: &::windows_core::BSTR, keystorageflags: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetServiceCertificateAuthentication(this: &Self::This, storelocation: &::windows_core::BSTR, revocationmode: &::windows_core::BSTR, certificatevalidationmode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetIssuedToken(this: &Self::This, localissueraddres: &::windows_core::BSTR, localissuerbindingtype: &::windows_core::BSTR, localissuerbinding: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IChannelCredentials {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChannelCredentials {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetWindowsCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWindowsCredential(this, ::core::mem::transmute(&domain), ::core::mem::transmute(&username), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&impersonationlevel), ::core::mem::transmute_copy(&allowntlm)).into())
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserNameCredential(this, ::core::mem::transmute(&username), ::core::mem::transmute(&password)).into())
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>, findyype: ::std::mem::MaybeUninit<::windows_core::BSTR>, findvalue: super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificateFromStore(this, ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&findyype), ::core::mem::transmute(&findvalue)).into())
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificateFromStoreByName(this, ::core::mem::transmute(&subjectname), ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename)).into())
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificateFromFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute(&password), ::core::mem::transmute(&keystorageflags)).into())
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>, findtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, findvalue: super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultServiceCertificateFromStore(this, ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&findtype), ::core::mem::transmute(&findvalue)).into())
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultServiceCertificateFromStoreByName(this, ::core::mem::transmute(&subjectname), ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename)).into())
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultServiceCertificateFromFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute(&password), ::core::mem::transmute(&keystorageflags)).into())
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, revocationmode: ::std::mem::MaybeUninit<::windows_core::BSTR>, certificatevalidationmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceCertificateAuthentication(this, ::core::mem::transmute(&storelocation), ::core::mem::transmute(&revocationmode), ::core::mem::transmute(&certificatevalidationmode)).into())
        }
        unsafe extern "system" fn SetIssuedToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localissueraddres: ::std::mem::MaybeUninit<::windows_core::BSTR>, localissuerbindingtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, localissuerbinding: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIssuedToken(this, ::core::mem::transmute(&localissueraddres), ::core::mem::transmute(&localissuerbindingtype), ::core::mem::transmute(&localissuerbinding)).into())
        }
        IChannelCredentials_Vtbl {
            base__: <super::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetWindowsCredential: SetWindowsCredential::<Identity, Impl, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, Impl, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, Impl, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, Impl, OFFSET>,
        }
    };
}
