pub trait IAudioFrameNative_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioFrameNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFrameNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioFrameNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFrameNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IAudioFrameNative_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetData: GetData::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IAudioFrameNativeFactory_Impl: ::windows_core::BaseImpl {
    fn CreateFromMFSample(this: &Self::This, data: ::core::option::Option<&super::super::super::Media::MediaFoundation::IMFSample>, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IAudioFrameNativeFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFrameNativeFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioFrameNativeFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromMFSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFrameNativeFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromMFSample(this, ::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&forcereadonly), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IAudioFrameNativeFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromMFSample: CreateFromMFSample::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVideoFrameNative_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDevice(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVideoFrameNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoFrameNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVideoFrameNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoFrameNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoFrameNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IVideoFrameNative_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IVideoFrameNativeFactory_Impl: ::windows_core::BaseImpl {
    fn CreateFromMFSample(this: &Self::This, data: ::core::option::Option<&super::super::super::Media::MediaFoundation::IMFSample>, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: ::core::option::Option<&super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IVideoFrameNativeFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoFrameNativeFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVideoFrameNativeFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromMFSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoFrameNativeFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromMFSample(this, ::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&subtype), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&forcereadonly), ::core::mem::transmute_copy(&mindisplayaperture), ::windows_core::from_raw_borrowed(&device), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IVideoFrameNativeFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromMFSample: CreateFromMFSample::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
