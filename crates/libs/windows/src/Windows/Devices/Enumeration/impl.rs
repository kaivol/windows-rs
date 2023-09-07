pub trait IDevicePairingSettings_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDevicePairingSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDevicePairingSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDevicePairingSettings {
    const VTABLE: Self::Vtable = { IDevicePairingSettings_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
