#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IDirect3DDevice_Impl: ::windows_core::BaseImpl + super::super::super::Foundation::IClosable_Impl {
    fn Trim(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IDirect3DDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Trim<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Trim(this).into())
        }
        IDirect3DDevice_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Trim: Trim::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IDirect3DSurface_Impl: ::windows_core::BaseImpl + super::super::super::Foundation::IClosable_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<Direct3DSurfaceDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IDirect3DSurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DSurface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirect3DSurface_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Description: Description::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
