#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode_Impl: ::windows_core::BaseImpl + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn OutgoingConnections(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>;
    fn AddOutgoingConnection(this: &Self::This, destination: ::core::option::Option<&IAudioNode>) -> ::windows_core::Result<()>;
    fn AddOutgoingConnectionWithGain(this: &Self::This, destination: ::core::option::Option<&IAudioNode>, gain: f64) -> ::windows_core::Result<()>;
    fn RemoveOutgoingConnection(this: &Self::This, destination: ::core::option::Option<&IAudioNode>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IAudioInputNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAudioNode as ::windows_core::ComInterface>::IID, <super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioInputNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OutgoingConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddOutgoingConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOutgoingConnection(this, ::windows_core::from_raw_borrowed(&destination)).into())
        }
        unsafe extern "system" fn AddOutgoingConnectionWithGain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, gain: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOutgoingConnectionWithGain(this, ::windows_core::from_raw_borrowed(&destination), gain).into())
        }
        unsafe extern "system" fn RemoveOutgoingConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveOutgoingConnection(this, ::windows_core::from_raw_borrowed(&destination)).into())
        }
        IAudioInputNode_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OutgoingConnections: OutgoingConnections::<Identity, Impl, OFFSET>,
            AddOutgoingConnection: AddOutgoingConnection::<Identity, Impl, OFFSET>,
            AddOutgoingConnectionWithGain: AddOutgoingConnectionWithGain::<Identity, Impl, OFFSET>,
            RemoveOutgoingConnection: RemoveOutgoingConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode2_Impl: ::windows_core::BaseImpl + IAudioInputNode_Impl + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn Emitter(this: &Self::This) -> ::windows_core::Result<AudioNodeEmitter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IAudioInputNode2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAudioInputNode as ::windows_core::ComInterface>::IID, <IAudioNode as ::windows_core::ComInterface>::IID, <super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioInputNode2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Emitter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputNode2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Emitter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioInputNode2_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Emitter: Emitter::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNode_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl {
    fn EffectDefinitions(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn SetOutgoingGain(this: &Self::This, value: f64) -> ::windows_core::Result<()>;
    fn OutgoingGain(this: &Self::This) -> ::windows_core::Result<f64>;
    fn EncodingProperties(this: &Self::This) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn ConsumeInput(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetConsumeInput(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableEffectsByDefinition(this: &Self::This, definition: ::core::option::Option<&super::Effects::IAudioEffectDefinition>) -> ::windows_core::Result<()>;
    fn EnableEffectsByDefinition(this: &Self::This, definition: ::core::option::Option<&super::Effects::IAudioEffectDefinition>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IAudioNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EffectDefinitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EffectDefinitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutgoingGain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutgoingGain(this, value).into())
        }
        unsafe extern "system" fn OutgoingGain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutgoingGain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EncodingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EncodingProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConsumeInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConsumeInput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConsumeInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConsumeInput(this, value).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn DisableEffectsByDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableEffectsByDefinition(this, ::windows_core::from_raw_borrowed(&definition)).into())
        }
        unsafe extern "system" fn EnableEffectsByDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableEffectsByDefinition(this, ::windows_core::from_raw_borrowed(&definition)).into())
        }
        IAudioNode_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EffectDefinitions: EffectDefinitions::<Identity, Impl, OFFSET>,
            SetOutgoingGain: SetOutgoingGain::<Identity, Impl, OFFSET>,
            OutgoingGain: OutgoingGain::<Identity, Impl, OFFSET>,
            EncodingProperties: EncodingProperties::<Identity, Impl, OFFSET>,
            ConsumeInput: ConsumeInput::<Identity, Impl, OFFSET>,
            SetConsumeInput: SetConsumeInput::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            DisableEffectsByDefinition: DisableEffectsByDefinition::<Identity, Impl, OFFSET>,
            EnableEffectsByDefinition: EnableEffectsByDefinition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`, `\"Media_MediaProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNodeWithListener_Impl: ::windows_core::BaseImpl + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn SetListener(this: &Self::This, value: ::core::option::Option<&AudioNodeListener>) -> ::windows_core::Result<()>;
    fn Listener(this: &Self::This) -> ::windows_core::Result<AudioNodeListener>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows_core::Iids for IAudioNodeWithListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAudioNode as ::windows_core::ComInterface>::IID, <super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioNodeWithListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListener(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Listener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Listener(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioNodeWithListener_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetListener: SetListener::<Identity, Impl, OFFSET>,
            Listener: Listener::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
