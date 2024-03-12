pub trait ISensorDataThreshold_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ISensorDataThreshold {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorDataThreshold_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensorDataThreshold {
    const VTABLE: Self::Vtable = { ISensorDataThreshold_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
