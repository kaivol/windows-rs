#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IAudioEffectDefinition_Impl: ::windows_core::BaseImpl {
    fn ActivatableClassId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IAudioEffectDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEffectDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivatableClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivatableClassId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioEffectDefinition_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivatableClassId: ActivatableClassId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
pub trait IBasicAudioEffect_Impl: ::windows_core::BaseImpl + super::IMediaExtension_Impl {
    fn UseInputFrameForOutput(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SupportedEncodingProperties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>;
    fn SetEncodingProperties(this: &Self::This, encodingproperties: ::core::option::Option<&super::MediaProperties::AudioEncodingProperties>) -> ::windows_core::Result<()>;
    fn ProcessFrame(this: &Self::This, context: ::core::option::Option<&ProcessAudioFrameContext>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, reason: MediaEffectClosedReason) -> ::windows_core::Result<()>;
    fn DiscardQueuedFrames(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IBasicAudioEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IMediaExtension as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBasicAudioEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UseInputFrameForOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseInputFrameForOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedEncodingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedEncodingProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncodingProperties(this, ::windows_core::from_raw_borrowed(&encodingproperties)).into())
        }
        unsafe extern "system" fn ProcessFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessFrame(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, reason).into())
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicAudioEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardQueuedFrames(this).into())
        }
        IBasicAudioEffect_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UseInputFrameForOutput: UseInputFrameForOutput::<Identity, Impl, OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Identity, Impl, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, Impl, OFFSET>,
            ProcessFrame: ProcessFrame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX_Direct3D11\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IBasicVideoEffect_Impl: ::windows_core::BaseImpl + super::IMediaExtension_Impl {
    fn IsReadOnly(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SupportedMemoryTypes(this: &Self::This) -> ::windows_core::Result<MediaMemoryTypes>;
    fn TimeIndependent(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SupportedEncodingProperties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>;
    fn SetEncodingProperties(this: &Self::This, encodingproperties: ::core::option::Option<&super::MediaProperties::VideoEncodingProperties>, device: ::core::option::Option<&super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows_core::Result<()>;
    fn ProcessFrame(this: &Self::This, context: ::core::option::Option<&ProcessVideoFrameContext>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, reason: MediaEffectClosedReason) -> ::windows_core::Result<()>;
    fn DiscardQueuedFrames(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IBasicVideoEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IMediaExtension as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBasicVideoEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedMemoryTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedMemoryTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TimeIndependent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimeIndependent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedEncodingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedEncodingProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncodingProperties(this, ::windows_core::from_raw_borrowed(&encodingproperties), ::windows_core::from_raw_borrowed(&device)).into())
        }
        unsafe extern "system" fn ProcessFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessFrame(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, reason).into())
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBasicVideoEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardQueuedFrames(this).into())
        }
        IBasicVideoEffect_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            SupportedMemoryTypes: SupportedMemoryTypes::<Identity, Impl, OFFSET>,
            TimeIndependent: TimeIndependent::<Identity, Impl, OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Identity, Impl, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, Impl, OFFSET>,
            ProcessFrame: ProcessFrame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX_Direct3D11\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IVideoCompositor_Impl: ::windows_core::BaseImpl + super::IMediaExtension_Impl {
    fn TimeIndependent(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetEncodingProperties(this: &Self::This, backgroundproperties: ::core::option::Option<&super::MediaProperties::VideoEncodingProperties>, device: ::core::option::Option<&super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows_core::Result<()>;
    fn CompositeFrame(this: &Self::This, context: ::core::option::Option<&CompositeVideoFrameContext>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, reason: MediaEffectClosedReason) -> ::windows_core::Result<()>;
    fn DiscardQueuedFrames(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IVideoCompositor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IMediaExtension as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVideoCompositor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TimeIndependent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimeIndependent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, backgroundproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncodingProperties(this, ::windows_core::from_raw_borrowed(&backgroundproperties), ::windows_core::from_raw_borrowed(&device)).into())
        }
        unsafe extern "system" fn CompositeFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompositeFrame(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, reason).into())
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardQueuedFrames(this).into())
        }
        IVideoCompositor_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TimeIndependent: TimeIndependent::<Identity, Impl, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, Impl, OFFSET>,
            CompositeFrame: CompositeFrame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoCompositorDefinition_Impl: ::windows_core::BaseImpl {
    fn ActivatableClassId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IVideoCompositorDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositorDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVideoCompositorDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivatableClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositorDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivatableClassId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoCompositorDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVideoCompositorDefinition_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivatableClassId: ActivatableClassId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoEffectDefinition_Impl: ::windows_core::BaseImpl {
    fn ActivatableClassId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IVideoEffectDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoEffectDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVideoEffectDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivatableClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoEffectDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivatableClassId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVideoEffectDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVideoEffectDefinition_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivatableClassId: ActivatableClassId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
