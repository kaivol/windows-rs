pub trait IDefaultAudioDeviceChangedEventArgs_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Role(this: &Self::This) -> ::windows_core::Result<AudioDeviceRole>;
}
impl ::windows_core::Iids for IDefaultAudioDeviceChangedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDefaultAudioDeviceChangedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Role<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceRole) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Role(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDefaultAudioDeviceChangedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
pub trait IMediaDeviceController_Impl: ::windows_core::BaseImpl {
    fn GetAvailableMediaStreamProperties(this: &Self::This, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>;
    fn GetMediaStreamProperties(this: &Self::This, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::MediaProperties::IMediaEncodingProperties>;
    fn SetMediaStreamPropertiesAsync(this: &Self::This, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: ::core::option::Option<&super::MediaProperties::IMediaEncodingProperties>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IMediaDeviceController {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaDeviceController {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAvailableMediaStreamProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableMediaStreamProperties(this, mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMediaStreamProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMediaStreamProperties(this, mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMediaStreamPropertiesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetMediaStreamPropertiesAsync(this, mediastreamtype, ::windows_core::from_raw_borrowed(&mediaencodingproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaDeviceController_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAvailableMediaStreamProperties: GetAvailableMediaStreamProperties::<Identity, Impl, OFFSET>,
            GetMediaStreamProperties: GetMediaStreamProperties::<Identity, Impl, OFFSET>,
            SetMediaStreamPropertiesAsync: SetMediaStreamPropertiesAsync::<Identity, Impl, OFFSET>,
        }
    };
}
