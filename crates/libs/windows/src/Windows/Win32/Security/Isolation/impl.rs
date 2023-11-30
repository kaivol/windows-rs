#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedAppLauncher_Impl: ::windows_core::BaseImpl {
    fn Launch(this: &Self::This, appusermodelid: &::windows_core::PCWSTR, arguments: &::windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IIsolatedAppLauncher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsolatedAppLauncher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIsolatedAppLauncher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Launch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsolatedAppLauncher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appusermodelid: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Launch(this, ::core::mem::transmute(&appusermodelid), ::core::mem::transmute(&arguments), ::core::mem::transmute_copy(&telemetryparameters)).into())
        }
        IIsolatedAppLauncher_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Launch: Launch::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
