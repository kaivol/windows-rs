#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<*mut super::super::super::Security::Cryptography::CERT_CONTEXT>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcCertificateEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows_core::Iids for IOpcCertificateEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcCertificateEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *mut *mut super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcCertificateEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateSet_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcCertificateEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows_core::Iids for IOpcCertificateSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcCertificateSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&certificate)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&certificate)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcCertificateSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcDigitalSignature_Impl: ::windows_core::BaseImpl {
    fn GetNamespaces(this: &Self::This, prefixes: *mut *mut ::windows_core::PWSTR, namespaces: *mut *mut ::windows_core::PWSTR, count: *mut u32) -> ::windows_core::Result<()>;
    fn GetSignatureId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignaturePartName(this: &Self::This) -> ::windows_core::Result<IOpcPartUri>;
    fn GetSignatureMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetCanonicalizationMethod(this: &Self::This) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetSignatureValue(this: &Self::This, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn GetSignaturePartReferenceEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
    fn GetSignatureRelationshipReferenceEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
    fn GetSigningTime(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetTimeFormat(this: &Self::This) -> ::windows_core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn GetPackageObjectReference(this: &Self::This) -> ::windows_core::Result<IOpcSignatureReference>;
    fn GetCertificateEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcCertificateEnumerator>;
    fn GetCustomReferenceEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignatureReferenceEnumerator>;
    fn GetCustomObjectEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
    fn GetSignatureXml(this: &Self::This, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcDigitalSignature {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcDigitalSignature {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::windows_core::PWSTR, namespaces: *mut *mut ::windows_core::PWSTR, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamespaces(this, ::core::mem::transmute_copy(&prefixes), ::core::mem::transmute_copy(&namespaces), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn GetSignatureId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignaturePartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCanonicalizationMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(canonicalizationmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignatureValue(this, ::core::mem::transmute_copy(&signaturevalue), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignaturePartReferenceEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureRelationshipReferenceEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSigningTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signingtime: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSigningTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signingtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTimeFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimeFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageObjectReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageObjectReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageobjectreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomReferenceEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomObjectEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignatureXml(this, ::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into())
        }
        IOpcDigitalSignature_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            GetCanonicalizationMethod: GetCanonicalizationMethod::<Identity, Impl, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, Impl, OFFSET>,
            GetSignaturePartReferenceEnumerator: GetSignaturePartReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureRelationshipReferenceEnumerator: GetSignatureRelationshipReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, Impl, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, Impl, OFFSET>,
            GetPackageObjectReference: GetPackageObjectReference::<Identity, Impl, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, Impl, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcDigitalSignatureEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcDigitalSignature>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcDigitalSignatureEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcDigitalSignatureEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcDigitalSignatureEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digitalsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcDigitalSignatureEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
pub trait IOpcDigitalSignatureManager_Impl: ::windows_core::BaseImpl {
    fn GetSignatureOriginPartName(this: &Self::This) -> ::windows_core::Result<IOpcPartUri>;
    fn SetSignatureOriginPartName(this: &Self::This, signatureoriginpartname: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetSignatureEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcDigitalSignatureEnumerator>;
    fn RemoveSignature(this: &Self::This, signaturepartname: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<()>;
    fn CreateSigningOptions(this: &Self::This) -> ::windows_core::Result<IOpcSigningOptions>;
    fn Validate(this: &Self::This, signature: ::core::option::Option<&IOpcDigitalSignature>, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<OPC_SIGNATURE_VALIDATION_RESULT>;
    fn Sign(this: &Self::This, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::core::option::Option<&IOpcSigningOptions>) -> ::windows_core::Result<IOpcDigitalSignature>;
    fn ReplaceSignatureXml(this: &Self::This, signaturepartname: ::core::option::Option<&IOpcPartUri>, newsignaturexml: *const u8, count: u32) -> ::windows_core::Result<IOpcDigitalSignature>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpcDigitalSignatureManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcDigitalSignatureManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureOriginPartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureoriginpartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureOriginPartName(this, ::windows_core::from_raw_borrowed(&signatureoriginpartname)).into())
        }
        unsafe extern "system" fn GetSignatureEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSignature(this, ::windows_core::from_raw_borrowed(&signaturepartname)).into())
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSigningOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signingoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Validate(this, ::windows_core::from_raw_borrowed(&signature), ::core::mem::transmute_copy(&certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validationresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Sign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sign(this, ::core::mem::transmute_copy(&certificate), ::windows_core::from_raw_borrowed(&signingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digitalsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReplaceSignatureXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplaceSignatureXml(this, ::windows_core::from_raw_borrowed(&signaturepartname), ::core::mem::transmute_copy(&newsignaturexml), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digitalsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcDigitalSignatureManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            GetSignatureEnumerator: GetSignatureEnumerator::<Identity, Impl, OFFSET>,
            RemoveSignature: RemoveSignature::<Identity, Impl, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            Sign: Sign::<Identity, Impl, OFFSET>,
            ReplaceSignatureXml: ReplaceSignatureXml::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
pub trait IOpcFactory_Impl: ::windows_core::BaseImpl {
    fn CreatePackageRootUri(this: &Self::This) -> ::windows_core::Result<IOpcUri>;
    fn CreatePartUri(this: &Self::This, pwzuri: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpcPartUri>;
    fn CreateStreamOnFile(this: &Self::This, filename: &::windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
    fn CreatePackage(this: &Self::This) -> ::windows_core::Result<IOpcPackage>;
    fn ReadPackageFromStream(this: &Self::This, stream: ::core::option::Option<&super::super::super::System::Com::IStream>, flags: OPC_READ_FLAGS) -> ::windows_core::Result<IOpcPackage>;
    fn WritePackageToStream(this: &Self::This, package: ::core::option::Option<&IOpcPackage>, flags: OPC_WRITE_FLAGS, stream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CreateDigitalSignatureManager(this: &Self::This, package: ::core::option::Option<&IOpcPackage>) -> ::windows_core::Result<IOpcDigitalSignatureManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpcFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePackageRootUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rooturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageRootUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rooturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePartUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzuri: ::windows_core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePartUri(this, ::core::mem::transmute(&pwzuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStreamOnFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStreamOnFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&iomode), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&dwflagsandattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadPackageFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadPackageFromStream(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WritePackageToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WritePackageToStream(this, ::windows_core::from_raw_borrowed(&package), ::core::mem::transmute_copy(&flags), ::windows_core::from_raw_borrowed(&stream)).into())
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, signaturemanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDigitalSignatureManager(this, ::windows_core::from_raw_borrowed(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePackageRootUri: CreatePackageRootUri::<Identity, Impl, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, Impl, OFFSET>,
            CreateStreamOnFile: CreateStreamOnFile::<Identity, Impl, OFFSET>,
            CreatePackage: CreatePackage::<Identity, Impl, OFFSET>,
            ReadPackageFromStream: ReadPackageFromStream::<Identity, Impl, OFFSET>,
            WritePackageToStream: WritePackageToStream::<Identity, Impl, OFFSET>,
            CreateDigitalSignatureManager: CreateDigitalSignatureManager::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpcPackage_Impl: ::windows_core::BaseImpl {
    fn GetPartSet(this: &Self::This) -> ::windows_core::Result<IOpcPartSet>;
    fn GetRelationshipSet(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSet>;
}
impl ::windows_core::Iids for IOpcPackage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPackage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcPackage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelationshipSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcPackage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartSet: GetPartSet::<Identity, Impl, OFFSET>,
            GetRelationshipSet: GetRelationshipSet::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPart_Impl: ::windows_core::BaseImpl {
    fn GetRelationshipSet(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSet>;
    fn GetContentStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<IOpcPartUri>;
    fn GetContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetCompressionOptions(this: &Self::This) -> ::windows_core::Result<OPC_COMPRESSION_OPTIONS>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcPart {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcPart {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRelationshipSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCompressionOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCompressionOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compressionoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcPart_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRelationshipSet: GetRelationshipSet::<Identity, Impl, OFFSET>,
            GetContentStream: GetContentStream::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            GetCompressionOptions: GetCompressionOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcPartEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcPart>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcPartEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcPartEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcPartEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcPartEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPartSet_Impl: ::windows_core::BaseImpl {
    fn GetPart(this: &Self::This, name: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<IOpcPart>;
    fn CreatePart(this: &Self::This, name: ::core::option::Option<&IOpcPartUri>, contenttype: &::windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows_core::Result<IOpcPart>;
    fn DeletePart(this: &Self::This, name: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<()>;
    fn PartExists(this: &Self::This, name: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcPartEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpcPartSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcPartSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPart(this, ::windows_core::from_raw_borrowed(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, contenttype: ::windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePart(this, ::windows_core::from_raw_borrowed(&name), ::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&compressionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeletePart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePart(this, ::windows_core::from_raw_borrowed(&name)).into())
        }
        unsafe extern "system" fn PartExists<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PartExists(this, ::windows_core::from_raw_borrowed(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partexists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcPartSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPart: GetPart::<Identity, Impl, OFFSET>,
            CreatePart: CreatePart::<Identity, Impl, OFFSET>,
            DeletePart: DeletePart::<Identity, Impl, OFFSET>,
            PartExists: PartExists::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPartUri_Impl: ::windows_core::BaseImpl + IOpcUri_Impl {
    fn ComparePartUri(this: &Self::This, parturi: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<i32>;
    fn GetSourceUri(this: &Self::This) -> ::windows_core::Result<IOpcUri>;
    fn IsRelationshipsPartUri(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpcPartUri {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOpcUri);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcPartUri {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComparePartUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, comparisonresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComparePartUri(this, ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comparisonresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRelationshipsPartUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isrelationshipuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcPartUri_Vtbl {
            base__: <IOpcUri as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComparePartUri: ComparePartUri::<Identity, Impl, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, Impl, OFFSET>,
            IsRelationshipsPartUri: IsRelationshipsPartUri::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcRelationship_Impl: ::windows_core::BaseImpl {
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetRelationshipType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSourceUri(this: &Self::This) -> ::windows_core::Result<IOpcUri>;
    fn GetTargetUri(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetTargetMode(this: &Self::This) -> ::windows_core::Result<OPC_URI_TARGET_MODE>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcRelationship {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcRelationship {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelationshipType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshiptype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshiptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targeturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targeturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcRelationship_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetRelationshipType: GetRelationshipType::<Identity, Impl, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, Impl, OFFSET>,
            GetTargetUri: GetTargetUri::<Identity, Impl, OFFSET>,
            GetTargetMode: GetTargetMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcRelationship>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcRelationshipEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcRelationshipEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationship, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcRelationshipEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpcRelationshipSelector_Impl: ::windows_core::BaseImpl {
    fn GetSelectorType(this: &Self::This) -> ::windows_core::Result<OPC_RELATIONSHIP_SELECTOR>;
    fn GetSelectionCriterion(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IOpcRelationshipSelector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcRelationshipSelector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSelectorType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelectorType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSelectionCriterion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectioncriterion: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelectionCriterion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectioncriterion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcRelationshipSelector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSelectorType: GetSelectorType::<Identity, Impl, OFFSET>,
            GetSelectionCriterion: GetSelectionCriterion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipSelectorEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSelector>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcRelationshipSelectorEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcRelationshipSelectorEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipselector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcRelationshipSelectorEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpcRelationshipSelectorSet_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpcRelationshipSelector>;
    fn Delete(this: &Self::This, relationshipselector: ::core::option::Option<&IOpcRelationshipSelector>) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
impl ::windows_core::Iids for IOpcRelationshipSelectorSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcRelationshipSelectorSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: ::windows_core::PCWSTR, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&selector), ::core::mem::transmute(&selectioncriterion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipselector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::windows_core::from_raw_borrowed(&relationshipselector)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipselectorenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcRelationshipSelectorSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcRelationshipSet_Impl: ::windows_core::BaseImpl {
    fn GetRelationship(this: &Self::This, relationshipidentifier: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpcRelationship>;
    fn CreateRelationship(this: &Self::This, relationshipidentifier: &::windows_core::PCWSTR, relationshiptype: &::windows_core::PCWSTR, targeturi: ::core::option::Option<&super::super::super::System::Com::IUri>, targetmode: OPC_URI_TARGET_MODE) -> ::windows_core::Result<IOpcRelationship>;
    fn DeleteRelationship(this: &Self::This, relationshipidentifier: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RelationshipExists(this: &Self::This, relationshipidentifier: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipEnumerator>;
    fn GetEnumeratorForType(this: &Self::This, relationshiptype: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpcRelationshipEnumerator>;
    fn GetRelationshipsContentStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpcRelationshipSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcRelationshipSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRelationship<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationship(this, ::core::mem::transmute(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationship, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRelationship<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR, relationshiptype: ::windows_core::PCWSTR, targeturi: *mut ::core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRelationship(this, ::core::mem::transmute(&relationshipidentifier), ::core::mem::transmute(&relationshiptype), ::windows_core::from_raw_borrowed(&targeturi), ::core::mem::transmute_copy(&targetmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationship, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRelationship<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRelationship(this, ::core::mem::transmute(&relationshipidentifier)).into())
        }
        unsafe extern "system" fn RelationshipExists<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RelationshipExists(this, ::core::mem::transmute(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipexists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnumeratorForType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshiptype: ::windows_core::PCWSTR, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumeratorForType(this, ::core::mem::transmute(&relationshiptype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipsContentStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcRelationshipSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRelationship: GetRelationship::<Identity, Impl, OFFSET>,
            CreateRelationship: CreateRelationship::<Identity, Impl, OFFSET>,
            DeleteRelationship: DeleteRelationship::<Identity, Impl, OFFSET>,
            RelationshipExists: RelationshipExists::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
            GetEnumeratorForType: GetEnumeratorForType::<Identity, Impl, OFFSET>,
            GetRelationshipsContentStream: GetRelationshipsContentStream::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpcSignatureCustomObject_Impl: ::windows_core::BaseImpl {
    fn GetXml(this: &Self::This, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOpcSignatureCustomObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureCustomObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXml(this, ::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)).into())
        }
        IOpcSignatureCustomObject_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetXml: GetXml::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureCustomObjectEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcSignatureCustomObject>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcSignatureCustomObjectEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureCustomObjectEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureCustomObjectEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpcSignatureCustomObjectSet_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, xmlmarkup: *const u8, count: u32) -> ::windows_core::Result<IOpcSignatureCustomObject>;
    fn Delete(this: &Self::This, customobject: ::core::option::Option<&IOpcSignatureCustomObject>) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl ::windows_core::Iids for IOpcSignatureCustomObjectSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureCustomObjectSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::windows_core::from_raw_borrowed(&customobject)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureCustomObjectSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReference_Impl: ::windows_core::BaseImpl {
    fn GetPartName(this: &Self::This) -> ::windows_core::Result<IOpcPartUri>;
    fn GetContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDigestMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDigestValue(this: &Self::This, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn GetTransformMethod(this: &Self::This) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSignaturePartReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignaturePartReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDigestMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDigestValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDigestValue(this, ::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn GetTransformMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignaturePartReference_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, Impl, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignaturePartReferenceEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcSignaturePartReference>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcSignaturePartReferenceEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignaturePartReferenceEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignaturePartReferenceEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReferenceSet_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, parturi: ::core::option::Option<&IOpcPartUri>, digestmethod: &::windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows_core::Result<IOpcSignaturePartReference>;
    fn Delete(this: &Self::This, partreference: ::core::option::Option<&IOpcSignaturePartReference>) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSignaturePartReferenceSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignaturePartReferenceSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::windows_core::from_raw_borrowed(&parturi), ::core::mem::transmute(&digestmethod), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::windows_core::from_raw_borrowed(&partreference)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignaturePartReferenceSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReference_Impl: ::windows_core::BaseImpl {
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetUri(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetTransformMethod(this: &Self::This) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetDigestMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDigestValue(this: &Self::This, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSignatureReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referenceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referenceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransformMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDigestMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDigestValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDigestValue(this, ::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into())
        }
        IOpcSignatureReference_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureReferenceEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcSignatureReference>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcSignatureReferenceEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureReferenceEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureReferenceEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReferenceSet_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, referenceuri: ::core::option::Option<&super::super::super::System::Com::IUri>, referenceid: &::windows_core::PCWSTR, r#type: &::windows_core::PCWSTR, digestmethod: &::windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows_core::Result<IOpcSignatureReference>;
    fn Delete(this: &Self::This, reference: ::core::option::Option<&IOpcSignatureReference>) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSignatureReferenceSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureReferenceSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceuri: *mut ::core::ffi::c_void, referenceid: ::windows_core::PCWSTR, r#type: ::windows_core::PCWSTR, digestmethod: ::windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::windows_core::from_raw_borrowed(&referenceuri), ::core::mem::transmute(&referenceid), ::core::mem::transmute(&r#type), ::core::mem::transmute(&digestmethod), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::windows_core::from_raw_borrowed(&reference)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureReferenceSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReference_Impl: ::windows_core::BaseImpl {
    fn GetSourceUri(this: &Self::This) -> ::windows_core::Result<IOpcUri>;
    fn GetDigestMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDigestValue(this: &Self::This, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn GetTransformMethod(this: &Self::This) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetRelationshipSigningOption(this: &Self::This) -> ::windows_core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION>;
    fn GetRelationshipSelectorEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSignatureRelationshipReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureRelationshipReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSourceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDigestMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDigestValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDigestValue(this, ::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn GetTransformMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipSigningOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipsigningoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipSelectorEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectorenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureRelationshipReference_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSourceUri: GetSourceUri::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, Impl, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, Impl, OFFSET>,
            GetRelationshipSigningOption: GetRelationshipSigningOption::<Identity, Impl, OFFSET>,
            GetRelationshipSelectorEnumerator: GetRelationshipSelectorEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureRelationshipReferenceEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IOpcSignatureRelationshipReference>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpcSignatureRelationshipReferenceEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureRelationshipReferenceEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MovePrevious(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReferenceSet_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, sourceuri: ::core::option::Option<&IOpcUri>, digestmethod: &::windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::core::option::Option<&IOpcRelationshipSelectorSet>, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows_core::Result<IOpcSignatureRelationshipReference>;
    fn CreateRelationshipSelectorSet(this: &Self::This) -> ::windows_core::Result<IOpcRelationshipSelectorSet>;
    fn Delete(this: &Self::This, relationshipreference: ::core::option::Option<&IOpcSignatureRelationshipReference>) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSignatureRelationshipReferenceSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSignatureRelationshipReferenceSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut ::core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::windows_core::from_raw_borrowed(&sourceuri), ::core::mem::transmute(&digestmethod), ::core::mem::transmute_copy(&relationshipsigningoption), ::windows_core::from_raw_borrowed(&selectorset), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectorset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRelationshipSelectorSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectorset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::windows_core::from_raw_borrowed(&relationshipreference)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcSignatureRelationshipReferenceSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            CreateRelationshipSelectorSet: CreateRelationshipSelectorSet::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSigningOptions_Impl: ::windows_core::BaseImpl {
    fn GetSignatureId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSignatureId(this: &Self::This, signatureid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignatureMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSignatureMethod(this: &Self::This, signaturemethod: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDefaultDigestMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDefaultDigestMethod(this: &Self::This, digestmethod: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCertificateEmbeddingOption(this: &Self::This) -> ::windows_core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION>;
    fn SetCertificateEmbeddingOption(this: &Self::This, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_core::Result<()>;
    fn GetTimeFormat(this: &Self::This) -> ::windows_core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn SetTimeFormat(this: &Self::This, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::Result<()>;
    fn GetSignaturePartReferenceSet(this: &Self::This) -> ::windows_core::Result<IOpcSignaturePartReferenceSet>;
    fn GetSignatureRelationshipReferenceSet(this: &Self::This) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceSet>;
    fn GetCustomObjectSet(this: &Self::This) -> ::windows_core::Result<IOpcSignatureCustomObjectSet>;
    fn GetCustomReferenceSet(this: &Self::This) -> ::windows_core::Result<IOpcSignatureReferenceSet>;
    fn GetCertificateSet(this: &Self::This) -> ::windows_core::Result<IOpcCertificateSet>;
    fn GetSignaturePartName(this: &Self::This) -> ::windows_core::Result<IOpcPartUri>;
    fn SetSignaturePartName(this: &Self::This, signaturepartname: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOpcSigningOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcSigningOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSignatureId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignatureId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureId(this, ::core::mem::transmute(&signatureid)).into())
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureMethod(this, ::core::mem::transmute(&signaturemethod)).into())
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultDigestMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultDigestMethod(this, ::core::mem::transmute(&digestmethod)).into())
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateEmbeddingOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(embeddingoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCertificateEmbeddingOption(this, ::core::mem::transmute_copy(&embeddingoption)).into())
        }
        unsafe extern "system" fn GetTimeFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimeFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTimeFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeFormat(this, ::core::mem::transmute_copy(&timeformat)).into())
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignaturePartReferenceSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreferenceset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureRelationshipReferenceSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreferenceset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomObjectSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomObjectSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomReferenceSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomReferenceSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCertificateSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignaturePartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignaturePartName(this, ::windows_core::from_raw_borrowed(&signaturepartname)).into())
        }
        IOpcSigningOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, Impl, OFFSET>,
            GetDefaultDigestMethod: GetDefaultDigestMethod::<Identity, Impl, OFFSET>,
            SetDefaultDigestMethod: SetDefaultDigestMethod::<Identity, Impl, OFFSET>,
            GetCertificateEmbeddingOption: GetCertificateEmbeddingOption::<Identity, Impl, OFFSET>,
            SetCertificateEmbeddingOption: SetCertificateEmbeddingOption::<Identity, Impl, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, Impl, OFFSET>,
            SetTimeFormat: SetTimeFormat::<Identity, Impl, OFFSET>,
            GetSignaturePartReferenceSet: GetSignaturePartReferenceSet::<Identity, Impl, OFFSET>,
            GetSignatureRelationshipReferenceSet: GetSignatureRelationshipReferenceSet::<Identity, Impl, OFFSET>,
            GetCustomObjectSet: GetCustomObjectSet::<Identity, Impl, OFFSET>,
            GetCustomReferenceSet: GetCustomReferenceSet::<Identity, Impl, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcUri_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IUri_Impl {
    fn GetRelationshipsPartUri(this: &Self::This) -> ::windows_core::Result<IOpcPartUri>;
    fn GetRelativeUri(this: &Self::This, targetparturi: ::core::option::Option<&IOpcPartUri>) -> ::windows_core::Result<super::super::super::System::Com::IUri>;
    fn CombinePartUri(this: &Self::This, relativeuri: ::core::option::Option<&super::super::super::System::Com::IUri>) -> ::windows_core::Result<IOpcPartUri>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpcUri {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IUri);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpcUri {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRelationshipsPartUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipparturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationshipsPartUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipparturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelativeUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetparturi: *mut ::core::ffi::c_void, relativeuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelativeUri(this, ::windows_core::from_raw_borrowed(&targetparturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relativeuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CombinePartUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeuri: *mut ::core::ffi::c_void, combineduri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CombinePartUri(this, ::windows_core::from_raw_borrowed(&relativeuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(combineduri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpcUri_Vtbl {
            base__: <super::super::super::System::Com::IUri as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRelationshipsPartUri: GetRelationshipsPartUri::<Identity, Impl, OFFSET>,
            GetRelativeUri: GetRelativeUri::<Identity, Impl, OFFSET>,
            CombinePartUri: CombinePartUri::<Identity, Impl, OFFSET>,
        }
    };
}
