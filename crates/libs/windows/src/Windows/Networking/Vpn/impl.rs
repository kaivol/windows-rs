pub trait IVpnChannelStatics_Impl: ::windows_core::BaseImpl {
    fn ProcessEventAsync(this: &Self::This, thirdpartyplugin: ::core::option::Option<&::windows_core::IInspectable>, event: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVpnChannelStatics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnChannelStatics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnChannelStatics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessEventAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnChannelStatics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessEventAsync(this, ::windows_core::from_raw_borrowed(&thirdpartyplugin), ::windows_core::from_raw_borrowed(&event)).into())
        }
        IVpnChannelStatics_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProcessEventAsync: ProcessEventAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Security_Credentials\"`, `\"Security_Cryptography_Certificates\"`"]
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IVpnCredential_Impl: ::windows_core::BaseImpl {
    fn PasskeyCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn OldPasswordCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows_core::Iids for IVpnCredential {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCredential_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnCredential {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PasskeyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCredential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasskeyCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CertificateCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCredential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CertificateCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AdditionalPin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCredential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdditionalPin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OldPasswordCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCredential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OldPasswordCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnCredential_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PasskeyCredential: PasskeyCredential::<Identity, Impl, OFFSET>,
            CertificateCredential: CertificateCredential::<Identity, Impl, OFFSET>,
            AdditionalPin: AdditionalPin::<Identity, Impl, OFFSET>,
            OldPasswordCredential: OldPasswordCredential::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVpnCustomPrompt_Impl: ::windows_core::BaseImpl {
    fn SetLabel(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetCompulsory(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Compulsory(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetBordered(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Bordered(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IVpnCustomPrompt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnCustomPrompt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompulsory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompulsory(this, value).into())
        }
        unsafe extern "system" fn Compulsory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compulsory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBordered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBordered(this, value).into())
        }
        unsafe extern "system" fn Bordered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPrompt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bordered(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnCustomPrompt_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetCompulsory: SetCompulsory::<Identity, Impl, OFFSET>,
            Compulsory: Compulsory::<Identity, Impl, OFFSET>,
            SetBordered: SetBordered::<Identity, Impl, OFFSET>,
            Bordered: Bordered::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVpnCustomPromptElement_Impl: ::windows_core::BaseImpl {
    fn SetDisplayName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetCompulsory(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Compulsory(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetEmphasized(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Emphasized(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IVpnCustomPromptElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnCustomPromptElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompulsory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompulsory(this, value).into())
        }
        unsafe extern "system" fn Compulsory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compulsory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEmphasized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEmphasized(this, value).into())
        }
        unsafe extern "system" fn Emphasized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnCustomPromptElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Emphasized(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnCustomPromptElement_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetCompulsory: SetCompulsory::<Identity, Impl, OFFSET>,
            Compulsory: Compulsory::<Identity, Impl, OFFSET>,
            SetEmphasized: SetEmphasized::<Identity, Impl, OFFSET>,
            Emphasized: Emphasized::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnDomainNameInfoFactory_Impl: ::windows_core::BaseImpl {
    fn CreateVpnDomainNameInfo(this: &Self::This, name: &::windows_core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: ::core::option::Option<&super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: ::core::option::Option<&super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows_core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IVpnDomainNameInfoFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnDomainNameInfoFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnDomainNameInfoFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnDomainNameInfoFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: *mut ::core::ffi::c_void, proxyserverlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVpnDomainNameInfo(this, ::core::mem::transmute(&name), nametype, ::windows_core::from_raw_borrowed(&dnsserverlist), ::windows_core::from_raw_borrowed(&proxyserverlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnDomainNameInfoFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVpnDomainNameInfo: CreateVpnDomainNameInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVpnInterfaceIdFactory_Impl: ::windows_core::BaseImpl {
    fn CreateVpnInterfaceId(this: &Self::This, address: &[u8]) -> ::windows_core::Result<VpnInterfaceId>;
}
impl ::windows_core::Iids for IVpnInterfaceIdFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnInterfaceIdFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnInterfaceIdFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVpnInterfaceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnInterfaceIdFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVpnInterfaceId(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&address), address_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnInterfaceIdFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVpnInterfaceId: CreateVpnInterfaceId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnNamespaceInfoFactory_Impl: ::windows_core::BaseImpl {
    fn CreateVpnNamespaceInfo(this: &Self::This, name: &::windows_core::HSTRING, dnsserverlist: ::core::option::Option<&super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: ::core::option::Option<&super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows_core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IVpnNamespaceInfoFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnNamespaceInfoFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnNamespaceInfoFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnNamespaceInfoFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, dnsserverlist: *mut ::core::ffi::c_void, proxyserverlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVpnNamespaceInfo(this, ::core::mem::transmute(&name), ::windows_core::from_raw_borrowed(&dnsserverlist), ::windows_core::from_raw_borrowed(&proxyserverlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnNamespaceInfoFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVpnNamespaceInfo: CreateVpnNamespaceInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVpnPacketBufferFactory_Impl: ::windows_core::BaseImpl {
    fn CreateVpnPacketBuffer(this: &Self::This, parentbuffer: ::core::option::Option<&VpnPacketBuffer>, offset: u32, length: u32) -> ::windows_core::Result<VpnPacketBuffer>;
}
impl ::windows_core::Iids for IVpnPacketBufferFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPacketBufferFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnPacketBufferFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVpnPacketBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPacketBufferFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentbuffer: *mut ::core::ffi::c_void, offset: u32, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVpnPacketBuffer(this, ::windows_core::from_raw_borrowed(&parentbuffer), offset, length) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnPacketBufferFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVpnPacketBuffer: CreateVpnPacketBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVpnPlugIn_Impl: ::windows_core::BaseImpl {
    fn Connect(this: &Self::This, channel: ::core::option::Option<&VpnChannel>) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This, channel: ::core::option::Option<&VpnChannel>) -> ::windows_core::Result<()>;
    fn GetKeepAlivePayload(this: &Self::This, channel: ::core::option::Option<&VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows_core::Result<()>;
    fn Encapsulate(this: &Self::This, channel: ::core::option::Option<&VpnChannel>, packets: ::core::option::Option<&VpnPacketBufferList>, encapulatedpackets: ::core::option::Option<&VpnPacketBufferList>) -> ::windows_core::Result<()>;
    fn Decapsulate(this: &Self::This, channel: ::core::option::Option<&VpnChannel>, encapbuffer: ::core::option::Option<&VpnPacketBuffer>, decapsulatedpackets: ::core::option::Option<&VpnPacketBufferList>, controlpacketstosend: ::core::option::Option<&VpnPacketBufferList>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVpnPlugIn {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPlugIn_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnPlugIn {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::windows_core::from_raw_borrowed(&channel)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this, ::windows_core::from_raw_borrowed(&channel)).into())
        }
        unsafe extern "system" fn GetKeepAlivePayload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, keepalivepacket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKeepAlivePayload(this, ::windows_core::from_raw_borrowed(&channel), ::core::mem::transmute_copy(&keepalivepacket)).into())
        }
        unsafe extern "system" fn Encapsulate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, packets: *mut ::core::ffi::c_void, encapulatedpackets: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Encapsulate(this, ::windows_core::from_raw_borrowed(&channel), ::windows_core::from_raw_borrowed(&packets), ::windows_core::from_raw_borrowed(&encapulatedpackets)).into())
        }
        unsafe extern "system" fn Decapsulate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, encapbuffer: *mut ::core::ffi::c_void, decapsulatedpackets: *mut ::core::ffi::c_void, controlpacketstosend: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Decapsulate(this, ::windows_core::from_raw_borrowed(&channel), ::windows_core::from_raw_borrowed(&encapbuffer), ::windows_core::from_raw_borrowed(&decapsulatedpackets), ::windows_core::from_raw_borrowed(&controlpacketstosend)).into())
        }
        IVpnPlugIn_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetKeepAlivePayload: GetKeepAlivePayload::<Identity, Impl, OFFSET>,
            Encapsulate: Encapsulate::<Identity, Impl, OFFSET>,
            Decapsulate: Decapsulate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnProfile_Impl: ::windows_core::BaseImpl {
    fn ProfileName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetProfileName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn AppTriggers(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetRememberCredentials(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn AlwaysOn(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetAlwaysOn(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IVpnProfile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnProfile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProfileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProfileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProfileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProfileName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn AppTriggers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppTriggers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Routes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Routes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainNameInfoList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainNameInfoList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrafficFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrafficFilters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RememberCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RememberCredentials(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRememberCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRememberCredentials(this, value).into())
        }
        unsafe extern "system" fn AlwaysOn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AlwaysOn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAlwaysOn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlwaysOn(this, value).into())
        }
        IVpnProfile_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProfileName: ProfileName::<Identity, Impl, OFFSET>,
            SetProfileName: SetProfileName::<Identity, Impl, OFFSET>,
            AppTriggers: AppTriggers::<Identity, Impl, OFFSET>,
            Routes: Routes::<Identity, Impl, OFFSET>,
            DomainNameInfoList: DomainNameInfoList::<Identity, Impl, OFFSET>,
            TrafficFilters: TrafficFilters::<Identity, Impl, OFFSET>,
            RememberCredentials: RememberCredentials::<Identity, Impl, OFFSET>,
            SetRememberCredentials: SetRememberCredentials::<Identity, Impl, OFFSET>,
            AlwaysOn: AlwaysOn::<Identity, Impl, OFFSET>,
            SetAlwaysOn: SetAlwaysOn::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVpnRouteFactory_Impl: ::windows_core::BaseImpl {
    fn CreateVpnRoute(this: &Self::This, address: ::core::option::Option<&super::HostName>, prefixsize: u8) -> ::windows_core::Result<VpnRoute>;
}
impl ::windows_core::Iids for IVpnRouteFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnRouteFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVpnRouteFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVpnRoute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVpnRouteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void, prefixsize: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVpnRoute(this, ::windows_core::from_raw_borrowed(&address), prefixsize) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVpnRouteFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVpnRoute: CreateVpnRoute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
