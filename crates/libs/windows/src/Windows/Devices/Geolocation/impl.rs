pub trait IGeoshape_Impl: ::windows_core::BaseImpl {
    fn GeoshapeType(this: &Self::This) -> ::windows_core::Result<GeoshapeType>;
    fn SpatialReferenceId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn AltitudeReferenceSystem(this: &Self::This) -> ::windows_core::Result<AltitudeReferenceSystem>;
}
impl ::windows_core::Iids for IGeoshape {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGeoshape {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GeoshapeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GeoshapeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpatialReferenceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpatialReferenceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AltitudeReferenceSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGeoshape_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GeoshapeType: GeoshapeType::<Identity, Impl, OFFSET>,
            SpatialReferenceId: SpatialReferenceId::<Identity, Impl, OFFSET>,
            AltitudeReferenceSystem: AltitudeReferenceSystem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
