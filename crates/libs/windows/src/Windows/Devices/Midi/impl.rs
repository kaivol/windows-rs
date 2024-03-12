#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiMessage_Impl: ::windows_core::BaseImpl {
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(this: &Self::This) -> ::windows_core::Result<MidiMessageType>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IMidiMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMidiMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RawData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RawData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMidiMessage_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            RawData: RawData::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiOutPort_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl {
    fn SendMessage(this: &Self::This, midimessage: ::core::option::Option<&IMidiMessage>) -> ::windows_core::Result<()>;
    fn SendBuffer(this: &Self::This, mididata: ::core::option::Option<&super::super::Storage::Streams::IBuffer>) -> ::windows_core::Result<()>;
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IMidiOutPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMidiOutPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, midimessage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMessage(this, ::windows_core::from_raw_borrowed(&midimessage)).into())
        }
        unsafe extern "system" fn SendBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mididata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendBuffer(this, ::windows_core::from_raw_borrowed(&mididata)).into())
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMidiOutPort_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            SendBuffer: SendBuffer::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
        }
    };
}
