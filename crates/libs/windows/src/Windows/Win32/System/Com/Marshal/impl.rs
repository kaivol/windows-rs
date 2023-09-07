pub trait IMarshal_Impl: ::windows_core::BaseImpl {
    fn GetUnmarshalClass(this: &Self::This, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetMarshalSizeMax(this: &Self::This, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<u32>;
    fn MarshalInterface(this: &Self::This, pstm: ::core::option::Option<&super::IStream>, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<()>;
    fn UnmarshalInterface(this: &Self::This, pstm: ::core::option::Option<&super::IStream>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReleaseMarshalData(this: &Self::This, pstm: ::core::option::Option<&super::IStream>) -> ::windows_core::Result<()>;
    fn DisconnectObject(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMarshal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMarshal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUnmarshalClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnmarshalClass(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMarshalSizeMax(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MarshalInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MarshalInterface(this, ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)).into())
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnmarshalInterface(this, ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMarshalData(this, ::windows_core::from_raw_borrowed(&pstm)).into())
        }
        unsafe extern "system" fn DisconnectObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectObject(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IMarshal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUnmarshalClass: GetUnmarshalClass::<Identity, Impl, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, Impl, OFFSET>,
            MarshalInterface: MarshalInterface::<Identity, Impl, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
            DisconnectObject: DisconnectObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMarshal2_Impl: ::windows_core::BaseImpl + IMarshal_Impl {}
impl ::windows_core::Iids for IMarshal2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMarshal);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshal2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMarshal2 {
    const VTABLE: Self::Vtable = { IMarshal2_Vtbl { base__: <IMarshal as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMarshalingStream_Impl: ::windows_core::BaseImpl + super::IStream_Impl {
    fn GetMarshalingContextAttribute(this: &Self::This, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows_core::Result<usize>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMarshalingStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IStream);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshalingStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMarshalingStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMarshalingContextAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMarshalingStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMarshalingContextAttribute(this, ::core::mem::transmute_copy(&attribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pattributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMarshalingStream_Vtbl {
            base__: <super::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMarshalingContextAttribute: GetMarshalingContextAttribute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
