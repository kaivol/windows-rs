#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInterop_Impl: ::windows_core::BaseImpl {
    fn RequestAccessForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProtectionPolicyManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtectionPolicyManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestAccessForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&targetidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into())
        }
        IProtectionPolicyManagerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestAccessForWindowAsync: RequestAccessForWindowAsync::<Identity, Impl, OFFSET>,
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInterop2_Impl: ::windows_core::BaseImpl {
    fn RequestAccessForAppWithWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessWithAuditingInfoForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessWithMessageForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, messagefromapp: &::windows_core::HSTRING, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessForAppWithAuditingInfoForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessForAppWithMessageForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, messagefromapp: &::windows_core::HSTRING, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProtectionPolicyManagerInterop2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtectionPolicyManagerInterop2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestAccessForAppWithWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessForAppWithWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&apppackagefamilyname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessWithAuditingInfoForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessWithAuditingInfoForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&targetidentity), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessWithMessageForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessWithMessageForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&targetidentity), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute(&messagefromapp), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessForAppWithAuditingInfoForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessForAppWithAuditingInfoForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&apppackagefamilyname), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessForAppWithMessageForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessForAppWithMessageForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&apppackagefamilyname), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute(&messagefromapp), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        IProtectionPolicyManagerInterop2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestAccessForAppWithWindowAsync: RequestAccessForAppWithWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessWithAuditingInfoForWindowAsync: RequestAccessWithAuditingInfoForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessWithMessageForWindowAsync: RequestAccessWithMessageForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessForAppWithAuditingInfoForWindowAsync: RequestAccessForAppWithAuditingInfoForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessForAppWithMessageForWindowAsync: RequestAccessForAppWithMessageForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInterop3_Impl: ::windows_core::BaseImpl {
    fn RequestAccessWithBehaviorForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, messagefromapp: &::windows_core::HSTRING, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessForAppWithBehaviorForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, messagefromapp: &::windows_core::HSTRING, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessToFilesForAppForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<&::windows_core::IUnknown>, apppackagefamilyname: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<&::windows_core::IUnknown>, apppackagefamilyname: &::windows_core::HSTRING, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, messagefromapp: &::windows_core::HSTRING, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessToFilesForProcessForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<&::windows_core::IUnknown>, processid: u32, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<&::windows_core::IUnknown>, processid: u32, auditinfounk: ::core::option::Option<&::windows_core::IUnknown>, messagefromapp: &::windows_core::HSTRING, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProtectionPolicyManagerInterop3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtectionPolicyManagerInterop3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestAccessWithBehaviorForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessWithBehaviorForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&targetidentity), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessForAppWithBehaviorForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessForAppWithBehaviorForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceidentity), ::core::mem::transmute(&apppackagefamilyname), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessToFilesForAppForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessToFilesForAppForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::windows_core::from_raw_borrowed(&sourceitemlistunk), ::core::mem::transmute(&apppackagefamilyname), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::windows_core::from_raw_borrowed(&sourceitemlistunk), ::core::mem::transmute(&apppackagefamilyname), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessToFilesForProcessForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::windows_core::from_raw_borrowed(&sourceitemlistunk), ::core::mem::transmute_copy(&processid), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::windows_core::from_raw_borrowed(&sourceitemlistunk), ::core::mem::transmute_copy(&processid), ::windows_core::from_raw_borrowed(&auditinfounk), ::core::mem::transmute(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        IProtectionPolicyManagerInterop3_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestAccessWithBehaviorForWindowAsync: RequestAccessWithBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessForAppWithBehaviorForWindowAsync: RequestAccessForAppWithBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForAppForWindowAsync: RequestAccessToFilesForAppForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForProcessForWindowAsync: RequestAccessToFilesForProcessForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
}
