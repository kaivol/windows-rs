pub trait ICcgDomainAuthCredentials_Impl: ::windows_core::BaseImpl {
    fn GetPasswordCredentials(this: &Self::This, plugininput: &::windows_core::PCWSTR, domainname: *mut ::windows_core::PWSTR, username: *mut ::windows_core::PWSTR, password: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICcgDomainAuthCredentials {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICcgDomainAuthCredentials_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICcgDomainAuthCredentials {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPasswordCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICcgDomainAuthCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plugininput: ::windows_core::PCWSTR, domainname: *mut ::windows_core::PWSTR, username: *mut ::windows_core::PWSTR, password: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPasswordCredentials(this, ::core::mem::transmute(&plugininput), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into())
        }
        ICcgDomainAuthCredentials_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPasswordCredentials: GetPasswordCredentials::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
