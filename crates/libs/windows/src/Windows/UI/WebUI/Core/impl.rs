pub trait IWebUICommandBarElement_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IWebUICommandBarElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUICommandBarElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebUICommandBarElement {
    const VTABLE: Self::Vtable = { IWebUICommandBarElement_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IWebUICommandBarIcon_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IWebUICommandBarIcon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUICommandBarIcon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebUICommandBarIcon {
    const VTABLE: Self::Vtable = { IWebUICommandBarIcon_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
