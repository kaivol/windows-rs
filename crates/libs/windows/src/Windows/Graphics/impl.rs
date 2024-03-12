pub trait IGeometrySource2D_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IGeometrySource2D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeometrySource2D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGeometrySource2D {
    const VTABLE: Self::Vtable = { IGeometrySource2D_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
