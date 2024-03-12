#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaExtension_Impl: ::windows_core::BaseImpl {
    fn SetProperties(this: &Self::This, configuration: ::core::option::Option<&super::Foundation::Collections::IPropertySet>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IMediaExtension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaExtension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaExtension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperties(this, ::windows_core::from_raw_borrowed(&configuration)).into())
        }
        IMediaExtension_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaFrame_Impl: ::windows_core::BaseImpl + super::Foundation::IClosable_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn IsReadOnly(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetRelativeTime(this: &Self::This, value: ::core::option::Option<&super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows_core::Result<()>;
    fn RelativeTime(this: &Self::This) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetSystemRelativeTime(this: &Self::This, value: ::core::option::Option<&super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows_core::Result<()>;
    fn SystemRelativeTime(this: &Self::This) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(this: &Self::This, value: ::core::option::Option<&super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows_core::Result<()>;
    fn Duration(this: &Self::This) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetIsDiscontinuous(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn IsDiscontinuous(this: &Self::This) -> ::windows_core::Result<bool>;
    fn ExtendedProperties(this: &Self::This) -> ::windows_core::Result<super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IMediaFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRelativeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRelativeTime(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn RelativeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RelativeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSystemRelativeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSystemRelativeTime(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SystemRelativeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SystemRelativeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuration(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Duration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Duration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsDiscontinuous<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsDiscontinuous(this, value).into())
        }
        unsafe extern "system" fn IsDiscontinuous<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDiscontinuous(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaFrame_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            SetRelativeTime: SetRelativeTime::<Identity, Impl, OFFSET>,
            RelativeTime: RelativeTime::<Identity, Impl, OFFSET>,
            SetSystemRelativeTime: SetSystemRelativeTime::<Identity, Impl, OFFSET>,
            SystemRelativeTime: SystemRelativeTime::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetIsDiscontinuous: SetIsDiscontinuous::<Identity, Impl, OFFSET>,
            IsDiscontinuous: IsDiscontinuous::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IMediaMarker_Impl: ::windows_core::BaseImpl {
    fn Time(this: &Self::This) -> ::windows_core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Text(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IMediaMarker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaMarker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaMarker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Time<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaMarker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Time(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaMarkerType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaMarker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaMarkerType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaMarker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaMarker_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Time: Time::<Identity, Impl, OFFSET>,
            MediaMarkerType: MediaMarkerType::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaMarkers_Impl: ::windows_core::BaseImpl {
    fn Markers(this: &Self::This) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IMediaMarkers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaMarkers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaMarkers {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Markers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaMarkers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Markers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaMarkers_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Markers: Markers::<Identity, Impl, OFFSET> }
    };
}
