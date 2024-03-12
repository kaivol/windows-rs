pub trait IKsAggregateControl_Impl: ::windows_core::BaseImpl {
    fn KsAddAggregate(this: &Self::This, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn KsRemoveAggregate(this: &Self::This, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IKsAggregateControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsAggregateControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsAddAggregate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsAddAggregate(this, ::core::mem::transmute_copy(&aggregateclass)).into())
        }
        unsafe extern "system" fn KsRemoveAggregate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsRemoveAggregate(this, ::core::mem::transmute_copy(&aggregateclass)).into())
        }
        IKsAggregateControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsAddAggregate: KsAddAggregate::<Identity, Impl, OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsAllocator_Impl: ::windows_core::BaseImpl {
    fn KsGetAllocatorHandle(this: &Self::This) -> super::super::Foundation::HANDLE;
    fn KsGetAllocatorMode(this: &Self::This) -> KSALLOCATORMODE;
    fn KsGetAllocatorStatus(this: &Self::This, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> ::windows_core::Result<()>;
    fn KsSetAllocatorMode(this: &Self::This, mode: KSALLOCATORMODE);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsAllocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsAllocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsGetAllocatorHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetAllocatorHandle(this))
        }
        unsafe extern "system" fn KsGetAllocatorMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> KSALLOCATORMODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetAllocatorMode(this))
        }
        unsafe extern "system" fn KsGetAllocatorStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetAllocatorStatus(this, ::core::mem::transmute_copy(&allocatorstatus)).into())
        }
        unsafe extern "system" fn KsSetAllocatorMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: KSALLOCATORMODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetAllocatorMode(this, ::core::mem::transmute_copy(&mode)))
        }
        IKsAllocator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsGetAllocatorHandle: KsGetAllocatorHandle::<Identity, Impl, OFFSET>,
            KsGetAllocatorMode: KsGetAllocatorMode::<Identity, Impl, OFFSET>,
            KsGetAllocatorStatus: KsGetAllocatorStatus::<Identity, Impl, OFFSET>,
            KsSetAllocatorMode: KsSetAllocatorMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsAllocatorEx_Impl: ::windows_core::BaseImpl + IKsAllocator_Impl {
    fn KsGetProperties(this: &Self::This) -> *mut ALLOCATOR_PROPERTIES_EX;
    fn KsSetProperties(this: &Self::This, param0: *const ALLOCATOR_PROPERTIES_EX);
    fn KsSetAllocatorHandle(this: &Self::This, allocatorhandle: super::super::Foundation::HANDLE);
    fn KsCreateAllocatorAndGetHandle(this: &Self::This, kspin: ::core::option::Option<&IKsPin>) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsAllocatorEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IKsAllocator);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsAllocatorEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsGetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ALLOCATOR_PROPERTIES_EX {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetProperties(this))
        }
        unsafe extern "system" fn KsSetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ALLOCATOR_PROPERTIES_EX) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetProperties(this, ::core::mem::transmute_copy(&param0)))
        }
        unsafe extern "system" fn KsSetAllocatorHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allocatorhandle: super::super::Foundation::HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetAllocatorHandle(this, ::core::mem::transmute_copy(&allocatorhandle)))
        }
        unsafe extern "system" fn KsCreateAllocatorAndGetHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kspin: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsCreateAllocatorAndGetHandle(this, ::windows_core::from_raw_borrowed(&kspin)))
        }
        IKsAllocatorEx_Vtbl {
            base__: <IKsAllocator as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsGetProperties: KsGetProperties::<Identity, Impl, OFFSET>,
            KsSetProperties: KsSetProperties::<Identity, Impl, OFFSET>,
            KsSetAllocatorHandle: KsSetAllocatorHandle::<Identity, Impl, OFFSET>,
            KsCreateAllocatorAndGetHandle: KsCreateAllocatorAndGetHandle::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsClockPropertySet_Impl: ::windows_core::BaseImpl {
    fn KsGetTime(this: &Self::This) -> ::windows_core::Result<i64>;
    fn KsSetTime(this: &Self::This, time: i64) -> ::windows_core::Result<()>;
    fn KsGetPhysicalTime(this: &Self::This) -> ::windows_core::Result<i64>;
    fn KsSetPhysicalTime(this: &Self::This, time: i64) -> ::windows_core::Result<()>;
    fn KsGetCorrelatedTime(this: &Self::This) -> ::windows_core::Result<KSCORRELATED_TIME>;
    fn KsSetCorrelatedTime(this: &Self::This, correlatedtime: *const KSCORRELATED_TIME) -> ::windows_core::Result<()>;
    fn KsGetCorrelatedPhysicalTime(this: &Self::This) -> ::windows_core::Result<KSCORRELATED_TIME>;
    fn KsSetCorrelatedPhysicalTime(this: &Self::This, correlatedtime: *const KSCORRELATED_TIME) -> ::windows_core::Result<()>;
    fn KsGetResolution(this: &Self::This) -> ::windows_core::Result<KSRESOLUTION>;
    fn KsGetState(this: &Self::This) -> ::windows_core::Result<KSSTATE>;
}
impl ::windows_core::Iids for IKsClockPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsClockPropertySet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsGetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsGetTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(time, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsSetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetTime(this, ::core::mem::transmute_copy(&time)).into())
        }
        unsafe extern "system" fn KsGetPhysicalTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsGetPhysicalTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(time, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsSetPhysicalTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetPhysicalTime(this, ::core::mem::transmute_copy(&time)).into())
        }
        unsafe extern "system" fn KsGetCorrelatedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, correlatedtime: *mut KSCORRELATED_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsGetCorrelatedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(correlatedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsSetCorrelatedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, correlatedtime: *const KSCORRELATED_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetCorrelatedTime(this, ::core::mem::transmute_copy(&correlatedtime)).into())
        }
        unsafe extern "system" fn KsGetCorrelatedPhysicalTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, correlatedtime: *mut KSCORRELATED_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsGetCorrelatedPhysicalTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(correlatedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsSetCorrelatedPhysicalTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, correlatedtime: *const KSCORRELATED_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetCorrelatedPhysicalTime(this, ::core::mem::transmute_copy(&correlatedtime)).into())
        }
        unsafe extern "system" fn KsGetResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolution: *mut KSRESOLUTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsGetResolution(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resolution, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsGetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut KSSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsGetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsClockPropertySet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsGetTime: KsGetTime::<Identity, Impl, OFFSET>,
            KsSetTime: KsSetTime::<Identity, Impl, OFFSET>,
            KsGetPhysicalTime: KsGetPhysicalTime::<Identity, Impl, OFFSET>,
            KsSetPhysicalTime: KsSetPhysicalTime::<Identity, Impl, OFFSET>,
            KsGetCorrelatedTime: KsGetCorrelatedTime::<Identity, Impl, OFFSET>,
            KsSetCorrelatedTime: KsSetCorrelatedTime::<Identity, Impl, OFFSET>,
            KsGetCorrelatedPhysicalTime: KsGetCorrelatedPhysicalTime::<Identity, Impl, OFFSET>,
            KsSetCorrelatedPhysicalTime: KsSetCorrelatedPhysicalTime::<Identity, Impl, OFFSET>,
            KsGetResolution: KsGetResolution::<Identity, Impl, OFFSET>,
            KsGetState: KsGetState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsControl_Impl: ::windows_core::BaseImpl {
    fn KsProperty(this: &Self::This, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn KsMethod(this: &Self::This, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn KsEvent(this: &Self::This, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IKsControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsProperty(this, ::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&propertylength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into())
        }
        unsafe extern "system" fn KsMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsMethod(this, ::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&methodlength), ::core::mem::transmute_copy(&methoddata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into())
        }
        unsafe extern "system" fn KsEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsEvent(this, ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&eventlength), ::core::mem::transmute_copy(&eventdata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into())
        }
        IKsControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsProperty: KsProperty::<Identity, Impl, OFFSET>,
            KsMethod: KsMethod::<Identity, Impl, OFFSET>,
            KsEvent: KsEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IKsDataTypeCompletion_Impl: ::windows_core::BaseImpl {
    fn KsCompleteMediaType(this: &Self::This, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IKsDataTypeCompletion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeCompletion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsDataTypeCompletion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsCompleteMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeCompletion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsCompleteMediaType(this, ::core::mem::transmute_copy(&filterhandle), ::core::mem::transmute_copy(&pinfactoryid), ::core::mem::transmute_copy(&ammediatype)).into())
        }
        IKsDataTypeCompletion_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsCompleteMediaType: KsCompleteMediaType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
pub trait IKsDataTypeHandler_Impl: ::windows_core::BaseImpl {
    fn KsCompleteIoOperation(this: &Self::This, sample: ::core::option::Option<&super::DirectShow::IMediaSample>, streamheader: *mut ::core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn KsIsMediaTypeInRanges(this: &Self::This, dataranges: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn KsPrepareIoOperation(this: &Self::This, sample: ::core::option::Option<&super::DirectShow::IMediaSample>, streamheader: *mut ::core::ffi::c_void, iooperation: KSIOOPERATION) -> ::windows_core::Result<()>;
    fn KsQueryExtendedSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn KsSetMediaType(this: &Self::This, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IKsDataTypeHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsDataTypeHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsCompleteIoOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sample: *mut ::core::ffi::c_void, streamheader: *mut ::core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsCompleteIoOperation(this, ::windows_core::from_raw_borrowed(&sample), ::core::mem::transmute_copy(&streamheader), ::core::mem::transmute_copy(&iooperation), ::core::mem::transmute_copy(&cancelled)).into())
        }
        unsafe extern "system" fn KsIsMediaTypeInRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dataranges: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsIsMediaTypeInRanges(this, ::core::mem::transmute_copy(&dataranges)).into())
        }
        unsafe extern "system" fn KsPrepareIoOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sample: *mut ::core::ffi::c_void, streamheader: *mut ::core::ffi::c_void, iooperation: KSIOOPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsPrepareIoOperation(this, ::windows_core::from_raw_borrowed(&sample), ::core::mem::transmute_copy(&streamheader), ::core::mem::transmute_copy(&iooperation)).into())
        }
        unsafe extern "system" fn KsQueryExtendedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendedsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsQueryExtendedSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extendedsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsSetMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetMediaType(this, ::core::mem::transmute_copy(&ammediatype)).into())
        }
        IKsDataTypeHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsCompleteIoOperation: KsCompleteIoOperation::<Identity, Impl, OFFSET>,
            KsIsMediaTypeInRanges: KsIsMediaTypeInRanges::<Identity, Impl, OFFSET>,
            KsPrepareIoOperation: KsPrepareIoOperation::<Identity, Impl, OFFSET>,
            KsQueryExtendedSize: KsQueryExtendedSize::<Identity, Impl, OFFSET>,
            KsSetMediaType: KsSetMediaType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsFormatSupport_Impl: ::windows_core::BaseImpl {
    fn IsFormatSupported(this: &Self::This, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDevicePreferredFormat(this: &Self::This) -> ::windows_core::Result<*mut KSDATAFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsFormatSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsFormatSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsFormatSupported(this, ::core::mem::transmute_copy(&pksformat), ::core::mem::transmute_copy(&cbformat), ::core::mem::transmute_copy(&pbsupported)).into())
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevicePreferredFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppksformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsFormatSupport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IKsInterfaceHandler_Impl: ::windows_core::BaseImpl {
    fn KsSetPin(this: &Self::This, kspin: ::core::option::Option<&IKsPin>) -> ::windows_core::Result<()>;
    fn KsProcessMediaSamples(this: &Self::This, ksdatatypehandler: ::core::option::Option<&IKsDataTypeHandler>, samplelist: *const ::core::option::Option<super::DirectShow::IMediaSample>, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> ::windows_core::Result<()>;
    fn KsCompleteIo(this: &Self::This, streamsegment: *mut KSSTREAM_SEGMENT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ::windows_core::Iids for IKsInterfaceHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsInterfaceHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsSetPin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kspin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetPin(this, ::windows_core::from_raw_borrowed(&kspin)).into())
        }
        unsafe extern "system" fn KsProcessMediaSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ksdatatypehandler: *mut ::core::ffi::c_void, samplelist: *const *mut ::core::ffi::c_void, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsProcessMediaSamples(this, ::windows_core::from_raw_borrowed(&ksdatatypehandler), ::core::mem::transmute_copy(&samplelist), ::core::mem::transmute_copy(&samplecount), ::core::mem::transmute_copy(&iooperation), ::core::mem::transmute_copy(&streamsegment)).into())
        }
        unsafe extern "system" fn KsCompleteIo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamsegment: *mut KSSTREAM_SEGMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsCompleteIo(this, ::core::mem::transmute_copy(&streamsegment)).into())
        }
        IKsInterfaceHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsSetPin: KsSetPin::<Identity, Impl, OFFSET>,
            KsProcessMediaSamples: KsProcessMediaSamples::<Identity, Impl, OFFSET>,
            KsCompleteIo: KsCompleteIo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsJackContainerId_Impl: ::windows_core::BaseImpl {
    fn GetJackContainerId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IKsJackContainerId {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsJackContainerId {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJackContainerId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJackContainerId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjackcontainerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsJackContainerId_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJackContainerId: GetJackContainerId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackDescription_Impl: ::windows_core::BaseImpl {
    fn GetJackCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetJackDescription(this: &Self::This, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsJackDescription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsJackDescription {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJackCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJackCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJackDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetJackDescription(this, ::core::mem::transmute_copy(&njack), ::core::mem::transmute_copy(&pdescription)).into())
        }
        IKsJackDescription_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsJackDescription2_Impl: ::windows_core::BaseImpl {
    fn GetJackCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetJackDescription2(this: &Self::This, njack: u32) -> ::windows_core::Result<KSJACK_DESCRIPTION2>;
}
impl ::windows_core::Iids for IKsJackDescription2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsJackDescription2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJackCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJackCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJackDescription2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJackDescription2(this, ::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsJackDescription2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription2: GetJackDescription2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsJackDescription3_Impl: ::windows_core::BaseImpl {
    fn GetJackCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetJackDescription3(this: &Self::This, njack: u32) -> ::windows_core::Result<KSJACK_DESCRIPTION3>;
}
impl ::windows_core::Iids for IKsJackDescription3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsJackDescription3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJackCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJackCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJackDescription3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackDescription3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription3: *mut KSJACK_DESCRIPTION3) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJackDescription3(this, ::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription3, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsJackDescription3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription3: GetJackDescription3::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackSinkInformation_Impl: ::windows_core::BaseImpl {
    fn GetJackSinkInformation(this: &Self::This, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsJackSinkInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsJackSinkInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetJackSinkInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetJackSinkInformation(this, ::core::mem::transmute_copy(&pjacksinkinformation)).into())
        }
        IKsJackSinkInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetJackSinkInformation: GetJackSinkInformation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsNodeControl_Impl: ::windows_core::BaseImpl {
    fn SetNodeId(this: &Self::This, dwnodeid: u32) -> ::windows_core::Result<()>;
    fn SetKsControl(this: &Self::This, pkscontrol: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IKsNodeControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsNodeControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsNodeControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNodeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsNodeControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnodeid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNodeId(this, ::core::mem::transmute_copy(&dwnodeid)).into())
        }
        unsafe extern "system" fn SetKsControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsNodeControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkscontrol: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKsControl(this, ::core::mem::transmute_copy(&pkscontrol)).into())
        }
        IKsNodeControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNodeId: SetNodeId::<Identity, Impl, OFFSET>,
            SetKsControl: SetKsControl::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsNotifyEvent_Impl: ::windows_core::BaseImpl {
    fn KsNotifyEvent(this: &Self::This, event: u32, lparam1: usize, lparam2: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IKsNotifyEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsNotifyEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsNotifyEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsNotifyEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsNotifyEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: u32, lparam1: usize, lparam2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsNotifyEvent(this, ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&lparam1), ::core::mem::transmute_copy(&lparam2)).into())
        }
        IKsNotifyEvent_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, KsNotifyEvent: KsNotifyEvent::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsObject_Impl: ::windows_core::BaseImpl {
    fn KsGetObjectHandle(this: &Self::This) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsGetObjectHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetObjectHandle(this))
        }
        IKsObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsGetObjectHandle: KsGetObjectHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IKsPin_Impl: ::windows_core::BaseImpl {
    fn KsQueryMediums(this: &Self::This) -> ::windows_core::Result<*mut KSMULTIPLE_ITEM>;
    fn KsQueryInterfaces(this: &Self::This) -> ::windows_core::Result<*mut KSMULTIPLE_ITEM>;
    fn KsCreateSinkPinHandle(this: &Self::This, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> ::windows_core::Result<()>;
    fn KsGetCurrentCommunication(this: &Self::This, communication: *mut KSPIN_COMMUNICATION, interface: *mut KSIDENTIFIER, medium: *mut KSIDENTIFIER) -> ::windows_core::Result<()>;
    fn KsPropagateAcquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn KsDeliver(this: &Self::This, sample: ::core::option::Option<&super::DirectShow::IMediaSample>, flags: u32) -> ::windows_core::Result<()>;
    fn KsMediaSamplesCompleted(this: &Self::This, streamsegment: *const KSSTREAM_SEGMENT) -> ::windows_core::Result<()>;
    fn KsPeekAllocator(this: &Self::This, operation: KSPEEKOPERATION) -> ::core::option::Option<super::DirectShow::IMemAllocator>;
    fn KsReceiveAllocator(this: &Self::This, memallocator: ::core::option::Option<&super::DirectShow::IMemAllocator>) -> ::windows_core::Result<()>;
    fn KsRenegotiateAllocator(this: &Self::This) -> ::windows_core::Result<()>;
    fn KsIncrementPendingIoCount(this: &Self::This) -> i32;
    fn KsDecrementPendingIoCount(this: &Self::This) -> i32;
    fn KsQualityNotify(this: &Self::This, proportion: u32, timedelta: i64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ::windows_core::Iids for IKsPin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsPin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsQueryMediums<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediumlist: *mut *mut KSMULTIPLE_ITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsQueryMediums(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mediumlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsQueryInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfacelist: *mut *mut KSMULTIPLE_ITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsQueryInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfacelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KsCreateSinkPinHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsCreateSinkPinHandle(this, ::core::mem::transmute_copy(&interface), ::core::mem::transmute_copy(&medium)).into())
        }
        unsafe extern "system" fn KsGetCurrentCommunication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, communication: *mut KSPIN_COMMUNICATION, interface: *mut KSIDENTIFIER, medium: *mut KSIDENTIFIER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetCurrentCommunication(this, ::core::mem::transmute_copy(&communication), ::core::mem::transmute_copy(&interface), ::core::mem::transmute_copy(&medium)).into())
        }
        unsafe extern "system" fn KsPropagateAcquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsPropagateAcquire(this).into())
        }
        unsafe extern "system" fn KsDeliver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sample: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsDeliver(this, ::windows_core::from_raw_borrowed(&sample), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn KsMediaSamplesCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamsegment: *const KSSTREAM_SEGMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsMediaSamplesCompleted(this, ::core::mem::transmute_copy(&streamsegment)).into())
        }
        unsafe extern "system" fn KsPeekAllocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: KSPEEKOPERATION) -> ::core::option::Option<super::DirectShow::IMemAllocator> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsPeekAllocator(this, ::core::mem::transmute_copy(&operation)))
        }
        unsafe extern "system" fn KsReceiveAllocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memallocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsReceiveAllocator(this, ::windows_core::from_raw_borrowed(&memallocator)).into())
        }
        unsafe extern "system" fn KsRenegotiateAllocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsRenegotiateAllocator(this).into())
        }
        unsafe extern "system" fn KsIncrementPendingIoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsIncrementPendingIoCount(this))
        }
        unsafe extern "system" fn KsDecrementPendingIoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsDecrementPendingIoCount(this))
        }
        unsafe extern "system" fn KsQualityNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proportion: u32, timedelta: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsQualityNotify(this, ::core::mem::transmute_copy(&proportion), ::core::mem::transmute_copy(&timedelta)).into())
        }
        IKsPin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsQueryMediums: KsQueryMediums::<Identity, Impl, OFFSET>,
            KsQueryInterfaces: KsQueryInterfaces::<Identity, Impl, OFFSET>,
            KsCreateSinkPinHandle: KsCreateSinkPinHandle::<Identity, Impl, OFFSET>,
            KsGetCurrentCommunication: KsGetCurrentCommunication::<Identity, Impl, OFFSET>,
            KsPropagateAcquire: KsPropagateAcquire::<Identity, Impl, OFFSET>,
            KsDeliver: KsDeliver::<Identity, Impl, OFFSET>,
            KsMediaSamplesCompleted: KsMediaSamplesCompleted::<Identity, Impl, OFFSET>,
            KsPeekAllocator: KsPeekAllocator::<Identity, Impl, OFFSET>,
            KsReceiveAllocator: KsReceiveAllocator::<Identity, Impl, OFFSET>,
            KsRenegotiateAllocator: KsRenegotiateAllocator::<Identity, Impl, OFFSET>,
            KsIncrementPendingIoCount: KsIncrementPendingIoCount::<Identity, Impl, OFFSET>,
            KsDecrementPendingIoCount: KsDecrementPendingIoCount::<Identity, Impl, OFFSET>,
            KsQualityNotify: KsQualityNotify::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IKsPinEx_Impl: ::windows_core::BaseImpl + IKsPin_Impl {
    fn KsNotifyError(this: &Self::This, sample: ::core::option::Option<&super::DirectShow::IMediaSample>, hr: ::windows_core::HRESULT);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ::windows_core::Iids for IKsPinEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IKsPin);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsPinEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsNotifyError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sample: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsNotifyError(this, ::windows_core::from_raw_borrowed(&sample), ::core::mem::transmute_copy(&hr)))
        }
        IKsPinEx_Vtbl { base__: <IKsPin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, KsNotifyError: KsNotifyError::<Identity, Impl, OFFSET> }
    };
}
pub trait IKsPinFactory_Impl: ::windows_core::BaseImpl {
    fn KsPinFactory(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IKsPinFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsPinFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsPinFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfactory: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KsPinFactory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsPinFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, KsPinFactory: KsPinFactory::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Media_DirectShow\"`"]
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPinPipe_Impl: ::windows_core::BaseImpl {
    fn KsGetPinFramingCache(this: &Self::This, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> ::windows_core::Result<()>;
    fn KsSetPinFramingCache(this: &Self::This, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> ::windows_core::Result<()>;
    fn KsGetConnectedPin(this: &Self::This) -> ::core::option::Option<super::DirectShow::IPin>;
    fn KsGetPipe(this: &Self::This, operation: KSPEEKOPERATION) -> ::core::option::Option<IKsAllocatorEx>;
    fn KsSetPipe(this: &Self::This, ksallocator: ::core::option::Option<&IKsAllocatorEx>) -> ::windows_core::Result<()>;
    fn KsGetPipeAllocatorFlag(this: &Self::This) -> u32;
    fn KsSetPipeAllocatorFlag(this: &Self::This, flag: u32) -> ::windows_core::Result<()>;
    fn KsGetPinBusCache(this: &Self::This) -> ::windows_core::GUID;
    fn KsSetPinBusCache(this: &Self::This, bus: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn KsGetPinName(this: &Self::This) -> ::windows_core::PWSTR;
    fn KsGetFilterName(this: &Self::This) -> ::windows_core::PWSTR;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl ::windows_core::Iids for IKsPinPipe {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsPinPipe {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsGetPinFramingCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetPinFramingCache(this, ::core::mem::transmute_copy(&framingex), ::core::mem::transmute_copy(&framingprop), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn KsSetPinFramingCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetPinFramingCache(this, ::core::mem::transmute_copy(&framingex), ::core::mem::transmute_copy(&framingprop), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn KsGetConnectedPin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<super::DirectShow::IPin> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetConnectedPin(this))
        }
        unsafe extern "system" fn KsGetPipe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: KSPEEKOPERATION) -> ::core::option::Option<IKsAllocatorEx> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetPipe(this, ::core::mem::transmute_copy(&operation)))
        }
        unsafe extern "system" fn KsSetPipe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ksallocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetPipe(this, ::windows_core::from_raw_borrowed(&ksallocator)).into())
        }
        unsafe extern "system" fn KsGetPipeAllocatorFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetPipeAllocatorFlag(this))
        }
        unsafe extern "system" fn KsSetPipeAllocatorFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flag: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetPipeAllocatorFlag(this, ::core::mem::transmute_copy(&flag)).into())
        }
        unsafe extern "system" fn KsGetPinBusCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::KsGetPinBusCache(this))
        }
        unsafe extern "system" fn KsSetPinBusCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bus: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsSetPinBusCache(this, ::core::mem::transmute(&bus)).into())
        }
        unsafe extern "system" fn KsGetPinName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::PWSTR {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetPinName(this))
        }
        unsafe extern "system" fn KsGetFilterName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::PWSTR {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsGetFilterName(this))
        }
        IKsPinPipe_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            KsGetPinFramingCache: KsGetPinFramingCache::<Identity, Impl, OFFSET>,
            KsSetPinFramingCache: KsSetPinFramingCache::<Identity, Impl, OFFSET>,
            KsGetConnectedPin: KsGetConnectedPin::<Identity, Impl, OFFSET>,
            KsGetPipe: KsGetPipe::<Identity, Impl, OFFSET>,
            KsSetPipe: KsSetPipe::<Identity, Impl, OFFSET>,
            KsGetPipeAllocatorFlag: KsGetPipeAllocatorFlag::<Identity, Impl, OFFSET>,
            KsSetPipeAllocatorFlag: KsSetPipeAllocatorFlag::<Identity, Impl, OFFSET>,
            KsGetPinBusCache: KsGetPinBusCache::<Identity, Impl, OFFSET>,
            KsSetPinBusCache: KsSetPinBusCache::<Identity, Impl, OFFSET>,
            KsGetPinName: KsGetPinName::<Identity, Impl, OFFSET>,
            KsGetFilterName: KsGetFilterName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsPropertySet_Impl: ::windows_core::BaseImpl {
    fn Set(this: &Self::This, guidpropset: *const ::windows_core::GUID, dwpropid: u32, pinstancedata: *const ::core::ffi::c_void, cbinstancedata: u32, ppropdata: *const ::core::ffi::c_void, cbpropdata: u32) -> ::windows_core::Result<()>;
    fn Get(this: &Self::This, guidpropset: *const ::windows_core::GUID, dwpropid: u32, pinstancedata: *const ::core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut ::core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> ::windows_core::Result<()>;
    fn QuerySupported(this: &Self::This, guidpropset: *const ::windows_core::GUID, dwpropid: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IKsPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsPropertySet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidpropset: *const ::windows_core::GUID, dwpropid: u32, pinstancedata: *const ::core::ffi::c_void, cbinstancedata: u32, ppropdata: *const ::core::ffi::c_void, cbpropdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::core::mem::transmute_copy(&guidpropset), ::core::mem::transmute_copy(&dwpropid), ::core::mem::transmute_copy(&pinstancedata), ::core::mem::transmute_copy(&cbinstancedata), ::core::mem::transmute_copy(&ppropdata), ::core::mem::transmute_copy(&cbpropdata)).into())
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidpropset: *const ::windows_core::GUID, dwpropid: u32, pinstancedata: *const ::core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut ::core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute_copy(&guidpropset), ::core::mem::transmute_copy(&dwpropid), ::core::mem::transmute_copy(&pinstancedata), ::core::mem::transmute_copy(&cbinstancedata), ::core::mem::transmute_copy(&ppropdata), ::core::mem::transmute_copy(&cbpropdata), ::core::mem::transmute_copy(&pcbreturned)).into())
        }
        unsafe extern "system" fn QuerySupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidpropset: *const ::windows_core::GUID, dwpropid: u32, ptypesupport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySupported(this, ::core::mem::transmute_copy(&guidpropset), ::core::mem::transmute_copy(&dwpropid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptypesupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKsPropertySet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            QuerySupported: QuerySupported::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKsQualityForwarder_Impl: ::windows_core::BaseImpl + IKsObject_Impl {
    fn KsFlushClient(this: &Self::This, pin: ::core::option::Option<&IKsPin>);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKsQualityForwarder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IKsObject);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsQualityForwarder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsQualityForwarder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn KsFlushClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsQualityForwarder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KsFlushClient(this, ::windows_core::from_raw_borrowed(&pin)))
        }
        IKsQualityForwarder_Vtbl { base__: <IKsObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, KsFlushClient: KsFlushClient::<Identity, Impl, OFFSET> }
    };
}
pub trait IKsTopology_Impl: ::windows_core::BaseImpl {
    fn CreateNodeInstance(this: &Self::This, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: ::core::option::Option<&::windows_core::IUnknown>, interfaceid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IKsTopology {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsTopology {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateNodeInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNodeInstance(this, ::core::mem::transmute_copy(&nodeid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&desiredaccess), ::windows_core::from_raw_borrowed(&unkouter), ::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&interface)).into())
        }
        IKsTopology_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IKsTopologyInfo_Impl: ::windows_core::BaseImpl {
    fn NumCategories(this: &Self::This) -> ::windows_core::Result<u32>;
    fn get_Category(this: &Self::This, dwindex: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn NumConnections(this: &Self::This) -> ::windows_core::Result<u32>;
    fn get_ConnectionInfo(this: &Self::This, dwindex: u32) -> ::windows_core::Result<KSTOPOLOGY_CONNECTION>;
    fn get_NodeName(this: &Self::This, dwnodeid: u32, pwchnodename: ::windows_core::PWSTR, dwbufsize: u32, pdwnamelen: *mut u32) -> ::windows_core::Result<()>;
    fn NumNodes(this: &Self::This) -> ::windows_core::Result<u32>;
    fn get_NodeType(this: &Self::This, dwnodeid: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn CreateNodeInstance(this: &Self::This, dwnodeid: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IKsTopologyInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKsTopologyInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NumCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwnumcategories: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumCategories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumcategories, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcategory: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Category(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwnumconnections: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumconnections, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ConnectionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pconnectioninfo: *mut KSTOPOLOGY_CONNECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ConnectionInfo(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectioninfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_NodeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnodeid: u32, pwchnodename: ::windows_core::PWSTR, dwbufsize: u32, pdwnamelen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_NodeName(this, ::core::mem::transmute_copy(&dwnodeid), ::core::mem::transmute_copy(&pwchnodename), ::core::mem::transmute_copy(&dwbufsize), ::core::mem::transmute_copy(&pdwnamelen)).into())
        }
        unsafe extern "system" fn NumNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwnumnodes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumnodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_NodeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnodeid: u32, pnodetype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_NodeType(this, ::core::mem::transmute_copy(&dwnodeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateNodeInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnodeid: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNodeInstance(this, ::core::mem::transmute_copy(&dwnodeid), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        IKsTopologyInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NumCategories: NumCategories::<Identity, Impl, OFFSET>,
            get_Category: get_Category::<Identity, Impl, OFFSET>,
            NumConnections: NumConnections::<Identity, Impl, OFFSET>,
            get_ConnectionInfo: get_ConnectionInfo::<Identity, Impl, OFFSET>,
            get_NodeName: get_NodeName::<Identity, Impl, OFFSET>,
            NumNodes: NumNodes::<Identity, Impl, OFFSET>,
            get_NodeType: get_NodeType::<Identity, Impl, OFFSET>,
            CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET>,
        }
    };
}
