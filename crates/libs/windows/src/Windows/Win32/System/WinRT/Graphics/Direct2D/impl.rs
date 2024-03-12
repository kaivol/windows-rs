#[doc = "Required features: `\"Win32_Graphics_Direct2D\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D")]
pub trait IGeometrySource2DInterop_Impl: ::windows_core::BaseImpl {
    fn GetGeometry(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
    fn TryGetGeometryUsingFactory(this: &Self::This, factory: ::core::option::Option<&super::super::super::super::Graphics::Direct2D::ID2D1Factory>) -> ::windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl ::windows_core::Iids for IGeometrySource2DInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeometrySource2DInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGeometrySource2DInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeometrySource2DInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeometry(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGeometrySource2DInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryGetGeometryUsingFactory(this, ::windows_core::from_raw_borrowed(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGeometrySource2DInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGeometry: GetGeometry::<Identity, Impl, OFFSET>,
            TryGetGeometryUsingFactory: TryGetGeometryUsingFactory::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Graphics_Effects\"`"]
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
pub trait IGraphicsEffectD2D1Interop_Impl: ::windows_core::BaseImpl {
    fn GetEffectId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetNamedPropertyMapping(this: &Self::This, name: &::windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_core::Result<()>;
    fn GetPropertyCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetProperty(this: &Self::This, index: u32) -> ::windows_core::Result<super::super::super::super::super::Foundation::IPropertyValue>;
    fn GetSource(this: &Self::This, index: u32) -> ::windows_core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn GetSourceCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
impl ::windows_core::Iids for IGraphicsEffectD2D1Interop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGraphicsEffectD2D1Interop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEffectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEffectId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamedPropertyMapping(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&mapping)).into())
        }
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, source: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSource(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(source, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGraphicsEffectD2D1Interop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEffectId: GetEffectId::<Identity, Impl, OFFSET>,
            GetNamedPropertyMapping: GetNamedPropertyMapping::<Identity, Impl, OFFSET>,
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, Impl, OFFSET>,
        }
    };
}
