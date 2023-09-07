#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ICompositionFramePresentStatistics_Impl: ::windows_core::BaseImpl + IPresentStatistics_Impl {
    fn GetContentTag(this: &Self::This) -> usize;
    fn GetCompositionFrameId(this: &Self::This) -> u64;
    fn GetDisplayInstanceArray(this: &Self::This, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ICompositionFramePresentStatistics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPresentStatistics);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionFramePresentStatistics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContentTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentTag(this))
        }
        unsafe extern "system" fn GetCompositionFrameId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionFrameId(this))
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayInstanceArray(this, ::core::mem::transmute_copy(&displayinstancearraycount), ::core::mem::transmute_copy(&displayinstancearray)))
        }
        ICompositionFramePresentStatistics_Vtbl {
            base__: <IPresentStatistics as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContentTag: GetContentTag::<Identity, Impl, OFFSET>,
            GetCompositionFrameId: GetCompositionFrameId::<Identity, Impl, OFFSET>,
            GetDisplayInstanceArray: GetDisplayInstanceArray::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IIndependentFlipFramePresentStatistics_Impl: ::windows_core::BaseImpl + IPresentStatistics_Impl {
    fn GetOutputAdapterLUID(this: &Self::This) -> super::super::Foundation::LUID;
    fn GetOutputVidPnSourceId(this: &Self::This) -> u32;
    fn GetContentTag(this: &Self::This) -> usize;
    fn GetDisplayedTime(this: &Self::This) -> SystemInterruptTime;
    fn GetPresentDuration(this: &Self::This) -> SystemInterruptTime;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IIndependentFlipFramePresentStatistics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPresentStatistics);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIndependentFlipFramePresentStatistics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOutputAdapterLUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetOutputAdapterLUID(this))
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputVidPnSourceId(this))
        }
        unsafe extern "system" fn GetContentTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentTag(this))
        }
        unsafe extern "system" fn GetDisplayedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDisplayedTime(this))
        }
        unsafe extern "system" fn GetPresentDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetPresentDuration(this))
        }
        IIndependentFlipFramePresentStatistics_Vtbl {
            base__: <IPresentStatistics as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOutputAdapterLUID: GetOutputAdapterLUID::<Identity, Impl, OFFSET>,
            GetOutputVidPnSourceId: GetOutputVidPnSourceId::<Identity, Impl, OFFSET>,
            GetContentTag: GetContentTag::<Identity, Impl, OFFSET>,
            GetDisplayedTime: GetDisplayedTime::<Identity, Impl, OFFSET>,
            GetPresentDuration: GetPresentDuration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPresentStatistics_Impl: ::windows_core::BaseImpl {
    fn GetPresentId(this: &Self::This) -> u64;
    fn GetKind(this: &Self::This) -> PresentStatisticsKind;
}
impl ::windows_core::Iids for IPresentStatistics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentStatistics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPresentId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresentId(this))
        }
        unsafe extern "system" fn GetKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKind(this))
        }
        IPresentStatistics_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPresentId: GetPresentId::<Identity, Impl, OFFSET>,
            GetKind: GetKind::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPresentStatusPresentStatistics_Impl: ::windows_core::BaseImpl + IPresentStatistics_Impl {
    fn GetCompositionFrameId(this: &Self::This) -> u64;
    fn GetPresentStatus(this: &Self::This) -> PresentStatus;
}
impl ::windows_core::Iids for IPresentStatusPresentStatistics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPresentStatistics);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentStatusPresentStatistics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCompositionFrameId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionFrameId(this))
        }
        unsafe extern "system" fn GetPresentStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> PresentStatus {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresentStatus(this))
        }
        IPresentStatusPresentStatistics_Vtbl {
            base__: <IPresentStatistics as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCompositionFrameId: GetCompositionFrameId::<Identity, Impl, OFFSET>,
            GetPresentStatus: GetPresentStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationBuffer_Impl: ::windows_core::BaseImpl {
    fn GetAvailableEvent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn IsAvailable(this: &Self::This) -> ::windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPresentationBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentationBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAvailableEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(availableeventhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPresentationBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAvailableEvent: GetAvailableEvent::<Identity, Impl, OFFSET>,
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPresentationContent_Impl: ::windows_core::BaseImpl {
    fn SetTag(this: &Self::This, tag: usize);
}
impl ::windows_core::Iids for IPresentationContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentationContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag: usize) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::core::mem::transmute_copy(&tag)))
        }
        IPresentationContent_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetTag: SetTag::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPresentationFactory_Impl: ::windows_core::BaseImpl {
    fn IsPresentationSupported(this: &Self::This) -> u8;
    fn IsPresentationSupportedWithIndependentFlip(this: &Self::This) -> u8;
    fn CreatePresentationManager(this: &Self::This) -> ::windows_core::Result<IPresentationManager>;
}
impl ::windows_core::Iids for IPresentationFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentationFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPresentationSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u8 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPresentationSupported(this))
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u8 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPresentationSupportedWithIndependentFlip(this))
        }
        unsafe extern "system" fn CreatePresentationManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePresentationManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppresentationmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPresentationFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsPresentationSupported: IsPresentationSupported::<Identity, Impl, OFFSET>,
            IsPresentationSupportedWithIndependentFlip: IsPresentationSupportedWithIndependentFlip::<Identity, Impl, OFFSET>,
            CreatePresentationManager: CreatePresentationManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationManager_Impl: ::windows_core::BaseImpl {
    fn AddBufferFromResource(this: &Self::This, resource: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IPresentationBuffer>;
    fn CreatePresentationSurface(this: &Self::This, compositionsurfacehandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<IPresentationSurface>;
    fn GetNextPresentId(this: &Self::This) -> u64;
    fn SetTargetTime(this: &Self::This, targettime: &SystemInterruptTime) -> ::windows_core::Result<()>;
    fn SetPreferredPresentDuration(this: &Self::This, preferredduration: &SystemInterruptTime, deviationtolerance: &SystemInterruptTime) -> ::windows_core::Result<()>;
    fn ForceVSyncInterrupt(this: &Self::This, forcevsyncinterrupt: u8) -> ::windows_core::Result<()>;
    fn Present(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetPresentRetiringFence(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
    fn CancelPresentsFrom(this: &Self::This, presentidtocancelfrom: u64) -> ::windows_core::Result<()>;
    fn GetLostEvent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetPresentStatisticsAvailableEvent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn EnablePresentStatisticsKind(this: &Self::This, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows_core::Result<()>;
    fn GetNextPresentStatistics(this: &Self::This) -> ::windows_core::Result<IPresentStatistics>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPresentationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddBufferFromResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddBufferFromResource(this, ::windows_core::from_raw_borrowed(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presentationbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePresentationSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePresentationSurface(this, ::core::mem::transmute_copy(&compositionsurfacehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presentationsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextPresentId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextPresentId(this))
        }
        unsafe extern "system" fn SetTargetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetTime(this, ::core::mem::transmute(&targettime)).into())
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreferredPresentDuration(this, ::core::mem::transmute(&preferredduration), ::core::mem::transmute(&deviationtolerance)).into())
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceVSyncInterrupt(this, ::core::mem::transmute_copy(&forcevsyncinterrupt)).into())
        }
        unsafe extern "system" fn Present<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present(this).into())
        }
        unsafe extern "system" fn GetPresentRetiringFence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPresentRetiringFence(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CancelPresentsFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelPresentsFrom(this, ::core::mem::transmute_copy(&presentidtocancelfrom)).into())
        }
        unsafe extern "system" fn GetLostEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLostEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(losteventhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPresentStatisticsAvailableEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presentstatisticsavailableeventhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnablePresentStatisticsKind(this, ::core::mem::transmute_copy(&presentstatisticskind), ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn GetNextPresentStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextPresentStatistics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextpresentstatistics, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPresentationManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddBufferFromResource: AddBufferFromResource::<Identity, Impl, OFFSET>,
            CreatePresentationSurface: CreatePresentationSurface::<Identity, Impl, OFFSET>,
            GetNextPresentId: GetNextPresentId::<Identity, Impl, OFFSET>,
            SetTargetTime: SetTargetTime::<Identity, Impl, OFFSET>,
            SetPreferredPresentDuration: SetPreferredPresentDuration::<Identity, Impl, OFFSET>,
            ForceVSyncInterrupt: ForceVSyncInterrupt::<Identity, Impl, OFFSET>,
            Present: Present::<Identity, Impl, OFFSET>,
            GetPresentRetiringFence: GetPresentRetiringFence::<Identity, Impl, OFFSET>,
            CancelPresentsFrom: CancelPresentsFrom::<Identity, Impl, OFFSET>,
            GetLostEvent: GetLostEvent::<Identity, Impl, OFFSET>,
            GetPresentStatisticsAvailableEvent: GetPresentStatisticsAvailableEvent::<Identity, Impl, OFFSET>,
            EnablePresentStatisticsKind: EnablePresentStatisticsKind::<Identity, Impl, OFFSET>,
            GetNextPresentStatistics: GetNextPresentStatistics::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IPresentationSurface_Impl: ::windows_core::BaseImpl + IPresentationContent_Impl {
    fn SetBuffer(this: &Self::This, presentationbuffer: ::core::option::Option<&IPresentationBuffer>) -> ::windows_core::Result<()>;
    fn SetColorSpace(this: &Self::This, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<()>;
    fn SetAlphaMode(this: &Self::This, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<()>;
    fn SetSourceRect(this: &Self::This, sourcerect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetTransform(this: &Self::This, transform: *const PresentationTransform) -> ::windows_core::Result<()>;
    fn RestrictToOutput(this: &Self::This, output: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetDisableReadback(this: &Self::This, value: u8) -> ::windows_core::Result<()>;
    fn SetLetterboxingMargins(this: &Self::This, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IPresentationSurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPresentationContent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPresentationSurface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentationbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBuffer(this, ::windows_core::from_raw_borrowed(&presentationbuffer)).into())
        }
        unsafe extern "system" fn SetColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorSpace(this, ::core::mem::transmute_copy(&colorspace)).into())
        }
        unsafe extern "system" fn SetAlphaMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaMode(this, ::core::mem::transmute_copy(&alphamode)).into())
        }
        unsafe extern "system" fn SetSourceRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourceRect(this, ::core::mem::transmute_copy(&sourcerect)).into())
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn RestrictToOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestrictToOutput(this, ::windows_core::from_raw_borrowed(&output)).into())
        }
        unsafe extern "system" fn SetDisableReadback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisableReadback(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetLetterboxingMargins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLetterboxingMargins(this, ::core::mem::transmute_copy(&leftletterboxsize), ::core::mem::transmute_copy(&topletterboxsize), ::core::mem::transmute_copy(&rightletterboxsize), ::core::mem::transmute_copy(&bottomletterboxsize)).into())
        }
        IPresentationSurface_Vtbl {
            base__: <IPresentationContent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBuffer: SetBuffer::<Identity, Impl, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, Impl, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, Impl, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            RestrictToOutput: RestrictToOutput::<Identity, Impl, OFFSET>,
            SetDisableReadback: SetDisableReadback::<Identity, Impl, OFFSET>,
            SetLetterboxingMargins: SetLetterboxingMargins::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
