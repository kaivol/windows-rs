#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSMan_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CreateSession(this: &Self::This, connection: &::windows_core::BSTR, flags: i32, connectionoptions: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<super::Com::IDispatch>;
    fn CreateConnectionOptions(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn CommandLine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSMan {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSMan {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connection: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, connectionoptions: *mut ::core::ffi::c_void, session: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSession(this, ::core::mem::transmute(&connection), ::core::mem::transmute_copy(&flags), ::windows_core::from_raw_borrowed(&connectionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(session, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateConnectionOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateConnectionOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommandLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommandLine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSMan_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Identity, Impl, OFFSET>,
            CommandLine: CommandLine::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManConnectionOptions_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn UserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetUserName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetPassword(this: &Self::This, password: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManConnectionOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManConnectionOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute(&password)).into())
        }
        IWSManConnectionOptions_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserName: UserName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManConnectionOptionsEx_Impl: ::windows_core::BaseImpl + IWSManConnectionOptions_Impl {
    fn CertificateThumbprint(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCertificateThumbprint(this: &Self::This, thumbprint: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManConnectionOptionsEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSManConnectionOptions);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManConnectionOptionsEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CertificateThumbprint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thumbprint: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CertificateThumbprint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(thumbprint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCertificateThumbprint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thumbprint: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCertificateThumbprint(this, ::core::mem::transmute(&thumbprint)).into())
        }
        IWSManConnectionOptionsEx_Vtbl {
            base__: <IWSManConnectionOptions as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CertificateThumbprint: CertificateThumbprint::<Identity, Impl, OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManConnectionOptionsEx2_Impl: ::windows_core::BaseImpl + IWSManConnectionOptionsEx_Impl {
    fn SetProxy(this: &Self::This, accesstype: i32, authenticationmechanism: i32, username: &::windows_core::BSTR, password: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ProxyIEConfig(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProxyWinHttpConfig(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProxyAutoDetect(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProxyNoProxyServer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProxyAuthenticationUseNegotiate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProxyAuthenticationUseBasic(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProxyAuthenticationUseDigest(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManConnectionOptionsEx2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSManConnectionOptionsEx);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManConnectionOptionsEx2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxy(this, ::core::mem::transmute_copy(&accesstype), ::core::mem::transmute_copy(&authenticationmechanism), ::core::mem::transmute(&username), ::core::mem::transmute(&password)).into())
        }
        unsafe extern "system" fn ProxyIEConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyIEConfig(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyWinHttpConfig(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyAutoDetect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyAutoDetect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyNoProxyServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyNoProxyServer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyAuthenticationUseNegotiate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyAuthenticationUseBasic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyAuthenticationUseDigest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManConnectionOptionsEx2_Vtbl {
            base__: <IWSManConnectionOptionsEx as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProxy: SetProxy::<Identity, Impl, OFFSET>,
            ProxyIEConfig: ProxyIEConfig::<Identity, Impl, OFFSET>,
            ProxyWinHttpConfig: ProxyWinHttpConfig::<Identity, Impl, OFFSET>,
            ProxyAutoDetect: ProxyAutoDetect::<Identity, Impl, OFFSET>,
            ProxyNoProxyServer: ProxyNoProxyServer::<Identity, Impl, OFFSET>,
            ProxyAuthenticationUseNegotiate: ProxyAuthenticationUseNegotiate::<Identity, Impl, OFFSET>,
            ProxyAuthenticationUseBasic: ProxyAuthenticationUseBasic::<Identity, Impl, OFFSET>,
            ProxyAuthenticationUseDigest: ProxyAuthenticationUseDigest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEnumerator_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ReadItem(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AtEndOfStream(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AtEndOfStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eos: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AtEndOfStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManEnumerator_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadItem: ReadItem::<Identity, Impl, OFFSET>,
            AtEndOfStream: AtEndOfStream::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEx_Impl: ::windows_core::BaseImpl + IWSMan_Impl {
    fn CreateResourceLocator(this: &Self::This, strresourcelocator: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn SessionFlagUTF8(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagCredUsernamePassword(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagSkipCACheck(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagSkipCNCheck(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseDigest(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseNegotiate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseBasic(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseKerberos(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagNoEncryption(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagEnableSPNServerPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseNoAuthentication(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagNonXmlText(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagReturnEPR(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagReturnObjectAndEPR(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetErrorMessage(this: &Self::This, errornumber: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnumerationFlagHierarchyDeep(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagHierarchyShallow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagHierarchyDeepBasePropsOnly(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagReturnObject(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSMan);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateResourceLocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strresourcelocator: ::std::mem::MaybeUninit<::windows_core::BSTR>, newresourcelocator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateResourceLocator(this, ::core::mem::transmute(&strresourcelocator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newresourcelocator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUTF8<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUTF8(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagCredUsernamePassword(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagSkipCACheck(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagSkipCNCheck(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseDigest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseDigest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseNegotiate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseBasic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseBasic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseKerberos(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagNoEncryption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagEnableSPNServerPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseNoAuthentication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagNonXmlText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagReturnEPR(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagReturnObjectAndEPR(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorMessage(this, ::core::mem::transmute_copy(&errornumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errormessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagHierarchyDeep(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagHierarchyShallow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagHierarchyDeepBasePropsOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagReturnObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManEx_Vtbl {
            base__: <IWSMan as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateResourceLocator: CreateResourceLocator::<Identity, Impl, OFFSET>,
            SessionFlagUTF8: SessionFlagUTF8::<Identity, Impl, OFFSET>,
            SessionFlagCredUsernamePassword: SessionFlagCredUsernamePassword::<Identity, Impl, OFFSET>,
            SessionFlagSkipCACheck: SessionFlagSkipCACheck::<Identity, Impl, OFFSET>,
            SessionFlagSkipCNCheck: SessionFlagSkipCNCheck::<Identity, Impl, OFFSET>,
            SessionFlagUseDigest: SessionFlagUseDigest::<Identity, Impl, OFFSET>,
            SessionFlagUseNegotiate: SessionFlagUseNegotiate::<Identity, Impl, OFFSET>,
            SessionFlagUseBasic: SessionFlagUseBasic::<Identity, Impl, OFFSET>,
            SessionFlagUseKerberos: SessionFlagUseKerberos::<Identity, Impl, OFFSET>,
            SessionFlagNoEncryption: SessionFlagNoEncryption::<Identity, Impl, OFFSET>,
            SessionFlagEnableSPNServerPort: SessionFlagEnableSPNServerPort::<Identity, Impl, OFFSET>,
            SessionFlagUseNoAuthentication: SessionFlagUseNoAuthentication::<Identity, Impl, OFFSET>,
            EnumerationFlagNonXmlText: EnumerationFlagNonXmlText::<Identity, Impl, OFFSET>,
            EnumerationFlagReturnEPR: EnumerationFlagReturnEPR::<Identity, Impl, OFFSET>,
            EnumerationFlagReturnObjectAndEPR: EnumerationFlagReturnObjectAndEPR::<Identity, Impl, OFFSET>,
            GetErrorMessage: GetErrorMessage::<Identity, Impl, OFFSET>,
            EnumerationFlagHierarchyDeep: EnumerationFlagHierarchyDeep::<Identity, Impl, OFFSET>,
            EnumerationFlagHierarchyShallow: EnumerationFlagHierarchyShallow::<Identity, Impl, OFFSET>,
            EnumerationFlagHierarchyDeepBasePropsOnly: EnumerationFlagHierarchyDeepBasePropsOnly::<Identity, Impl, OFFSET>,
            EnumerationFlagReturnObject: EnumerationFlagReturnObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEx2_Impl: ::windows_core::BaseImpl + IWSManEx_Impl {
    fn SessionFlagUseClientCertificate(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManEx2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSManEx);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManEx2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseClientCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManEx2_Vtbl {
            base__: <IWSManEx as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEx3_Impl: ::windows_core::BaseImpl + IWSManEx2_Impl {
    fn SessionFlagUTF16(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseCredSsp(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagAssociationInstance(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumerationFlagAssociatedInstance(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagSkipRevocationCheck(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagAllowNegotiateImplicitCredentials(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SessionFlagUseSsl(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManEx3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSManEx2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManEx3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SessionFlagUTF16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUTF16(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseCredSsp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagAssociationInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationFlagAssociatedInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagSkipRevocationCheck(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagAllowNegotiateImplicitCredentials(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionFlagUseSsl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionFlagUseSsl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManEx3_Vtbl {
            base__: <IWSManEx2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SessionFlagUTF16: SessionFlagUTF16::<Identity, Impl, OFFSET>,
            SessionFlagUseCredSsp: SessionFlagUseCredSsp::<Identity, Impl, OFFSET>,
            EnumerationFlagAssociationInstance: EnumerationFlagAssociationInstance::<Identity, Impl, OFFSET>,
            EnumerationFlagAssociatedInstance: EnumerationFlagAssociatedInstance::<Identity, Impl, OFFSET>,
            SessionFlagSkipRevocationCheck: SessionFlagSkipRevocationCheck::<Identity, Impl, OFFSET>,
            SessionFlagAllowNegotiateImplicitCredentials: SessionFlagAllowNegotiateImplicitCredentials::<Identity, Impl, OFFSET>,
            SessionFlagUseSsl: SessionFlagUseSsl::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManInternal_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ConfigSDDL(this: &Self::This, session: ::core::option::Option<&super::Com::IDispatch>, resourceuri: &super::Variant::VARIANT, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManInternal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManInternal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManInternal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigSDDL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManInternal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32, resource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConfigSDDL(this, ::windows_core::from_raw_borrowed(&session), ::core::mem::transmute(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManInternal_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ConfigSDDL: ConfigSDDL::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManResourceLocator_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SetResourceURI(this: &Self::This, uri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ResourceURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddSelector(this: &Self::This, resourceselname: &::windows_core::BSTR, selvalue: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ClearSelectors(this: &Self::This) -> ::windows_core::Result<()>;
    fn FragmentPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFragmentPath(this: &Self::This, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FragmentDialect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFragmentDialect(this: &Self::This, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddOption(this: &Self::This, optionname: &::windows_core::BSTR, optionvalue: &super::Variant::VARIANT, mustcomply: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetMustUnderstandOptions(this: &Self::This, mustunderstand: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn MustUnderstandOptions(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ClearOptions(this: &Self::This) -> ::windows_core::Result<()>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManResourceLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManResourceLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetResourceURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResourceURI(this, ::core::mem::transmute(&uri)).into())
        }
        unsafe extern "system" fn ResourceURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResourceURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddSelector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceselname: ::std::mem::MaybeUninit<::windows_core::BSTR>, selvalue: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSelector(this, ::core::mem::transmute(&resourceselname), ::core::mem::transmute(&selvalue)).into())
        }
        unsafe extern "system" fn ClearSelectors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearSelectors(this).into())
        }
        unsafe extern "system" fn FragmentPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FragmentPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFragmentPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFragmentPath(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn FragmentDialect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FragmentDialect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFragmentDialect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFragmentDialect(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn AddOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionvalue: super::Variant::VARIANT, mustcomply: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOption(this, ::core::mem::transmute(&optionname), ::core::mem::transmute(&optionvalue), ::core::mem::transmute_copy(&mustcomply)).into())
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMustUnderstandOptions(this, ::core::mem::transmute_copy(&mustunderstand)).into())
        }
        unsafe extern "system" fn MustUnderstandOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MustUnderstandOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mustunderstand, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClearOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearOptions(this).into())
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSManResourceLocator_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetResourceURI: SetResourceURI::<Identity, Impl, OFFSET>,
            ResourceURI: ResourceURI::<Identity, Impl, OFFSET>,
            AddSelector: AddSelector::<Identity, Impl, OFFSET>,
            ClearSelectors: ClearSelectors::<Identity, Impl, OFFSET>,
            FragmentPath: FragmentPath::<Identity, Impl, OFFSET>,
            SetFragmentPath: SetFragmentPath::<Identity, Impl, OFFSET>,
            FragmentDialect: FragmentDialect::<Identity, Impl, OFFSET>,
            SetFragmentDialect: SetFragmentDialect::<Identity, Impl, OFFSET>,
            AddOption: AddOption::<Identity, Impl, OFFSET>,
            SetMustUnderstandOptions: SetMustUnderstandOptions::<Identity, Impl, OFFSET>,
            MustUnderstandOptions: MustUnderstandOptions::<Identity, Impl, OFFSET>,
            ClearOptions: ClearOptions::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWSManResourceLocatorInternal_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IWSManResourceLocatorInternal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManResourceLocatorInternal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManResourceLocatorInternal {
    const VTABLE: Self::Vtable = { IWSManResourceLocatorInternal_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManSession_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Get(this: &Self::This, resourceuri: &super::Variant::VARIANT, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Put(this: &Self::This, resourceuri: &super::Variant::VARIANT, resource: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Create(this: &Self::This, resourceuri: &super::Variant::VARIANT, resource: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Delete(this: &Self::This, resourceuri: &super::Variant::VARIANT, flags: i32) -> ::windows_core::Result<()>;
    fn Invoke2(this: &Self::This, actionuri: &::windows_core::BSTR, resourceuri: &super::Variant::VARIANT, parameters: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enumerate(this: &Self::This, resourceuri: &super::Variant::VARIANT, filter: &::windows_core::BSTR, dialect: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Identify(this: &Self::This, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BatchItems(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBatchItems(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn Timeout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTimeout(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSManSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSManSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32, resource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Put<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, resource: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, resultresource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Put(this, ::core::mem::transmute(&resourceuri), ::core::mem::transmute(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, resource: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, newuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute(&resourceuri), ::core::mem::transmute(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&resourceuri), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn Invoke2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actionuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, resourceuri: super::Variant::VARIANT, parameters: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, result: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Invoke2(this, ::core::mem::transmute(&actionuri), ::core::mem::transmute(&resourceuri), ::core::mem::transmute(&parameters), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enumerate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, filter: ::std::mem::MaybeUninit<::windows_core::BSTR>, dialect: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, resultset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enumerate(this, ::core::mem::transmute(&resourceuri), ::core::mem::transmute(&filter), ::core::mem::transmute(&dialect), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Identify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32, result: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Identify(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BatchItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BatchItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBatchItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBatchItems(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Timeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeout(this, ::core::mem::transmute_copy(&value)).into())
        }
        IWSManSession_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Invoke2: Invoke2::<Identity, Impl, OFFSET>,
            Enumerate: Enumerate::<Identity, Impl, OFFSET>,
            Identify: Identify::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            BatchItems: BatchItems::<Identity, Impl, OFFSET>,
            SetBatchItems: SetBatchItems::<Identity, Impl, OFFSET>,
            Timeout: Timeout::<Identity, Impl, OFFSET>,
            SetTimeout: SetTimeout::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
