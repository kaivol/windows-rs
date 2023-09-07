#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaEncodingProperties_Impl: ::windows_core::BaseImpl {
    fn Properties(this: &Self::This) -> ::windows_core::Result<MediaPropertySet>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetSubtype(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Subtype(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IMediaEncodingProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaEncodingProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubtype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubtype(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Subtype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subtype(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaEncodingProperties_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetSubtype: SetSubtype::<Identity, Impl, OFFSET>,
            Subtype: Subtype::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
