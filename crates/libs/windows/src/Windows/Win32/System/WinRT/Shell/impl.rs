#[doc = "Required features: `\"Win32_UI_Shell\"`"]
#[cfg(feature = "Win32_UI_Shell")]
pub trait IDDEInitializer_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, fileextensionorprotocol: &::windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: &::windows_core::PCWSTR, exectarget: ::core::option::Option<&super::super::super::UI::Shell::IShellItem>, site: ::core::option::Option<&::windows_core::IUnknown>, application: &::windows_core::PCWSTR, targetfile: &::windows_core::PCWSTR, arguments: &::windows_core::PCWSTR, verb: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell")]
impl ::windows_core::Iids for IDDEInitializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDEInitializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDDEInitializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDEInitializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows_core::PCWSTR, exectarget: *mut ::core::ffi::c_void, site: *mut ::core::ffi::c_void, application: ::windows_core::PCWSTR, targetfile: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, verb: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&fileextensionorprotocol), ::core::mem::transmute_copy(&method), ::core::mem::transmute(&currentdirectory), ::windows_core::from_raw_borrowed(&exectarget), ::windows_core::from_raw_borrowed(&site), ::core::mem::transmute(&application), ::core::mem::transmute(&targetfile), ::core::mem::transmute(&arguments), ::core::mem::transmute(&verb)).into())
        }
        IDDEInitializer_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
