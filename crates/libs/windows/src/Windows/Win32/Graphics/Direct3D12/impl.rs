pub trait ID3D12CommandAllocator_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12CommandAllocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandAllocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12CommandAllocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandAllocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        ID3D12CommandAllocator_Vtbl { base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Reset: Reset::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12CommandList_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {
    fn GetType(this: &Self::This) -> D3D12_COMMAND_LIST_TYPE;
}
impl ::windows_core::Iids for ID3D12CommandList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12CommandList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this))
        }
        ID3D12CommandList_Vtbl { base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetType: GetType::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12CommandQueue_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn UpdateTileMappings(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: ::core::option::Option<&ID3D12Heap>, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS);
    fn CopyTileMappings(this: &Self::This, pdstresource: ::core::option::Option<&ID3D12Resource>, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: ::core::option::Option<&ID3D12Resource>, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS);
    fn ExecuteCommandLists(this: &Self::This, numcommandlists: u32, ppcommandlists: *const ::core::option::Option<ID3D12CommandList>);
    fn SetMarker(this: &Self::This, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn BeginEvent(this: &Self::This, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn EndEvent(this: &Self::This);
    fn Signal(this: &Self::This, pfence: ::core::option::Option<&ID3D12Fence>, value: u64) -> ::windows_core::Result<()>;
    fn Wait(this: &Self::This, pfence: ::core::option::Option<&ID3D12Fence>, value: u64) -> ::windows_core::Result<()>;
    fn GetTimestampFrequency(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetClockCalibration(this: &Self::This, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows_core::Result<()>;
    fn GetDesc(this: &Self::This) -> D3D12_COMMAND_QUEUE_DESC;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12CommandQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12CommandQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateTileMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: *mut ::core::ffi::c_void, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS) {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::UpdateTileMappings(
                    this,
                    ::windows_core::from_raw_borrowed(&presource),
                    ::core::mem::transmute_copy(&numresourceregions),
                    ::core::mem::transmute_copy(&presourceregionstartcoordinates),
                    ::core::mem::transmute_copy(&presourceregionsizes),
                    ::windows_core::from_raw_borrowed(&pheap),
                    ::core::mem::transmute_copy(&numranges),
                    ::core::mem::transmute_copy(&prangeflags),
                    ::core::mem::transmute_copy(&pheaprangestartoffsets),
                    ::core::mem::transmute_copy(&prangetilecounts),
                    ::core::mem::transmute_copy(&flags),
                )
            })
        }
        unsafe extern "system" fn CopyTileMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: *mut ::core::ffi::c_void, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTileMappings(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&pdstregionstartcoordinate), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&psrcregionstartcoordinate), ::core::mem::transmute_copy(&pregionsize), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn ExecuteCommandLists<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numcommandlists: u32, ppcommandlists: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteCommandLists(this, ::core::mem::transmute_copy(&numcommandlists), ::core::mem::transmute_copy(&ppcommandlists)))
        }
        unsafe extern "system" fn SetMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMarker(this, ::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size)))
        }
        unsafe extern "system" fn BeginEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEvent(this, ::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size)))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEvent(this))
        }
        unsafe extern "system" fn Signal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Signal(this, ::windows_core::from_raw_borrowed(&pfence), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::windows_core::from_raw_borrowed(&pfence), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetTimestampFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrequency: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimestampFrequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClockCalibration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClockCalibration(this, ::core::mem::transmute_copy(&pgputimestamp), ::core::mem::transmute_copy(&pcputimestamp)).into())
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_COMMAND_QUEUE_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        ID3D12CommandQueue_Vtbl {
            base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateTileMappings: UpdateTileMappings::<Identity, Impl, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, Impl, OFFSET>,
            ExecuteCommandLists: ExecuteCommandLists::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            GetTimestampFrequency: GetTimestampFrequency::<Identity, Impl, OFFSET>,
            GetClockCalibration: GetClockCalibration::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12CommandSignature_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {}
impl ::windows_core::Iids for ID3D12CommandSignature {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12CommandSignature_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12CommandSignature {
    const VTABLE: Self::Vtable = { ID3D12CommandSignature_Vtbl { base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DSRDeviceFactory_Impl: ::windows_core::BaseImpl {
    fn CreateDSRDevice(this: &Self::This, pd3d12device: ::core::option::Option<&ID3D12Device>, nodemask: u32, riid: *const ::windows_core::GUID, ppvdsrdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12DSRDeviceFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DSRDeviceFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DSRDeviceFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDSRDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DSRDeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pd3d12device: *mut ::core::ffi::c_void, nodemask: u32, riid: *const ::windows_core::GUID, ppvdsrdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDSRDevice(this, ::windows_core::from_raw_borrowed(&pd3d12device), ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdsrdevice)).into())
        }
        ID3D12DSRDeviceFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDSRDevice: CreateDSRDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12Debug_Impl: ::windows_core::BaseImpl {
    fn EnableDebugLayer(this: &Self::This);
}
impl ::windows_core::Iids for ID3D12Debug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableDebugLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableDebugLayer(this))
        }
        ID3D12Debug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableDebugLayer: EnableDebugLayer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug1_Impl: ::windows_core::BaseImpl {
    fn EnableDebugLayer(this: &Self::This);
    fn SetEnableGPUBasedValidation(this: &Self::This, enable: super::super::Foundation::BOOL);
    fn SetEnableSynchronizedCommandQueueValidation(this: &Self::This, enable: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Debug1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableDebugLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableDebugLayer(this))
        }
        unsafe extern "system" fn SetEnableGPUBasedValidation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableGPUBasedValidation(this, ::core::mem::transmute_copy(&enable)))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableSynchronizedCommandQueueValidation(this, ::core::mem::transmute_copy(&enable)))
        }
        ID3D12Debug1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableDebugLayer: EnableDebugLayer::<Identity, Impl, OFFSET>,
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Identity, Impl, OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12Debug2_Impl: ::windows_core::BaseImpl {
    fn SetGPUBasedValidationFlags(this: &Self::This, flags: D3D12_GPU_BASED_VALIDATION_FLAGS);
}
impl ::windows_core::Iids for ID3D12Debug2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGPUBasedValidationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGPUBasedValidationFlags(this, ::core::mem::transmute_copy(&flags)))
        }
        ID3D12Debug2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug3_Impl: ::windows_core::BaseImpl + ID3D12Debug_Impl {
    fn SetEnableGPUBasedValidation(this: &Self::This, enable: super::super::Foundation::BOOL);
    fn SetEnableSynchronizedCommandQueueValidation(this: &Self::This, enable: super::super::Foundation::BOOL);
    fn SetGPUBasedValidationFlags(this: &Self::This, flags: D3D12_GPU_BASED_VALIDATION_FLAGS);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Debug3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Debug);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEnableGPUBasedValidation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableGPUBasedValidation(this, ::core::mem::transmute_copy(&enable)))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableSynchronizedCommandQueueValidation(this, ::core::mem::transmute_copy(&enable)))
        }
        unsafe extern "system" fn SetGPUBasedValidationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGPUBasedValidationFlags(this, ::core::mem::transmute_copy(&flags)))
        }
        ID3D12Debug3_Vtbl {
            base__: <ID3D12Debug as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Identity, Impl, OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Identity, Impl, OFFSET>,
            SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug4_Impl: ::windows_core::BaseImpl + ID3D12Debug3_Impl {
    fn DisableDebugLayer(this: &Self::This);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Debug4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Debug3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisableDebugLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableDebugLayer(this))
        }
        ID3D12Debug4_Vtbl { base__: <ID3D12Debug3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DisableDebugLayer: DisableDebugLayer::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug5_Impl: ::windows_core::BaseImpl + ID3D12Debug4_Impl {
    fn SetEnableAutoName(this: &Self::This, enable: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Debug5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Debug4);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEnableAutoName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableAutoName(this, ::core::mem::transmute_copy(&enable)))
        }
        ID3D12Debug5_Vtbl { base__: <ID3D12Debug4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetEnableAutoName: SetEnableAutoName::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Debug6_Impl: ::windows_core::BaseImpl + ID3D12Debug5_Impl {
    fn SetForceLegacyBarrierValidation(this: &Self::This, enable: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Debug6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Debug5);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Debug6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetForceLegacyBarrierValidation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Debug6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForceLegacyBarrierValidation(this, ::core::mem::transmute_copy(&enable)))
        }
        ID3D12Debug6_Vtbl {
            base__: <ID3D12Debug5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetForceLegacyBarrierValidation: SetForceLegacyBarrierValidation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList_Impl: ::windows_core::BaseImpl {
    fn AssertResourceState(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
    fn SetFeatureMask(this: &Self::This, mask: D3D12_DEBUG_FEATURE) -> ::windows_core::Result<()>;
    fn GetFeatureMask(this: &Self::This) -> D3D12_DEBUG_FEATURE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DebugCommandList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugCommandList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssertResourceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertResourceState(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state)))
        }
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFeatureMask(this, ::core::mem::transmute_copy(&mask)).into())
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureMask(this))
        }
        ID3D12DebugCommandList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList1_Impl: ::windows_core::BaseImpl {
    fn AssertResourceState(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
    fn SetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
    fn GetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DebugCommandList1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugCommandList1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssertResourceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertResourceState(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state)))
        }
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        ID3D12DebugCommandList1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList2_Impl: ::windows_core::BaseImpl + ID3D12DebugCommandList_Impl {
    fn SetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
    fn GetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DebugCommandList2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DebugCommandList);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugCommandList2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        ID3D12DebugCommandList2_Vtbl {
            base__: <ID3D12DebugCommandList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandList3_Impl: ::windows_core::BaseImpl + ID3D12DebugCommandList2_Impl {
    fn AssertResourceAccess(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, access: D3D12_BARRIER_ACCESS);
    fn AssertTextureLayout(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, layout: D3D12_BARRIER_LAYOUT);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DebugCommandList3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DebugCommandList2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugCommandList3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssertResourceAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, access: D3D12_BARRIER_ACCESS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertResourceAccess(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&access)))
        }
        unsafe extern "system" fn AssertTextureLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandList3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, layout: D3D12_BARRIER_LAYOUT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertTextureLayout(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&layout)))
        }
        ID3D12DebugCommandList3_Vtbl {
            base__: <ID3D12DebugCommandList2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssertResourceAccess: AssertResourceAccess::<Identity, Impl, OFFSET>,
            AssertTextureLayout: AssertTextureLayout::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandQueue_Impl: ::windows_core::BaseImpl {
    fn AssertResourceState(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DebugCommandQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugCommandQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssertResourceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertResourceState(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&state)))
        }
        ID3D12DebugCommandQueue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DebugCommandQueue1_Impl: ::windows_core::BaseImpl + ID3D12DebugCommandQueue_Impl {
    fn AssertResourceAccess(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, access: D3D12_BARRIER_ACCESS);
    fn AssertTextureLayout(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, layout: D3D12_BARRIER_LAYOUT);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DebugCommandQueue1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DebugCommandQueue);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandQueue1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugCommandQueue1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssertResourceAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandQueue1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, access: D3D12_BARRIER_ACCESS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertResourceAccess(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&access)))
        }
        unsafe extern "system" fn AssertTextureLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugCommandQueue1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, layout: D3D12_BARRIER_LAYOUT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssertTextureLayout(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&layout)))
        }
        ID3D12DebugCommandQueue1_Vtbl {
            base__: <ID3D12DebugCommandQueue as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssertResourceAccess: AssertResourceAccess::<Identity, Impl, OFFSET>,
            AssertTextureLayout: AssertTextureLayout::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DebugDevice_Impl: ::windows_core::BaseImpl {
    fn SetFeatureMask(this: &Self::This, mask: D3D12_DEBUG_FEATURE) -> ::windows_core::Result<()>;
    fn GetFeatureMask(this: &Self::This) -> D3D12_DEBUG_FEATURE;
    fn ReportLiveDeviceObjects(this: &Self::This, flags: D3D12_RLDO_FLAGS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12DebugDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFeatureMask(this, ::core::mem::transmute_copy(&mask)).into())
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureMask(this))
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportLiveDeviceObjects(this, ::core::mem::transmute_copy(&flags)).into())
        }
        ID3D12DebugDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DebugDevice1_Impl: ::windows_core::BaseImpl {
    fn SetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
    fn GetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
    fn ReportLiveDeviceObjects(this: &Self::This, flags: D3D12_RLDO_FLAGS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12DebugDevice1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugDevice1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportLiveDeviceObjects(this, ::core::mem::transmute_copy(&flags)).into())
        }
        ID3D12DebugDevice1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DebugDevice2_Impl: ::windows_core::BaseImpl + ID3D12DebugDevice_Impl {
    fn SetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
    fn GetDebugParameter(this: &Self::This, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12DebugDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DebugDevice);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DebugDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetDebugParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DebugDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDebugParameter(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize)).into())
        }
        ID3D12DebugDevice2_Vtbl {
            base__: <ID3D12DebugDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DescriptorHeap_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn GetDesc(this: &Self::This) -> D3D12_DESCRIPTOR_HEAP_DESC;
    fn GetCPUDescriptorHandleForHeapStart(this: &Self::This) -> D3D12_CPU_DESCRIPTOR_HANDLE;
    fn GetGPUDescriptorHandleForHeapStart(this: &Self::This) -> D3D12_GPU_DESCRIPTOR_HANDLE;
}
impl ::windows_core::Iids for ID3D12DescriptorHeap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DescriptorHeap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_DESCRIPTOR_HEAP_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        unsafe extern "system" fn GetCPUDescriptorHandleForHeapStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetCPUDescriptorHandleForHeapStart(this))
        }
        unsafe extern "system" fn GetGPUDescriptorHandleForHeapStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_GPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetGPUDescriptorHandleForHeapStart(this))
        }
        ID3D12DescriptorHeap_Vtbl {
            base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetCPUDescriptorHandleForHeapStart: GetCPUDescriptorHandleForHeapStart::<Identity, Impl, OFFSET>,
            GetGPUDescriptorHandleForHeapStart: GetGPUDescriptorHandleForHeapStart::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device_Impl: ::windows_core::BaseImpl + ID3D12Object_Impl {
    fn GetNodeCount(this: &Self::This) -> u32;
    fn CreateCommandQueue(this: &Self::This, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows_core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateCommandAllocator(this: &Self::This, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows_core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateGraphicsPipelineState(this: &Self::This, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateComputePipelineState(this: &Self::This, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateCommandList(this: &Self::This, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: ::core::option::Option<&ID3D12CommandAllocator>, pinitialstate: ::core::option::Option<&ID3D12PipelineState>, riid: *const ::windows_core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CheckFeatureSupport(this: &Self::This, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()>;
    fn CreateDescriptorHeap(this: &Self::This, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDescriptorHandleIncrementSize(this: &Self::This, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32;
    fn CreateRootSignature(this: &Self::This, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows_core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateConstantBufferView(this: &Self::This, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateShaderResourceView(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateUnorderedAccessView(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, pcounterresource: ::core::option::Option<&ID3D12Resource>, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateRenderTargetView(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateDepthStencilView(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateSampler(this: &Self::This, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CopyDescriptors(this: &Self::This, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE);
    fn CopyDescriptorsSimple(this: &Self::This, numdescriptors: u32, destdescriptorrangestart: &D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: &D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE);
    fn GetResourceAllocationInfo(this: &Self::This, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) -> D3D12_RESOURCE_ALLOCATION_INFO;
    fn GetCustomHeapProperties(this: &Self::This, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES;
    fn CreateCommittedResource(this: &Self::This, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateHeap(this: &Self::This, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreatePlacedResource(this: &Self::This, pheap: ::core::option::Option<&ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateReservedResource(this: &Self::This, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateSharedHandle(this: &Self::This, pobject: ::core::option::Option<&ID3D12DeviceChild>, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(this: &Self::This, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OpenSharedHandleByName(this: &Self::This, name: &::windows_core::PCWSTR, access: u32) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn MakeResident(this: &Self::This, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>) -> ::windows_core::Result<()>;
    fn Evict(this: &Self::This, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>) -> ::windows_core::Result<()>;
    fn CreateFence(this: &Self::This, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDeviceRemovedReason(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCopyableFootprints(this: &Self::This, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64);
    fn CreateQueryHeap(this: &Self::This, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetStablePowerState(this: &Self::This, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CreateCommandSignature(this: &Self::This, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: ::core::option::Option<&ID3D12RootSignature>, riid: *const ::windows_core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetResourceTiling(this: &Self::This, ptiledresource: ::core::option::Option<&ID3D12Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING);
    fn GetAdapterLuid(this: &Self::This) -> super::super::Foundation::LUID;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Object);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNodeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNodeCount(this))
        }
        unsafe extern "system" fn CreateCommandQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows_core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandQueue(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandqueue)).into())
        }
        unsafe extern "system" fn CreateCommandAllocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows_core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandAllocator(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandallocator)).into())
        }
        unsafe extern "system" fn CreateGraphicsPipelineState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateGraphicsPipelineState(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into())
        }
        unsafe extern "system" fn CreateComputePipelineState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateComputePipelineState(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into())
        }
        unsafe extern "system" fn CreateCommandList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: *mut ::core::ffi::c_void, pinitialstate: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandList(this, ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&r#type), ::windows_core::from_raw_borrowed(&pcommandallocator), ::windows_core::from_raw_borrowed(&pinitialstate), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandlist)).into())
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckFeatureSupport(this, ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into())
        }
        unsafe extern "system" fn CreateDescriptorHeap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDescriptorHeap(this, ::core::mem::transmute_copy(&pdescriptorheapdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into())
        }
        unsafe extern "system" fn GetDescriptorHandleIncrementSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescriptorHandleIncrementSize(this, ::core::mem::transmute_copy(&descriptorheaptype)))
        }
        unsafe extern "system" fn CreateRootSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows_core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRootSignature(this, ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&pblobwithrootsignature), ::core::mem::transmute_copy(&bloblengthinbytes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvrootsignature)).into())
        }
        unsafe extern "system" fn CreateConstantBufferView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateConstantBufferView(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateShaderResourceView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pcounterresource: *mut ::core::ffi::c_void, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateUnorderedAccessView(this, ::windows_core::from_raw_borrowed(&presource), ::windows_core::from_raw_borrowed(&pcounterresource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRenderTargetView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn CreateSampler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSampler(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn CopyDescriptors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyDescriptors(this, ::core::mem::transmute_copy(&numdestdescriptorranges), ::core::mem::transmute_copy(&pdestdescriptorrangestarts), ::core::mem::transmute_copy(&pdestdescriptorrangesizes), ::core::mem::transmute_copy(&numsrcdescriptorranges), ::core::mem::transmute_copy(&psrcdescriptorrangestarts), ::core::mem::transmute_copy(&psrcdescriptorrangesizes), ::core::mem::transmute_copy(&descriptorheapstype)))
        }
        unsafe extern "system" fn CopyDescriptorsSimple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyDescriptorsSimple(this, ::core::mem::transmute_copy(&numdescriptors), ::core::mem::transmute(&destdescriptorrangestart), ::core::mem::transmute(&srcdescriptorrangestart), ::core::mem::transmute_copy(&descriptorheapstype)))
        }
        unsafe extern "system" fn GetResourceAllocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetResourceAllocationInfo(this, ::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs)))
        }
        unsafe extern "system" fn GetCustomHeapProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_PROPERTIES, nodemask: u32, heaptype: D3D12_HEAP_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetCustomHeapProperties(this, ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&heaptype)))
        }
        unsafe extern "system" fn CreateCommittedResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommittedResource(this, ::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreateHeap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateHeap(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into())
        }
        unsafe extern "system" fn CreatePlacedResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheap: *mut ::core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePlacedResource(this, ::windows_core::from_raw_borrowed(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreateReservedResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateReservedResource(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSharedHandle(this, ::windows_core::from_raw_borrowed(&pobject), ::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&access), ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenSharedHandle(this, ::core::mem::transmute_copy(&nthandle), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        unsafe extern "system" fn OpenSharedHandleByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, access: u32, pnthandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenSharedHandleByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&access)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnthandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MakeResident<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeResident(this, ::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects)).into())
        }
        unsafe extern "system" fn Evict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Evict(this, ::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects)).into())
        }
        unsafe extern "system" fn CreateFence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFence(this, ::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppfence)).into())
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceRemovedReason(this).into())
        }
        unsafe extern "system" fn GetCopyableFootprints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCopyableFootprints(this, ::core::mem::transmute_copy(&presourcedesc), ::core::mem::transmute_copy(&firstsubresource), ::core::mem::transmute_copy(&numsubresources), ::core::mem::transmute_copy(&baseoffset), ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes)))
        }
        unsafe extern "system" fn CreateQueryHeap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateQueryHeap(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into())
        }
        unsafe extern "system" fn SetStablePowerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStablePowerState(this, ::core::mem::transmute_copy(&enable)).into())
        }
        unsafe extern "system" fn CreateCommandSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandSignature(this, ::core::mem::transmute_copy(&pdesc), ::windows_core::from_raw_borrowed(&prootsignature), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcommandsignature)).into())
        }
        unsafe extern "system" fn GetResourceTiling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceTiling(this, ::windows_core::from_raw_borrowed(&ptiledresource), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), ::core::mem::transmute_copy(&pnumsubresourcetilings), ::core::mem::transmute_copy(&firstsubresourcetilingtoget), ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips)))
        }
        unsafe extern "system" fn GetAdapterLuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetAdapterLuid(this))
        }
        ID3D12Device_Vtbl {
            base__: <ID3D12Object as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNodeCount: GetNodeCount::<Identity, Impl, OFFSET>,
            CreateCommandQueue: CreateCommandQueue::<Identity, Impl, OFFSET>,
            CreateCommandAllocator: CreateCommandAllocator::<Identity, Impl, OFFSET>,
            CreateGraphicsPipelineState: CreateGraphicsPipelineState::<Identity, Impl, OFFSET>,
            CreateComputePipelineState: CreateComputePipelineState::<Identity, Impl, OFFSET>,
            CreateCommandList: CreateCommandList::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            CreateDescriptorHeap: CreateDescriptorHeap::<Identity, Impl, OFFSET>,
            GetDescriptorHandleIncrementSize: GetDescriptorHandleIncrementSize::<Identity, Impl, OFFSET>,
            CreateRootSignature: CreateRootSignature::<Identity, Impl, OFFSET>,
            CreateConstantBufferView: CreateConstantBufferView::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateSampler: CreateSampler::<Identity, Impl, OFFSET>,
            CopyDescriptors: CopyDescriptors::<Identity, Impl, OFFSET>,
            CopyDescriptorsSimple: CopyDescriptorsSimple::<Identity, Impl, OFFSET>,
            GetResourceAllocationInfo: GetResourceAllocationInfo::<Identity, Impl, OFFSET>,
            GetCustomHeapProperties: GetCustomHeapProperties::<Identity, Impl, OFFSET>,
            CreateCommittedResource: CreateCommittedResource::<Identity, Impl, OFFSET>,
            CreateHeap: CreateHeap::<Identity, Impl, OFFSET>,
            CreatePlacedResource: CreatePlacedResource::<Identity, Impl, OFFSET>,
            CreateReservedResource: CreateReservedResource::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandleByName: OpenSharedHandleByName::<Identity, Impl, OFFSET>,
            MakeResident: MakeResident::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            CreateFence: CreateFence::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetCopyableFootprints: GetCopyableFootprints::<Identity, Impl, OFFSET>,
            CreateQueryHeap: CreateQueryHeap::<Identity, Impl, OFFSET>,
            SetStablePowerState: SetStablePowerState::<Identity, Impl, OFFSET>,
            CreateCommandSignature: CreateCommandSignature::<Identity, Impl, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, Impl, OFFSET>,
            GetAdapterLuid: GetAdapterLuid::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device1_Impl: ::windows_core::BaseImpl + ID3D12Device_Impl {
    fn CreatePipelineLibrary(this: &Self::This, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows_core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetEventOnMultipleFenceCompletion(this: &Self::This, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetResidencyPriority(this: &Self::This, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePipelineLibrary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows_core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePipelineLibrary(this, ::core::mem::transmute_copy(&plibraryblob), ::core::mem::transmute_copy(&bloblength), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinelibrary)).into())
        }
        unsafe extern "system" fn SetEventOnMultipleFenceCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfences: *const *mut ::core::ffi::c_void, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventOnMultipleFenceCompletion(this, ::core::mem::transmute_copy(&ppfences), ::core::mem::transmute_copy(&pfencevalues), ::core::mem::transmute_copy(&numfences), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&hevent)).into())
        }
        unsafe extern "system" fn SetResidencyPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResidencyPriority(this, ::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects), ::core::mem::transmute_copy(&ppriorities)).into())
        }
        ID3D12Device1_Vtbl {
            base__: <ID3D12Device as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePipelineLibrary: CreatePipelineLibrary::<Identity, Impl, OFFSET>,
            SetEventOnMultipleFenceCompletion: SetEventOnMultipleFenceCompletion::<Identity, Impl, OFFSET>,
            SetResidencyPriority: SetResidencyPriority::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device10_Impl: ::windows_core::BaseImpl + ID3D12Device9_Impl {
    fn CreateCommittedResource3(this: &Self::This, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::core::option::Option<&ID3D12ProtectedResourceSession>, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreatePlacedResource2(this: &Self::This, pheap: ::core::option::Option<&ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateReservedResource2(this: &Self::This, pdesc: *const D3D12_RESOURCE_DESC, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::core::option::Option<&ID3D12ProtectedResourceSession>, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device10 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device9);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device10 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCommittedResource3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CreateCommittedResource3(this, ::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initiallayout), ::core::mem::transmute_copy(&poptimizedclearvalue), ::windows_core::from_raw_borrowed(&pprotectedsession), ::core::mem::transmute_copy(&numcastableformats), ::core::mem::transmute_copy(&pcastableformats), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into()
            })
        }
        unsafe extern "system" fn CreatePlacedResource2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheap: *mut ::core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePlacedResource2(this, ::windows_core::from_raw_borrowed(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initiallayout), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&numcastableformats), ::core::mem::transmute_copy(&pcastableformats), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreateReservedResource2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateReservedResource2(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initiallayout), ::core::mem::transmute_copy(&poptimizedclearvalue), ::windows_core::from_raw_borrowed(&pprotectedsession), ::core::mem::transmute_copy(&numcastableformats), ::core::mem::transmute_copy(&pcastableformats), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        ID3D12Device10_Vtbl {
            base__: <ID3D12Device9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateCommittedResource3: CreateCommittedResource3::<Identity, Impl, OFFSET>,
            CreatePlacedResource2: CreatePlacedResource2::<Identity, Impl, OFFSET>,
            CreateReservedResource2: CreateReservedResource2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device11_Impl: ::windows_core::BaseImpl + ID3D12Device10_Impl {
    fn CreateSampler2(this: &Self::This, pdesc: *const D3D12_SAMPLER_DESC2, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device11 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device10);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device11_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device11 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSampler2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device11_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC2, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSampler2(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&destdescriptor)))
        }
        ID3D12Device11_Vtbl { base__: <ID3D12Device10 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateSampler2: CreateSampler2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device12_Impl: ::windows_core::BaseImpl + ID3D12Device11_Impl {
    fn GetResourceAllocationInfo3(this: &Self::This, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, pnumcastableformats: *const u32, ppcastableformats: *const *const super::Dxgi::Common::DXGI_FORMAT, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device12 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device11);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device12_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device12 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResourceAllocationInfo3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, pnumcastableformats: *const u32, ppcastableformats: *const *const super::Dxgi::Common::DXGI_FORMAT, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetResourceAllocationInfo3(this, ::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&pnumcastableformats), ::core::mem::transmute_copy(&ppcastableformats), ::core::mem::transmute_copy(&presourceallocationinfo1)))
        }
        ID3D12Device12_Vtbl {
            base__: <ID3D12Device11 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResourceAllocationInfo3: GetResourceAllocationInfo3::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device2_Impl: ::windows_core::BaseImpl + ID3D12Device1_Impl {
    fn CreatePipelineState(this: &Self::This, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePipelineState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePipelineState(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into())
        }
        ID3D12Device2_Vtbl { base__: <ID3D12Device1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreatePipelineState: CreatePipelineState::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device3_Impl: ::windows_core::BaseImpl + ID3D12Device2_Impl {
    fn OpenExistingHeapFromAddress(this: &Self::This, paddress: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OpenExistingHeapFromFileMapping(this: &Self::This, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnqueueMakeResident(this: &Self::This, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, pfencetosignal: ::core::option::Option<&ID3D12Fence>, fencevaluetosignal: u64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenExistingHeapFromAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenExistingHeapFromAddress(this, ::core::mem::transmute_copy(&paddress), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into())
        }
        unsafe extern "system" fn OpenExistingHeapFromFileMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenExistingHeapFromFileMapping(this, ::core::mem::transmute_copy(&hfilemapping), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into())
        }
        unsafe extern "system" fn EnqueueMakeResident<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void, pfencetosignal: *mut ::core::ffi::c_void, fencevaluetosignal: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnqueueMakeResident(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numobjects), ::core::mem::transmute_copy(&ppobjects), ::windows_core::from_raw_borrowed(&pfencetosignal), ::core::mem::transmute_copy(&fencevaluetosignal)).into())
        }
        ID3D12Device3_Vtbl {
            base__: <ID3D12Device2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenExistingHeapFromAddress: OpenExistingHeapFromAddress::<Identity, Impl, OFFSET>,
            OpenExistingHeapFromFileMapping: OpenExistingHeapFromFileMapping::<Identity, Impl, OFFSET>,
            EnqueueMakeResident: EnqueueMakeResident::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device4_Impl: ::windows_core::BaseImpl + ID3D12Device3_Impl {
    fn CreateCommandList1(this: &Self::This, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows_core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateProtectedResourceSession(this: &Self::This, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows_core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateCommittedResource1(this: &Self::This, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::core::option::Option<&ID3D12ProtectedResourceSession>, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateHeap1(this: &Self::This, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: ::core::option::Option<&ID3D12ProtectedResourceSession>, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateReservedResource1(this: &Self::This, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::core::option::Option<&ID3D12ProtectedResourceSession>, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetResourceAllocationInfo1(this: &Self::This, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCommandList1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows_core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandList1(this, ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandlist)).into())
        }
        unsafe extern "system" fn CreateProtectedResourceSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows_core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateProtectedResourceSession(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsession)).into())
        }
        unsafe extern "system" fn CreateCommittedResource1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommittedResource1(this, ::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::windows_core::from_raw_borrowed(&pprotectedsession), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreateHeap1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateHeap1(this, ::core::mem::transmute_copy(&pdesc), ::windows_core::from_raw_borrowed(&pprotectedsession), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvheap)).into())
        }
        unsafe extern "system" fn CreateReservedResource1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateReservedResource1(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::windows_core::from_raw_borrowed(&pprotectedsession), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn GetResourceAllocationInfo1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetResourceAllocationInfo1(this, ::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&presourceallocationinfo1)))
        }
        ID3D12Device4_Vtbl {
            base__: <ID3D12Device3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateCommandList1: CreateCommandList1::<Identity, Impl, OFFSET>,
            CreateProtectedResourceSession: CreateProtectedResourceSession::<Identity, Impl, OFFSET>,
            CreateCommittedResource1: CreateCommittedResource1::<Identity, Impl, OFFSET>,
            CreateHeap1: CreateHeap1::<Identity, Impl, OFFSET>,
            CreateReservedResource1: CreateReservedResource1::<Identity, Impl, OFFSET>,
            GetResourceAllocationInfo1: GetResourceAllocationInfo1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device5_Impl: ::windows_core::BaseImpl + ID3D12Device4_Impl {
    fn CreateLifetimeTracker(this: &Self::This, powner: ::core::option::Option<&ID3D12LifetimeOwner>, riid: *const ::windows_core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RemoveDevice(this: &Self::This);
    fn EnumerateMetaCommands(this: &Self::This, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows_core::Result<()>;
    fn EnumerateMetaCommandParameters(this: &Self::This, commandid: *const ::windows_core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn CreateMetaCommand(this: &Self::This, commandid: *const ::windows_core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows_core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateStateObject(this: &Self::This, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows_core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetRaytracingAccelerationStructurePrebuildInfo(this: &Self::This, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO);
    fn CheckDriverMatchingIdentifier(this: &Self::This, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLifetimeTracker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, powner: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateLifetimeTracker(this, ::windows_core::from_raw_borrowed(&powner), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtracker)).into())
        }
        unsafe extern "system" fn RemoveDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveDevice(this))
        }
        unsafe extern "system" fn EnumerateMetaCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateMetaCommands(this, ::core::mem::transmute_copy(&pnummetacommands), ::core::mem::transmute_copy(&pdescs)).into())
        }
        unsafe extern "system" fn EnumerateMetaCommandParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows_core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateMetaCommandParameters(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&ptotalstructuresizeinbytes), ::core::mem::transmute_copy(&pparametercount), ::core::mem::transmute_copy(&pparameterdescs)).into())
        }
        unsafe extern "system" fn CreateMetaCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: *const ::windows_core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows_core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateMetaCommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&nodemask), ::core::mem::transmute_copy(&pcreationparametersdata), ::core::mem::transmute_copy(&creationparametersdatasizeinbytes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppmetacommand)).into())
        }
        unsafe extern "system" fn CreateStateObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows_core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateStateObject(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstateobject)).into())
        }
        unsafe extern "system" fn GetRaytracingAccelerationStructurePrebuildInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRaytracingAccelerationStructurePrebuildInfo(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinfo)))
        }
        unsafe extern "system" fn CheckDriverMatchingIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDriverMatchingIdentifier(this, ::core::mem::transmute_copy(&serializeddatatype), ::core::mem::transmute_copy(&pidentifiertocheck)))
        }
        ID3D12Device5_Vtbl {
            base__: <ID3D12Device4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLifetimeTracker: CreateLifetimeTracker::<Identity, Impl, OFFSET>,
            RemoveDevice: RemoveDevice::<Identity, Impl, OFFSET>,
            EnumerateMetaCommands: EnumerateMetaCommands::<Identity, Impl, OFFSET>,
            EnumerateMetaCommandParameters: EnumerateMetaCommandParameters::<Identity, Impl, OFFSET>,
            CreateMetaCommand: CreateMetaCommand::<Identity, Impl, OFFSET>,
            CreateStateObject: CreateStateObject::<Identity, Impl, OFFSET>,
            GetRaytracingAccelerationStructurePrebuildInfo: GetRaytracingAccelerationStructurePrebuildInfo::<Identity, Impl, OFFSET>,
            CheckDriverMatchingIdentifier: CheckDriverMatchingIdentifier::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device6_Impl: ::windows_core::BaseImpl + ID3D12Device5_Impl {
    fn SetBackgroundProcessingMode(this: &Self::This, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBackgroundProcessingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackgroundProcessingMode(this, ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&measurementsaction), ::core::mem::transmute_copy(&heventtosignaluponcompletion), ::core::mem::transmute_copy(&pbfurthermeasurementsdesired)).into())
        }
        ID3D12Device6_Vtbl {
            base__: <ID3D12Device5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBackgroundProcessingMode: SetBackgroundProcessingMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device7_Impl: ::windows_core::BaseImpl + ID3D12Device6_Impl {
    fn AddToStateObject(this: &Self::This, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: ::core::option::Option<&ID3D12StateObject>, riid: *const ::windows_core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateProtectedResourceSession1(this: &Self::This, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows_core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device6);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddToStateObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToStateObject(this, ::core::mem::transmute_copy(&paddition), ::windows_core::from_raw_borrowed(&pstateobjecttogrowfrom), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewstateobject)).into())
        }
        unsafe extern "system" fn CreateProtectedResourceSession1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows_core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateProtectedResourceSession1(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsession)).into())
        }
        ID3D12Device7_Vtbl {
            base__: <ID3D12Device6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddToStateObject: AddToStateObject::<Identity, Impl, OFFSET>,
            CreateProtectedResourceSession1: CreateProtectedResourceSession1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device8_Impl: ::windows_core::BaseImpl + ID3D12Device7_Impl {
    fn GetResourceAllocationInfo2(this: &Self::This, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
    fn CreateCommittedResource2(this: &Self::This, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: ::core::option::Option<&ID3D12ProtectedResourceSession>, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreatePlacedResource1(this: &Self::This, pheap: ::core::option::Option<&ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateSamplerFeedbackUnorderedAccessView(this: &Self::This, ptargetedresource: ::core::option::Option<&ID3D12Resource>, pfeedbackresource: ::core::option::Option<&ID3D12Resource>, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn GetCopyableFootprints1(this: &Self::This, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device7);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResourceAllocationInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetResourceAllocationInfo2(this, ::core::mem::transmute_copy(&visiblemask), ::core::mem::transmute_copy(&numresourcedescs), ::core::mem::transmute_copy(&presourcedescs), ::core::mem::transmute_copy(&presourceallocationinfo1)))
        }
        unsafe extern "system" fn CreateCommittedResource2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommittedResource2(this, ::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&heapflags), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialresourcestate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::windows_core::from_raw_borrowed(&pprotectedsession), ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreatePlacedResource1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheap: *mut ::core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePlacedResource1(this, ::windows_core::from_raw_borrowed(&pheap), ::core::mem::transmute_copy(&heapoffset), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&initialstate), ::core::mem::transmute_copy(&poptimizedclearvalue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource)).into())
        }
        unsafe extern "system" fn CreateSamplerFeedbackUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetedresource: *mut ::core::ffi::c_void, pfeedbackresource: *mut ::core::ffi::c_void, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSamplerFeedbackUnorderedAccessView(this, ::windows_core::from_raw_borrowed(&ptargetedresource), ::windows_core::from_raw_borrowed(&pfeedbackresource), ::core::mem::transmute(&destdescriptor)))
        }
        unsafe extern "system" fn GetCopyableFootprints1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCopyableFootprints1(this, ::core::mem::transmute_copy(&presourcedesc), ::core::mem::transmute_copy(&firstsubresource), ::core::mem::transmute_copy(&numsubresources), ::core::mem::transmute_copy(&baseoffset), ::core::mem::transmute_copy(&playouts), ::core::mem::transmute_copy(&pnumrows), ::core::mem::transmute_copy(&prowsizeinbytes), ::core::mem::transmute_copy(&ptotalbytes)))
        }
        ID3D12Device8_Vtbl {
            base__: <ID3D12Device7 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResourceAllocationInfo2: GetResourceAllocationInfo2::<Identity, Impl, OFFSET>,
            CreateCommittedResource2: CreateCommittedResource2::<Identity, Impl, OFFSET>,
            CreatePlacedResource1: CreatePlacedResource1::<Identity, Impl, OFFSET>,
            CreateSamplerFeedbackUnorderedAccessView: CreateSamplerFeedbackUnorderedAccessView::<Identity, Impl, OFFSET>,
            GetCopyableFootprints1: GetCopyableFootprints1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device9_Impl: ::windows_core::BaseImpl + ID3D12Device8_Impl {
    fn CreateShaderCacheSession(this: &Self::This, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows_core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ShaderCacheControl(this: &Self::This, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows_core::Result<()>;
    fn CreateCommandQueue1(this: &Self::This, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D12Device9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Device8);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Device9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateShaderCacheSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows_core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateShaderCacheSession(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsession)).into())
        }
        unsafe extern "system" fn ShaderCacheControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShaderCacheControl(this, ::core::mem::transmute_copy(&kinds), ::core::mem::transmute_copy(&control)).into())
        }
        unsafe extern "system" fn CreateCommandQueue1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandQueue1(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&creatorid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcommandqueue)).into())
        }
        ID3D12Device9_Vtbl {
            base__: <ID3D12Device8 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateShaderCacheSession: CreateShaderCacheSession::<Identity, Impl, OFFSET>,
            ShaderCacheControl: ShaderCacheControl::<Identity, Impl, OFFSET>,
            CreateCommandQueue1: CreateCommandQueue1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DeviceChild_Impl: ::windows_core::BaseImpl + ID3D12Object_Impl {
    fn GetDevice(this: &Self::This, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12DeviceChild {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Object);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceChild_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceChild {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into())
        }
        ID3D12DeviceChild_Vtbl { base__: <ID3D12Object as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12DeviceConfiguration_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This) -> D3D12_DEVICE_CONFIGURATION_DESC;
    fn GetEnabledExperimentalFeatures(this: &Self::This, pguids: *mut ::windows_core::GUID, numguids: u32) -> ::windows_core::Result<()>;
    fn SerializeVersionedRootSignature(this: &Self::This, pdesc: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppresult: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperror: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows_core::Result<()>;
    fn CreateVersionedRootSignatureDeserializer(this: &Self::This, pblob: *const ::core::ffi::c_void, size: usize, riid: *const ::windows_core::GUID, ppvdeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for ID3D12DeviceConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_DEVICE_CONFIGURATION_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        unsafe extern "system" fn GetEnabledExperimentalFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguids: *mut ::windows_core::GUID, numguids: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEnabledExperimentalFeatures(this, ::core::mem::transmute_copy(&pguids), ::core::mem::transmute_copy(&numguids)).into())
        }
        unsafe extern "system" fn SerializeVersionedRootSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppresult: *mut *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SerializeVersionedRootSignature(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppresult), ::core::mem::transmute_copy(&pperror)).into())
        }
        unsafe extern "system" fn CreateVersionedRootSignatureDeserializer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *const ::core::ffi::c_void, size: usize, riid: *const ::windows_core::GUID, ppvdeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVersionedRootSignatureDeserializer(this, ::core::mem::transmute_copy(&pblob), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdeserializer)).into())
        }
        ID3D12DeviceConfiguration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetEnabledExperimentalFeatures: GetEnabledExperimentalFeatures::<Identity, Impl, OFFSET>,
            SerializeVersionedRootSignature: SerializeVersionedRootSignature::<Identity, Impl, OFFSET>,
            CreateVersionedRootSignatureDeserializer: CreateVersionedRootSignatureDeserializer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12DeviceFactory_Impl: ::windows_core::BaseImpl {
    fn InitializeFromGlobalState(this: &Self::This) -> ::windows_core::Result<()>;
    fn ApplyToGlobalState(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetFlags(this: &Self::This, flags: D3D12_DEVICE_FACTORY_FLAGS) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This) -> D3D12_DEVICE_FACTORY_FLAGS;
    fn GetConfigurationInterface(this: &Self::This, clsid: *const ::windows_core::GUID, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnableExperimentalFeatures(this: &Self::This, numfeatures: u32, piids: *const ::windows_core::GUID, pconfigurationstructs: *const ::core::ffi::c_void, pconfigurationstructsizes: *const u32) -> ::windows_core::Result<()>;
    fn CreateDevice(this: &Self::This, adapter: ::core::option::Option<&::windows_core::IUnknown>, featurelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for ID3D12DeviceFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFromGlobalState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromGlobalState(this).into())
        }
        unsafe extern "system" fn ApplyToGlobalState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyToGlobalState(this).into())
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D12_DEVICE_FACTORY_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_DEVICE_FACTORY_FLAGS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlags(this))
        }
        unsafe extern "system" fn GetConfigurationInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConfigurationInterface(this, ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn EnableExperimentalFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numfeatures: u32, piids: *const ::windows_core::GUID, pconfigurationstructs: *const ::core::ffi::c_void, pconfigurationstructsizes: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableExperimentalFeatures(this, ::core::mem::transmute_copy(&numfeatures), ::core::mem::transmute_copy(&piids), ::core::mem::transmute_copy(&pconfigurationstructs), ::core::mem::transmute_copy(&pconfigurationstructsizes)).into())
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, featurelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::windows_core::from_raw_borrowed(&adapter), ::core::mem::transmute_copy(&featurelevel), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into())
        }
        ID3D12DeviceFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFromGlobalState: InitializeFromGlobalState::<Identity, Impl, OFFSET>,
            ApplyToGlobalState: ApplyToGlobalState::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetConfigurationInterface: GetConfigurationInterface::<Identity, Impl, OFFSET>,
            EnableExperimentalFeatures: EnableExperimentalFeatures::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DeviceRemovedExtendedData_Impl: ::windows_core::BaseImpl {
    fn GetAutoBreadcrumbsOutput(this: &Self::This) -> ::windows_core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT>;
    fn GetPageFaultAllocationOutput(this: &Self::This) -> ::windows_core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT>;
}
impl ::windows_core::Iids for ID3D12DeviceRemovedExtendedData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceRemovedExtendedData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutoBreadcrumbsOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPageFaultAllocationOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D12DeviceRemovedExtendedData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAutoBreadcrumbsOutput: GetAutoBreadcrumbsOutput::<Identity, Impl, OFFSET>,
            GetPageFaultAllocationOutput: GetPageFaultAllocationOutput::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DeviceRemovedExtendedData1_Impl: ::windows_core::BaseImpl + ID3D12DeviceRemovedExtendedData_Impl {
    fn GetAutoBreadcrumbsOutput1(this: &Self::This) -> ::windows_core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1>;
    fn GetPageFaultAllocationOutput1(this: &Self::This) -> ::windows_core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT1>;
}
impl ::windows_core::Iids for ID3D12DeviceRemovedExtendedData1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceRemovedExtendedData);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceRemovedExtendedData1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutoBreadcrumbsOutput1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPageFaultAllocationOutput1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D12DeviceRemovedExtendedData1_Vtbl {
            base__: <ID3D12DeviceRemovedExtendedData as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAutoBreadcrumbsOutput1: GetAutoBreadcrumbsOutput1::<Identity, Impl, OFFSET>,
            GetPageFaultAllocationOutput1: GetPageFaultAllocationOutput1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DeviceRemovedExtendedData2_Impl: ::windows_core::BaseImpl + ID3D12DeviceRemovedExtendedData1_Impl {
    fn GetPageFaultAllocationOutput2(this: &Self::This, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows_core::Result<()>;
    fn GetDeviceState(this: &Self::This) -> D3D12_DRED_DEVICE_STATE;
}
impl ::windows_core::Iids for ID3D12DeviceRemovedExtendedData2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceRemovedExtendedData1);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceRemovedExtendedData2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPageFaultAllocationOutput2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPageFaultAllocationOutput2(this, ::core::mem::transmute_copy(&poutput)).into())
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceState(this))
        }
        ID3D12DeviceRemovedExtendedData2_Vtbl {
            base__: <ID3D12DeviceRemovedExtendedData1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPageFaultAllocationOutput2: GetPageFaultAllocationOutput2::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DeviceRemovedExtendedDataSettings_Impl: ::windows_core::BaseImpl {
    fn SetAutoBreadcrumbsEnablement(this: &Self::This, enablement: D3D12_DRED_ENABLEMENT);
    fn SetPageFaultEnablement(this: &Self::This, enablement: D3D12_DRED_ENABLEMENT);
    fn SetWatsonDumpEnablement(this: &Self::This, enablement: D3D12_DRED_ENABLEMENT);
}
impl ::windows_core::Iids for ID3D12DeviceRemovedExtendedDataSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceRemovedExtendedDataSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAutoBreadcrumbsEnablement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoBreadcrumbsEnablement(this, ::core::mem::transmute_copy(&enablement)))
        }
        unsafe extern "system" fn SetPageFaultEnablement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPageFaultEnablement(this, ::core::mem::transmute_copy(&enablement)))
        }
        unsafe extern "system" fn SetWatsonDumpEnablement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWatsonDumpEnablement(this, ::core::mem::transmute_copy(&enablement)))
        }
        ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAutoBreadcrumbsEnablement: SetAutoBreadcrumbsEnablement::<Identity, Impl, OFFSET>,
            SetPageFaultEnablement: SetPageFaultEnablement::<Identity, Impl, OFFSET>,
            SetWatsonDumpEnablement: SetWatsonDumpEnablement::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12DeviceRemovedExtendedDataSettings1_Impl: ::windows_core::BaseImpl + ID3D12DeviceRemovedExtendedDataSettings_Impl {
    fn SetBreadcrumbContextEnablement(this: &Self::This, enablement: D3D12_DRED_ENABLEMENT);
}
impl ::windows_core::Iids for ID3D12DeviceRemovedExtendedDataSettings1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceRemovedExtendedDataSettings);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceRemovedExtendedDataSettings1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBreadcrumbContextEnablement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreadcrumbContextEnablement(this, ::core::mem::transmute_copy(&enablement)))
        }
        ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
            base__: <ID3D12DeviceRemovedExtendedDataSettings as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBreadcrumbContextEnablement: SetBreadcrumbContextEnablement::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12DeviceRemovedExtendedDataSettings2_Impl: ::windows_core::BaseImpl + ID3D12DeviceRemovedExtendedDataSettings1_Impl {
    fn UseMarkersOnlyAutoBreadcrumbs(this: &Self::This, markersonly: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12DeviceRemovedExtendedDataSettings2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceRemovedExtendedDataSettings1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12DeviceRemovedExtendedDataSettings2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UseMarkersOnlyAutoBreadcrumbs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, markersonly: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseMarkersOnlyAutoBreadcrumbs(this, ::core::mem::transmute_copy(&markersonly)))
        }
        ID3D12DeviceRemovedExtendedDataSettings2_Vtbl {
            base__: <ID3D12DeviceRemovedExtendedDataSettings1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UseMarkersOnlyAutoBreadcrumbs: UseMarkersOnlyAutoBreadcrumbs::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Fence_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn GetCompletedValue(this: &Self::This) -> u64;
    fn SetEventOnCompletion(this: &Self::This, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn Signal(this: &Self::This, value: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Fence {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Fence {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCompletedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompletedValue(this))
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventOnCompletion(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&hevent)).into())
        }
        unsafe extern "system" fn Signal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Signal(this, ::core::mem::transmute_copy(&value)).into())
        }
        ID3D12Fence_Vtbl {
            base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCompletedValue: GetCompletedValue::<Identity, Impl, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Fence1_Impl: ::windows_core::BaseImpl + ID3D12Fence_Impl {
    fn GetCreationFlags(this: &Self::This) -> D3D12_FENCE_FLAGS;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Fence1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Fence);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Fence1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Fence1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Fence1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_FENCE_FLAGS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationFlags(this))
        }
        ID3D12Fence1_Vtbl { base__: <ID3D12Fence as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12FunctionParameterReflection_Vtbl {
    pub const fn new<Impl: ID3D12FunctionParameterReflection_Impl>() -> ID3D12FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionParameterReflection_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        Self { GetDesc: GetDesc::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12FunctionParameterReflection_ImplVtbl<T: ID3D12FunctionParameterReflection_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12FunctionParameterReflection_Impl> ID3D12FunctionParameterReflection_ImplVtbl<T> {
    const VTABLE: ID3D12FunctionParameterReflection_Vtbl = ID3D12FunctionParameterReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12FunctionParameterReflection {
    pub fn new<'a, T: ID3D12FunctionParameterReflection_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D12FunctionParameterReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12FunctionReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &::windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12FunctionReflection_Vtbl {
    pub const fn new<Impl: ID3D12FunctionReflection_Impl>() -> ID3D12FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetResourceBindingDesc(this, ::core::mem::transmute_copy(&resourceindex), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetResourceBindingDescByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D12FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetFunctionParameter(this, ::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl>,
            GetVariableByName: GetVariableByName::<Impl>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl>,
            GetFunctionParameter: GetFunctionParameter::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
struct ID3D12FunctionReflection_ImplVtbl<T: ID3D12FunctionReflection_Impl>(::std::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<T: ID3D12FunctionReflection_Impl> ID3D12FunctionReflection_ImplVtbl<T> {
    const VTABLE: ID3D12FunctionReflection_Vtbl = ID3D12FunctionReflection_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D12FunctionReflection {
    pub fn new<'a, T: ID3D12FunctionReflection_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D12FunctionReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList_Impl: ::windows_core::BaseImpl + ID3D12CommandList_Impl {
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This, pallocator: ::core::option::Option<&ID3D12CommandAllocator>, pinitialstate: ::core::option::Option<&ID3D12PipelineState>) -> ::windows_core::Result<()>;
    fn ClearState(this: &Self::This, ppipelinestate: ::core::option::Option<&ID3D12PipelineState>);
    fn DrawInstanced(this: &Self::This, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn DrawIndexedInstanced(this: &Self::This, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn Dispatch(this: &Self::This, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn CopyBufferRegion(this: &Self::This, pdstbuffer: ::core::option::Option<&ID3D12Resource>, dstoffset: u64, psrcbuffer: ::core::option::Option<&ID3D12Resource>, srcoffset: u64, numbytes: u64);
    fn CopyTextureRegion(this: &Self::This, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX);
    fn CopyResource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D12Resource>, psrcresource: ::core::option::Option<&ID3D12Resource>);
    fn CopyTiles(this: &Self::This, ptiledresource: ::core::option::Option<&ID3D12Resource>, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: ::core::option::Option<&ID3D12Resource>, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS);
    fn ResolveSubresource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D12Resource>, dstsubresource: u32, psrcresource: ::core::option::Option<&ID3D12Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn IASetPrimitiveTopology(this: &Self::This, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn RSSetViewports(this: &Self::This, numviewports: u32, pviewports: *const D3D12_VIEWPORT);
    fn RSSetScissorRects(this: &Self::This, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn OMSetBlendFactor(this: &Self::This, blendfactor: *const f32);
    fn OMSetStencilRef(this: &Self::This, stencilref: u32);
    fn SetPipelineState(this: &Self::This, ppipelinestate: ::core::option::Option<&ID3D12PipelineState>);
    fn ResourceBarrier(this: &Self::This, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER);
    fn ExecuteBundle(this: &Self::This, pcommandlist: ::core::option::Option<&ID3D12GraphicsCommandList>);
    fn SetDescriptorHeaps(this: &Self::This, numdescriptorheaps: u32, ppdescriptorheaps: *const ::core::option::Option<ID3D12DescriptorHeap>);
    fn SetComputeRootSignature(this: &Self::This, prootsignature: ::core::option::Option<&ID3D12RootSignature>);
    fn SetGraphicsRootSignature(this: &Self::This, prootsignature: ::core::option::Option<&ID3D12RootSignature>);
    fn SetComputeRootDescriptorTable(this: &Self::This, rootparameterindex: u32, basedescriptor: &D3D12_GPU_DESCRIPTOR_HANDLE);
    fn SetGraphicsRootDescriptorTable(this: &Self::This, rootparameterindex: u32, basedescriptor: &D3D12_GPU_DESCRIPTOR_HANDLE);
    fn SetComputeRoot32BitConstant(this: &Self::This, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32);
    fn SetGraphicsRoot32BitConstant(this: &Self::This, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32);
    fn SetComputeRoot32BitConstants(this: &Self::This, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32);
    fn SetGraphicsRoot32BitConstants(this: &Self::This, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32);
    fn SetComputeRootConstantBufferView(this: &Self::This, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootConstantBufferView(this: &Self::This, rootparameterindex: u32, bufferlocation: u64);
    fn SetComputeRootShaderResourceView(this: &Self::This, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootShaderResourceView(this: &Self::This, rootparameterindex: u32, bufferlocation: u64);
    fn SetComputeRootUnorderedAccessView(this: &Self::This, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootUnorderedAccessView(this: &Self::This, rootparameterindex: u32, bufferlocation: u64);
    fn IASetIndexBuffer(this: &Self::This, pview: *const D3D12_INDEX_BUFFER_VIEW);
    fn IASetVertexBuffers(this: &Self::This, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW);
    fn SOSetTargets(this: &Self::This, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW);
    fn OMSetRenderTargets(this: &Self::This, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE);
    fn ClearDepthStencilView(this: &Self::This, depthstencilview: &D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearRenderTargetView(this: &Self::This, rendertargetview: &D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearUnorderedAccessViewUint(this: &Self::This, viewgpuhandleincurrentheap: &D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: &D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::core::option::Option<&ID3D12Resource>, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearUnorderedAccessViewFloat(this: &Self::This, viewgpuhandleincurrentheap: &D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: &D3D12_CPU_DESCRIPTOR_HANDLE, presource: ::core::option::Option<&ID3D12Resource>, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn DiscardResource(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, pregion: *const D3D12_DISCARD_REGION);
    fn BeginQuery(this: &Self::This, pqueryheap: ::core::option::Option<&ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, index: u32);
    fn EndQuery(this: &Self::This, pqueryheap: ::core::option::Option<&ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, index: u32);
    fn ResolveQueryData(this: &Self::This, pqueryheap: ::core::option::Option<&ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: ::core::option::Option<&ID3D12Resource>, aligneddestinationbufferoffset: u64);
    fn SetPredication(this: &Self::This, pbuffer: ::core::option::Option<&ID3D12Resource>, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP);
    fn SetMarker(this: &Self::This, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn BeginEvent(this: &Self::This, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32);
    fn EndEvent(this: &Self::This);
    fn ExecuteIndirect(this: &Self::This, pcommandsignature: ::core::option::Option<&ID3D12CommandSignature>, maxcommandcount: u32, pargumentbuffer: ::core::option::Option<&ID3D12Resource>, argumentbufferoffset: u64, pcountbuffer: ::core::option::Option<&ID3D12Resource>, countbufferoffset: u64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12CommandList);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallocator: *mut ::core::ffi::c_void, pinitialstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this, ::windows_core::from_raw_borrowed(&pallocator), ::windows_core::from_raw_borrowed(&pinitialstate)).into())
        }
        unsafe extern "system" fn ClearState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppipelinestate: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearState(this, ::windows_core::from_raw_borrowed(&ppipelinestate)))
        }
        unsafe extern "system" fn DrawInstanced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInstanced(this, ::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation)))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexedInstanced(this, ::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation)))
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dispatch(this, ::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz)))
        }
        unsafe extern "system" fn CopyBufferRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut ::core::ffi::c_void, srcoffset: u64, numbytes: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyBufferRegion(this, ::windows_core::from_raw_borrowed(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::windows_core::from_raw_borrowed(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&numbytes)))
        }
        unsafe extern "system" fn CopyTextureRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTextureRegion(this, ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&psrcbox)))
        }
        unsafe extern "system" fn CopyResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, psrcresource: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyResource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::windows_core::from_raw_borrowed(&psrcresource)))
        }
        unsafe extern "system" fn CopyTiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: *mut ::core::ffi::c_void, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTiles(this, ::windows_core::from_raw_borrowed(&ptiledresource), ::core::mem::transmute_copy(&ptileregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::windows_core::from_raw_borrowed(&pbuffer), ::core::mem::transmute_copy(&bufferstartoffsetinbytes), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveSubresource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format)))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetPrimitiveTopology(this, ::core::mem::transmute_copy(&primitivetopology)))
        }
        unsafe extern "system" fn RSSetViewports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D12_VIEWPORT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetViewports(this, ::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports)))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetScissorRects(this, ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn OMSetBlendFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blendfactor: *const f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetBlendFactor(this, ::core::mem::transmute_copy(&blendfactor)))
        }
        unsafe extern "system" fn OMSetStencilRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stencilref: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetStencilRef(this, ::core::mem::transmute_copy(&stencilref)))
        }
        unsafe extern "system" fn SetPipelineState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppipelinestate: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPipelineState(this, ::windows_core::from_raw_borrowed(&ppipelinestate)))
        }
        unsafe extern "system" fn ResourceBarrier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResourceBarrier(this, ::core::mem::transmute_copy(&numbarriers), ::core::mem::transmute_copy(&pbarriers)))
        }
        unsafe extern "system" fn ExecuteBundle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandlist: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteBundle(this, ::windows_core::from_raw_borrowed(&pcommandlist)))
        }
        unsafe extern "system" fn SetDescriptorHeaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numdescriptorheaps: u32, ppdescriptorheaps: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescriptorHeaps(this, ::core::mem::transmute_copy(&numdescriptorheaps), ::core::mem::transmute_copy(&ppdescriptorheaps)))
        }
        unsafe extern "system" fn SetComputeRootSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prootsignature: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRootSignature(this, ::windows_core::from_raw_borrowed(&prootsignature)))
        }
        unsafe extern "system" fn SetGraphicsRootSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prootsignature: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRootSignature(this, ::windows_core::from_raw_borrowed(&prootsignature)))
        }
        unsafe extern "system" fn SetComputeRootDescriptorTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRootDescriptorTable(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute(&basedescriptor)))
        }
        unsafe extern "system" fn SetGraphicsRootDescriptorTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRootDescriptorTable(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute(&basedescriptor)))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRoot32BitConstant(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues)))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRoot32BitConstant(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues)))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRoot32BitConstants(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&num32bitvaluestoset), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues)))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRoot32BitConstants(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&num32bitvaluestoset), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&destoffsetin32bitvalues)))
        }
        unsafe extern "system" fn SetComputeRootConstantBufferView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRootConstantBufferView(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation)))
        }
        unsafe extern "system" fn SetGraphicsRootConstantBufferView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRootConstantBufferView(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation)))
        }
        unsafe extern "system" fn SetComputeRootShaderResourceView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRootShaderResourceView(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation)))
        }
        unsafe extern "system" fn SetGraphicsRootShaderResourceView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRootShaderResourceView(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation)))
        }
        unsafe extern "system" fn SetComputeRootUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeRootUnorderedAccessView(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation)))
        }
        unsafe extern "system" fn SetGraphicsRootUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraphicsRootUnorderedAccessView(this, ::core::mem::transmute_copy(&rootparameterindex), ::core::mem::transmute_copy(&bufferlocation)))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pview: *const D3D12_INDEX_BUFFER_VIEW) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetIndexBuffer(this, ::core::mem::transmute_copy(&pview)))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetVertexBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pviews)))
        }
        unsafe extern "system" fn SOSetTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SOSetTargets(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pviews)))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetRenderTargets(this, ::core::mem::transmute_copy(&numrendertargetdescriptors), ::core::mem::transmute_copy(&prendertargetdescriptors), ::core::mem::transmute_copy(&rtssinglehandletodescriptorrange), ::core::mem::transmute_copy(&pdepthstencildescriptor)))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearDepthStencilView(this, ::core::mem::transmute(&depthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRenderTargetView(this, ::core::mem::transmute(&rendertargetview), ::core::mem::transmute_copy(&colorrgba), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: *mut ::core::ffi::c_void, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearUnorderedAccessViewUint(this, ::core::mem::transmute(&viewgpuhandleincurrentheap), ::core::mem::transmute(&viewcpuhandle), ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: *mut ::core::ffi::c_void, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearUnorderedAccessViewFloat(this, ::core::mem::transmute(&viewgpuhandleincurrentheap), ::core::mem::transmute(&viewcpuhandle), ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn DiscardResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pregion: *const D3D12_DISCARD_REGION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardResource(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pregion)))
        }
        unsafe extern "system" fn BeginQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqueryheap: *mut ::core::ffi::c_void, r#type: D3D12_QUERY_TYPE, index: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginQuery(this, ::windows_core::from_raw_borrowed(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn EndQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqueryheap: *mut ::core::ffi::c_void, r#type: D3D12_QUERY_TYPE, index: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndQuery(this, ::windows_core::from_raw_borrowed(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn ResolveQueryData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqueryheap: *mut ::core::ffi::c_void, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: *mut ::core::ffi::c_void, aligneddestinationbufferoffset: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveQueryData(this, ::windows_core::from_raw_borrowed(&pqueryheap), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&numqueries), ::windows_core::from_raw_borrowed(&pdestinationbuffer), ::core::mem::transmute_copy(&aligneddestinationbufferoffset)))
        }
        unsafe extern "system" fn SetPredication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPredication(this, ::windows_core::from_raw_borrowed(&pbuffer), ::core::mem::transmute_copy(&alignedbufferoffset), ::core::mem::transmute_copy(&operation)))
        }
        unsafe extern "system" fn SetMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMarker(this, ::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size)))
        }
        unsafe extern "system" fn BeginEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEvent(this, ::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size)))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEvent(this))
        }
        unsafe extern "system" fn ExecuteIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandsignature: *mut ::core::ffi::c_void, maxcommandcount: u32, pargumentbuffer: *mut ::core::ffi::c_void, argumentbufferoffset: u64, pcountbuffer: *mut ::core::ffi::c_void, countbufferoffset: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteIndirect(this, ::windows_core::from_raw_borrowed(&pcommandsignature), ::core::mem::transmute_copy(&maxcommandcount), ::windows_core::from_raw_borrowed(&pargumentbuffer), ::core::mem::transmute_copy(&argumentbufferoffset), ::windows_core::from_raw_borrowed(&pcountbuffer), ::core::mem::transmute_copy(&countbufferoffset)))
        }
        ID3D12GraphicsCommandList_Vtbl {
            base__: <ID3D12CommandList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Close: Close::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
            CopyBufferRegion: CopyBufferRegion::<Identity, Impl, OFFSET>,
            CopyTextureRegion: CopyTextureRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            CopyTiles: CopyTiles::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            OMSetBlendFactor: OMSetBlendFactor::<Identity, Impl, OFFSET>,
            OMSetStencilRef: OMSetStencilRef::<Identity, Impl, OFFSET>,
            SetPipelineState: SetPipelineState::<Identity, Impl, OFFSET>,
            ResourceBarrier: ResourceBarrier::<Identity, Impl, OFFSET>,
            ExecuteBundle: ExecuteBundle::<Identity, Impl, OFFSET>,
            SetDescriptorHeaps: SetDescriptorHeaps::<Identity, Impl, OFFSET>,
            SetComputeRootSignature: SetComputeRootSignature::<Identity, Impl, OFFSET>,
            SetGraphicsRootSignature: SetGraphicsRootSignature::<Identity, Impl, OFFSET>,
            SetComputeRootDescriptorTable: SetComputeRootDescriptorTable::<Identity, Impl, OFFSET>,
            SetGraphicsRootDescriptorTable: SetGraphicsRootDescriptorTable::<Identity, Impl, OFFSET>,
            SetComputeRoot32BitConstant: SetComputeRoot32BitConstant::<Identity, Impl, OFFSET>,
            SetGraphicsRoot32BitConstant: SetGraphicsRoot32BitConstant::<Identity, Impl, OFFSET>,
            SetComputeRoot32BitConstants: SetComputeRoot32BitConstants::<Identity, Impl, OFFSET>,
            SetGraphicsRoot32BitConstants: SetGraphicsRoot32BitConstants::<Identity, Impl, OFFSET>,
            SetComputeRootConstantBufferView: SetComputeRootConstantBufferView::<Identity, Impl, OFFSET>,
            SetGraphicsRootConstantBufferView: SetGraphicsRootConstantBufferView::<Identity, Impl, OFFSET>,
            SetComputeRootShaderResourceView: SetComputeRootShaderResourceView::<Identity, Impl, OFFSET>,
            SetGraphicsRootShaderResourceView: SetGraphicsRootShaderResourceView::<Identity, Impl, OFFSET>,
            SetComputeRootUnorderedAccessView: SetComputeRootUnorderedAccessView::<Identity, Impl, OFFSET>,
            SetGraphicsRootUnorderedAccessView: SetGraphicsRootUnorderedAccessView::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, Impl, OFFSET>,
            DiscardResource: DiscardResource::<Identity, Impl, OFFSET>,
            BeginQuery: BeginQuery::<Identity, Impl, OFFSET>,
            EndQuery: EndQuery::<Identity, Impl, OFFSET>,
            ResolveQueryData: ResolveQueryData::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            ExecuteIndirect: ExecuteIndirect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList1_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList_Impl {
    fn AtomicCopyBufferUINT(this: &Self::This, pdstbuffer: ::core::option::Option<&ID3D12Resource>, dstoffset: u64, psrcbuffer: ::core::option::Option<&ID3D12Resource>, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64);
    fn AtomicCopyBufferUINT64(this: &Self::This, pdstbuffer: ::core::option::Option<&ID3D12Resource>, dstoffset: u64, psrcbuffer: ::core::option::Option<&ID3D12Resource>, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64);
    fn OMSetDepthBounds(this: &Self::This, min: f32, max: f32);
    fn SetSamplePositions(this: &Self::This, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION);
    fn ResolveSubresourceRegion(this: &Self::This, pdstresource: ::core::option::Option<&ID3D12Resource>, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: ::core::option::Option<&ID3D12Resource>, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE);
    fn SetViewInstanceMask(this: &Self::This, mask: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AtomicCopyBufferUINT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut ::core::ffi::c_void, srcoffset: u64, dependencies: u32, ppdependentresources: *const *mut ::core::ffi::c_void, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AtomicCopyBufferUINT(this, ::windows_core::from_raw_borrowed(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::windows_core::from_raw_borrowed(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&dependencies), ::core::mem::transmute_copy(&ppdependentresources), ::core::mem::transmute_copy(&pdependentsubresourceranges)))
        }
        unsafe extern "system" fn AtomicCopyBufferUINT64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut ::core::ffi::c_void, srcoffset: u64, dependencies: u32, ppdependentresources: *const *mut ::core::ffi::c_void, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AtomicCopyBufferUINT64(this, ::windows_core::from_raw_borrowed(&pdstbuffer), ::core::mem::transmute_copy(&dstoffset), ::windows_core::from_raw_borrowed(&psrcbuffer), ::core::mem::transmute_copy(&srcoffset), ::core::mem::transmute_copy(&dependencies), ::core::mem::transmute_copy(&ppdependentresources), ::core::mem::transmute_copy(&pdependentsubresourceranges)))
        }
        unsafe extern "system" fn OMSetDepthBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, min: f32, max: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetDepthBounds(this, ::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&max)))
        }
        unsafe extern "system" fn SetSamplePositions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSamplePositions(this, ::core::mem::transmute_copy(&numsamplesperpixel), ::core::mem::transmute_copy(&numpixels), ::core::mem::transmute_copy(&psamplepositions)))
        }
        unsafe extern "system" fn ResolveSubresourceRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveSubresourceRegion(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcrect), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&resolvemode)))
        }
        unsafe extern "system" fn SetViewInstanceMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewInstanceMask(this, ::core::mem::transmute_copy(&mask)))
        }
        ID3D12GraphicsCommandList1_Vtbl {
            base__: <ID3D12GraphicsCommandList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AtomicCopyBufferUINT: AtomicCopyBufferUINT::<Identity, Impl, OFFSET>,
            AtomicCopyBufferUINT64: AtomicCopyBufferUINT64::<Identity, Impl, OFFSET>,
            OMSetDepthBounds: OMSetDepthBounds::<Identity, Impl, OFFSET>,
            SetSamplePositions: SetSamplePositions::<Identity, Impl, OFFSET>,
            ResolveSubresourceRegion: ResolveSubresourceRegion::<Identity, Impl, OFFSET>,
            SetViewInstanceMask: SetViewInstanceMask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList2_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList1_Impl {
    fn WriteBufferImmediate(this: &Self::This, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteBufferImmediate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteBufferImmediate(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pmodes)))
        }
        ID3D12GraphicsCommandList2_Vtbl {
            base__: <ID3D12GraphicsCommandList1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteBufferImmediate: WriteBufferImmediate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList3_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList2_Impl {
    fn SetProtectedResourceSession(this: &Self::This, pprotectedresourcesession: ::core::option::Option<&ID3D12ProtectedResourceSession>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProtectedResourceSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotectedresourcesession: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtectedResourceSession(this, ::windows_core::from_raw_borrowed(&pprotectedresourcesession)))
        }
        ID3D12GraphicsCommandList3_Vtbl {
            base__: <ID3D12GraphicsCommandList2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProtectedResourceSession: SetProtectedResourceSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList4_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList3_Impl {
    fn BeginRenderPass(this: &Self::This, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS);
    fn EndRenderPass(this: &Self::This);
    fn InitializeMetaCommand(this: &Self::This, pmetacommand: ::core::option::Option<&ID3D12MetaCommand>, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize);
    fn ExecuteMetaCommand(this: &Self::This, pmetacommand: ::core::option::Option<&ID3D12MetaCommand>, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize);
    fn BuildRaytracingAccelerationStructure(this: &Self::This, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC);
    fn EmitRaytracingAccelerationStructurePostbuildInfo(this: &Self::This, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64);
    fn CopyRaytracingAccelerationStructure(this: &Self::This, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE);
    fn SetPipelineState1(this: &Self::This, pstateobject: ::core::option::Option<&ID3D12StateObject>);
    fn DispatchRays(this: &Self::This, pdesc: *const D3D12_DISPATCH_RAYS_DESC);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginRenderPass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginRenderPass(this, ::core::mem::transmute_copy(&numrendertargets), ::core::mem::transmute_copy(&prendertargets), ::core::mem::transmute_copy(&pdepthstencil), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn EndRenderPass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndRenderPass(this))
        }
        unsafe extern "system" fn InitializeMetaCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmetacommand: *mut ::core::ffi::c_void, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeMetaCommand(this, ::windows_core::from_raw_borrowed(&pmetacommand), ::core::mem::transmute_copy(&pinitializationparametersdata), ::core::mem::transmute_copy(&initializationparametersdatasizeinbytes)))
        }
        unsafe extern "system" fn ExecuteMetaCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmetacommand: *mut ::core::ffi::c_void, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteMetaCommand(this, ::windows_core::from_raw_borrowed(&pmetacommand), ::core::mem::transmute_copy(&pexecutionparametersdata), ::core::mem::transmute_copy(&executionparametersdatasizeinbytes)))
        }
        unsafe extern "system" fn BuildRaytracingAccelerationStructure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BuildRaytracingAccelerationStructure(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numpostbuildinfodescs), ::core::mem::transmute_copy(&ppostbuildinfodescs)))
        }
        unsafe extern "system" fn EmitRaytracingAccelerationStructurePostbuildInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmitRaytracingAccelerationStructurePostbuildInfo(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsourceaccelerationstructures), ::core::mem::transmute_copy(&psourceaccelerationstructuredata)))
        }
        unsafe extern "system" fn CopyRaytracingAccelerationStructure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyRaytracingAccelerationStructure(this, ::core::mem::transmute_copy(&destaccelerationstructuredata), ::core::mem::transmute_copy(&sourceaccelerationstructuredata), ::core::mem::transmute_copy(&mode)))
        }
        unsafe extern "system" fn SetPipelineState1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstateobject: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPipelineState1(this, ::windows_core::from_raw_borrowed(&pstateobject)))
        }
        unsafe extern "system" fn DispatchRays<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DispatchRays(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D12GraphicsCommandList4_Vtbl {
            base__: <ID3D12GraphicsCommandList3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginRenderPass: BeginRenderPass::<Identity, Impl, OFFSET>,
            EndRenderPass: EndRenderPass::<Identity, Impl, OFFSET>,
            InitializeMetaCommand: InitializeMetaCommand::<Identity, Impl, OFFSET>,
            ExecuteMetaCommand: ExecuteMetaCommand::<Identity, Impl, OFFSET>,
            BuildRaytracingAccelerationStructure: BuildRaytracingAccelerationStructure::<Identity, Impl, OFFSET>,
            EmitRaytracingAccelerationStructurePostbuildInfo: EmitRaytracingAccelerationStructurePostbuildInfo::<Identity, Impl, OFFSET>,
            CopyRaytracingAccelerationStructure: CopyRaytracingAccelerationStructure::<Identity, Impl, OFFSET>,
            SetPipelineState1: SetPipelineState1::<Identity, Impl, OFFSET>,
            DispatchRays: DispatchRays::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList5_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList4_Impl {
    fn RSSetShadingRate(this: &Self::This, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER);
    fn RSSetShadingRateImage(this: &Self::This, shadingrateimage: ::core::option::Option<&ID3D12Resource>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RSSetShadingRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetShadingRate(this, ::core::mem::transmute_copy(&baseshadingrate), ::core::mem::transmute_copy(&combiners)))
        }
        unsafe extern "system" fn RSSetShadingRateImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shadingrateimage: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetShadingRateImage(this, ::windows_core::from_raw_borrowed(&shadingrateimage)))
        }
        ID3D12GraphicsCommandList5_Vtbl {
            base__: <ID3D12GraphicsCommandList4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RSSetShadingRate: RSSetShadingRate::<Identity, Impl, OFFSET>,
            RSSetShadingRateImage: RSSetShadingRateImage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList6_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList5_Impl {
    fn DispatchMesh(this: &Self::This, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DispatchMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DispatchMesh(this, ::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz)))
        }
        ID3D12GraphicsCommandList6_Vtbl { base__: <ID3D12GraphicsCommandList5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DispatchMesh: DispatchMesh::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList7_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList6_Impl {
    fn Barrier(this: &Self::This, numbarriergroups: u32, pbarriergroups: *const D3D12_BARRIER_GROUP);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList6);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Barrier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numbarriergroups: u32, pbarriergroups: *const D3D12_BARRIER_GROUP) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Barrier(this, ::core::mem::transmute_copy(&numbarriergroups), ::core::mem::transmute_copy(&pbarriergroups)))
        }
        ID3D12GraphicsCommandList7_Vtbl { base__: <ID3D12GraphicsCommandList6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Barrier: Barrier::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList8_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList7_Impl {
    fn OMSetFrontAndBackStencilRef(this: &Self::This, frontstencilref: u32, backstencilref: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList7);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OMSetFrontAndBackStencilRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frontstencilref: u32, backstencilref: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetFrontAndBackStencilRef(this, ::core::mem::transmute_copy(&frontstencilref), ::core::mem::transmute_copy(&backstencilref)))
        }
        ID3D12GraphicsCommandList8_Vtbl {
            base__: <ID3D12GraphicsCommandList7 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OMSetFrontAndBackStencilRef: OMSetFrontAndBackStencilRef::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList9_Impl: ::windows_core::BaseImpl + ID3D12GraphicsCommandList8_Impl {
    fn RSSetDepthBias(this: &Self::This, depthbias: f32, depthbiasclamp: f32, slopescaleddepthbias: f32);
    fn IASetIndexBufferStripCutValue(this: &Self::This, ibstripcutvalue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12GraphicsCommandList9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12GraphicsCommandList8);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12GraphicsCommandList9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RSSetDepthBias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, depthbias: f32, depthbiasclamp: f32, slopescaleddepthbias: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetDepthBias(this, ::core::mem::transmute_copy(&depthbias), ::core::mem::transmute_copy(&depthbiasclamp), ::core::mem::transmute_copy(&slopescaleddepthbias)))
        }
        unsafe extern "system" fn IASetIndexBufferStripCutValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12GraphicsCommandList9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ibstripcutvalue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetIndexBufferStripCutValue(this, ::core::mem::transmute_copy(&ibstripcutvalue)))
        }
        ID3D12GraphicsCommandList9_Vtbl {
            base__: <ID3D12GraphicsCommandList8 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RSSetDepthBias: RSSetDepthBias::<Identity, Impl, OFFSET>,
            IASetIndexBufferStripCutValue: IASetIndexBufferStripCutValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12Heap_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn GetDesc(this: &Self::This) -> D3D12_HEAP_DESC;
}
impl ::windows_core::Iids for ID3D12Heap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Heap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Heap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Heap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        ID3D12Heap_Vtbl { base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12Heap1_Impl: ::windows_core::BaseImpl + ID3D12Heap_Impl {
    fn GetProtectedResourceSession(this: &Self::This, riid: *const ::windows_core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12Heap1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Heap);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Heap1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Heap1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Heap1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtectedResourceSession(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppprotectedsession)).into())
        }
        ID3D12Heap1_Vtbl {
            base__: <ID3D12Heap as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProtectedResourceSession: GetProtectedResourceSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12InfoQueue_Impl: ::windows_core::BaseImpl {
    fn SetMessageCountLimit(this: &Self::This, messagecountlimit: u64) -> ::windows_core::Result<()>;
    fn ClearStoredMessages(this: &Self::This);
    fn GetMessage(this: &Self::This, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(this: &Self::This) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(this: &Self::This) -> u64;
    fn GetNumStoredMessages(this: &Self::This) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(this: &Self::This) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(this: &Self::This) -> u64;
    fn GetMessageCountLimit(this: &Self::This) -> u64;
    fn AddStorageFilterEntries(this: &Self::This, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetStorageFilter(this: &Self::This, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearStorageFilter(this: &Self::This);
    fn PushEmptyStorageFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushCopyOfStorageFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushStorageFilter(this: &Self::This, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopStorageFilter(this: &Self::This);
    fn GetStorageFilterStackSize(this: &Self::This) -> u32;
    fn AddRetrievalFilterEntries(this: &Self::This, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetRetrievalFilter(this: &Self::This, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearRetrievalFilter(this: &Self::This);
    fn PushEmptyRetrievalFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushRetrievalFilter(this: &Self::This, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopRetrievalFilter(this: &Self::This);
    fn GetRetrievalFilterStackSize(this: &Self::This) -> u32;
    fn AddMessage(this: &Self::This, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn AddApplicationMessage(this: &Self::This, severity: D3D12_MESSAGE_SEVERITY, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetBreakOnCategory(this: &Self::This, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnSeverity(this: &Self::This, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnID(this: &Self::This, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBreakOnCategory(this: &Self::This, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(this: &Self::This, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(this: &Self::This, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(this: &Self::This, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12InfoQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12InfoQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageCountLimit(this, ::core::mem::transmute_copy(&messagecountlimit)).into())
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStoredMessages(this))
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessage(this, ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into())
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesAllowedByStorageFilter(this))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDeniedByStorageFilter(this))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessages(this))
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDiscardedByMessageCountLimit(this))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessageCountLimit(this))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStorageFilterEntries(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilter(this, ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStorageFilter(this))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyStorageFilter(this).into())
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfStorageFilter(this).into())
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushStorageFilter(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopStorageFilter(this))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilterStackSize(this))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRetrievalFilterEntries(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilter(this, ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRetrievalFilter(this))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyRetrievalFilter(this).into())
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfRetrievalFilter(this).into())
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushRetrievalFilter(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopRetrievalFilter(this))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilterStackSize(this))
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMessage(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddApplicationMessage(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnCategory(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnSeverity(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnID(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnCategory(this, ::core::mem::transmute_copy(&category)))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnSeverity(this, ::core::mem::transmute_copy(&severity)))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnID(this, ::core::mem::transmute_copy(&id)))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMuteDebugOutput(this, ::core::mem::transmute_copy(&bmute)))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMuteDebugOutput(this))
        }
        ID3D12InfoQueue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12InfoQueue1_Impl: ::windows_core::BaseImpl + ID3D12InfoQueue_Impl {
    fn RegisterMessageCallback(this: &Self::This, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows_core::Result<()>;
    fn UnregisterMessageCallback(this: &Self::This, callbackcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12InfoQueue1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12InfoQueue);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12InfoQueue1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterMessageCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterMessageCallback(this, ::core::mem::transmute_copy(&callbackfunc), ::core::mem::transmute_copy(&callbackfilterflags), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcallbackcookie)).into())
        }
        unsafe extern "system" fn UnregisterMessageCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12InfoQueue1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callbackcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterMessageCallback(this, ::core::mem::transmute_copy(&callbackcookie)).into())
        }
        ID3D12InfoQueue1_Vtbl {
            base__: <ID3D12InfoQueue as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterMessageCallback: RegisterMessageCallback::<Identity, Impl, OFFSET>,
            UnregisterMessageCallback: UnregisterMessageCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12LibraryReflection_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This) -> ::windows_core::Result<D3D12_LIBRARY_DESC>;
    fn GetFunctionByIndex(this: &Self::This, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection>;
}
impl ::windows_core::Iids for ID3D12LibraryReflection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LibraryReflection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12LibraryReflection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LibraryReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDesc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LibraryReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionByIndex(this, ::core::mem::transmute_copy(&functionindex)))
        }
        ID3D12LibraryReflection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12LifetimeOwner_Impl: ::windows_core::BaseImpl {
    fn LifetimeStateUpdated(this: &Self::This, newstate: D3D12_LIFETIME_STATE);
}
impl ::windows_core::Iids for ID3D12LifetimeOwner {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LifetimeOwner_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12LifetimeOwner {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LifetimeStateUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LifetimeOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: D3D12_LIFETIME_STATE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LifetimeStateUpdated(this, ::core::mem::transmute_copy(&newstate)))
        }
        ID3D12LifetimeOwner_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LifetimeStateUpdated: LifetimeStateUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12LifetimeTracker_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {
    fn DestroyOwnedObject(this: &Self::This, pobject: ::core::option::Option<&ID3D12DeviceChild>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12LifetimeTracker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LifetimeTracker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12LifetimeTracker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DestroyOwnedObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12LifetimeTracker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyOwnedObject(this, ::windows_core::from_raw_borrowed(&pobject)).into())
        }
        ID3D12LifetimeTracker_Vtbl {
            base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DestroyOwnedObject: DestroyOwnedObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12ManualWriteTrackingResource_Impl: ::windows_core::BaseImpl {
    fn TrackWrite(this: &Self::This, subresource: u32, pwrittenrange: *const D3D12_RANGE);
}
impl ::windows_core::Iids for ID3D12ManualWriteTrackingResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ManualWriteTrackingResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12ManualWriteTrackingResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TrackWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ManualWriteTrackingResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TrackWrite(this, ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&pwrittenrange)))
        }
        ID3D12ManualWriteTrackingResource_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TrackWrite: TrackWrite::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12MetaCommand_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn GetRequiredParameterResourceSize(this: &Self::This, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64;
}
impl ::windows_core::Iids for ID3D12MetaCommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12MetaCommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12MetaCommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRequiredParameterResourceSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12MetaCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRequiredParameterResourceSize(this, ::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&parameterindex)))
        }
        ID3D12MetaCommand_Vtbl {
            base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRequiredParameterResourceSize: GetRequiredParameterResourceSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12Object_Impl: ::windows_core::BaseImpl {
    fn GetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, pdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12Object {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Object {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        ID3D12Object_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12Pageable_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D12Pageable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Pageable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Pageable {
    const VTABLE: Self::Vtable = { ID3D12Pageable_Vtbl { base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12PipelineLibrary_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {
    fn StorePipeline(this: &Self::This, pname: &::windows_core::PCWSTR, ppipeline: ::core::option::Option<&ID3D12PipelineState>) -> ::windows_core::Result<()>;
    fn LoadGraphicsPipeline(this: &Self::This, pname: &::windows_core::PCWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn LoadComputePipeline(this: &Self::This, pname: &::windows_core::PCWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetSerializedSize(this: &Self::This) -> usize;
    fn Serialize(this: &Self::This, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12PipelineLibrary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12PipelineLibrary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StorePipeline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, ppipeline: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StorePipeline(this, ::core::mem::transmute(&pname), ::windows_core::from_raw_borrowed(&ppipeline)).into())
        }
        unsafe extern "system" fn LoadGraphicsPipeline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadGraphicsPipeline(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into())
        }
        unsafe extern "system" fn LoadComputePipeline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadComputePipeline(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into())
        }
        unsafe extern "system" fn GetSerializedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerializedSize(this))
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasizeinbytes)).into())
        }
        ID3D12PipelineLibrary_Vtbl {
            base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StorePipeline: StorePipeline::<Identity, Impl, OFFSET>,
            LoadGraphicsPipeline: LoadGraphicsPipeline::<Identity, Impl, OFFSET>,
            LoadComputePipeline: LoadComputePipeline::<Identity, Impl, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12PipelineLibrary1_Impl: ::windows_core::BaseImpl + ID3D12PipelineLibrary_Impl {
    fn LoadPipeline(this: &Self::This, pname: &::windows_core::PCWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D12PipelineLibrary1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12PipelineLibrary);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12PipelineLibrary1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadPipeline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows_core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadPipeline(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pppipelinestate)).into())
        }
        ID3D12PipelineLibrary1_Vtbl { base__: <ID3D12PipelineLibrary as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LoadPipeline: LoadPipeline::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12PipelineState_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn GetCachedBlob(this: &Self::This) -> ::windows_core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for ID3D12PipelineState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12PipelineState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCachedBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12PipelineState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppblob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedBlob(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppblob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D12PipelineState_Vtbl { base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCachedBlob: GetCachedBlob::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12ProtectedResourceSession_Impl: ::windows_core::BaseImpl + ID3D12ProtectedSession_Impl {
    fn GetDesc(this: &Self::This) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC;
}
impl ::windows_core::Iids for ID3D12ProtectedResourceSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12ProtectedSession);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12ProtectedResourceSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        ID3D12ProtectedResourceSession_Vtbl { base__: <ID3D12ProtectedSession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12ProtectedResourceSession1_Impl: ::windows_core::BaseImpl + ID3D12ProtectedResourceSession_Impl {
    fn GetDesc1(this: &Self::This) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC1;
}
impl ::windows_core::Iids for ID3D12ProtectedResourceSession1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12ProtectedResourceSession);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12ProtectedResourceSession1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc1(this))
        }
        ID3D12ProtectedResourceSession1_Vtbl { base__: <ID3D12ProtectedResourceSession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12ProtectedSession_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {
    fn GetStatusFence(this: &Self::This, riid: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetSessionStatus(this: &Self::This) -> D3D12_PROTECTED_SESSION_STATUS;
}
impl ::windows_core::Iids for ID3D12ProtectedSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12ProtectedSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatusFence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatusFence(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppfence)).into())
        }
        unsafe extern "system" fn GetSessionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ProtectedSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSessionStatus(this))
        }
        ID3D12ProtectedSession_Vtbl {
            base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatusFence: GetStatusFence::<Identity, Impl, OFFSET>,
            GetSessionStatus: GetSessionStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12QueryHeap_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {}
impl ::windows_core::Iids for ID3D12QueryHeap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12QueryHeap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12QueryHeap {
    const VTABLE: Self::Vtable = { ID3D12QueryHeap_Vtbl { base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12Resource_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {
    fn Map(this: &Self::This, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This, subresource: u32, pwrittenrange: *const D3D12_RANGE);
    fn GetDesc(this: &Self::This) -> D3D12_RESOURCE_DESC;
    fn GetGPUVirtualAddress(this: &Self::This) -> u64;
    fn WriteToSubresource(this: &Self::This, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows_core::Result<()>;
    fn ReadFromSubresource(this: &Self::This, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows_core::Result<()>;
    fn GetHeapProperties(this: &Self::This, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D12Resource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Resource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Map(this, ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&preadrange), ::core::mem::transmute_copy(&ppdata)).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this, ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&pwrittenrange)))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        unsafe extern "system" fn GetGPUVirtualAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGPUVirtualAddress(this))
        }
        unsafe extern "system" fn WriteToSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToSubresource(this, ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch)).into())
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadFromSubresource(this, ::core::mem::transmute_copy(&pdstdata), ::core::mem::transmute_copy(&dstrowpitch), ::core::mem::transmute_copy(&dstdepthpitch), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox)).into())
        }
        unsafe extern "system" fn GetHeapProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHeapProperties(this, ::core::mem::transmute_copy(&pheapproperties), ::core::mem::transmute_copy(&pheapflags)).into())
        }
        ID3D12Resource_Vtbl {
            base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetGPUVirtualAddress: GetGPUVirtualAddress::<Identity, Impl, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, Impl, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, Impl, OFFSET>,
            GetHeapProperties: GetHeapProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12Resource1_Impl: ::windows_core::BaseImpl + ID3D12Resource_Impl {
    fn GetProtectedResourceSession(this: &Self::This, riid: *const ::windows_core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D12Resource1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Resource1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtectedResourceSession(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppprotectedsession)).into())
        }
        ID3D12Resource1_Vtbl {
            base__: <ID3D12Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProtectedResourceSession: GetProtectedResourceSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12Resource2_Impl: ::windows_core::BaseImpl + ID3D12Resource1_Impl {
    fn GetDesc1(this: &Self::This) -> D3D12_RESOURCE_DESC1;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D12Resource2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Resource1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Resource2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Resource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc1(this))
        }
        ID3D12Resource2_Vtbl { base__: <ID3D12Resource1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12RootSignature_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D12RootSignature {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12RootSignature_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12RootSignature {
    const VTABLE: Self::Vtable = { ID3D12RootSignature_Vtbl { base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12RootSignatureDeserializer_Impl: ::windows_core::BaseImpl {
    fn GetRootSignatureDesc(this: &Self::This) -> *mut D3D12_ROOT_SIGNATURE_DESC;
}
impl ::windows_core::Iids for ID3D12RootSignatureDeserializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12RootSignatureDeserializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRootSignatureDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRootSignatureDesc(this))
        }
        ID3D12RootSignatureDeserializer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRootSignatureDesc: GetRootSignatureDesc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12SDKConfiguration_Impl: ::windows_core::BaseImpl {
    fn SetSDKVersion(this: &Self::This, sdkversion: u32, sdkpath: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D12SDKConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SDKConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12SDKConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSDKVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SDKConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sdkversion: u32, sdkpath: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSDKVersion(this, ::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute(&sdkpath)).into())
        }
        ID3D12SDKConfiguration_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetSDKVersion: SetSDKVersion::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12SDKConfiguration1_Impl: ::windows_core::BaseImpl + ID3D12SDKConfiguration_Impl {
    fn CreateDeviceFactory(this: &Self::This, sdkversion: u32, sdkpath: &::windows_core::PCSTR, riid: *const ::windows_core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FreeUnusedSDKs(this: &Self::This);
}
impl ::windows_core::Iids for ID3D12SDKConfiguration1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12SDKConfiguration);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SDKConfiguration1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12SDKConfiguration1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SDKConfiguration1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sdkversion: u32, sdkpath: ::windows_core::PCSTR, riid: *const ::windows_core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeviceFactory(this, ::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute(&sdkpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into())
        }
        unsafe extern "system" fn FreeUnusedSDKs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SDKConfiguration1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeUnusedSDKs(this))
        }
        ID3D12SDKConfiguration1_Vtbl {
            base__: <ID3D12SDKConfiguration as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDeviceFactory: CreateDeviceFactory::<Identity, Impl, OFFSET>,
            FreeUnusedSDKs: FreeUnusedSDKs::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12ShaderCacheSession_Impl: ::windows_core::BaseImpl + ID3D12DeviceChild_Impl {
    fn FindValue(this: &Self::This, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows_core::Result<()>;
    fn StoreValue(this: &Self::This, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows_core::Result<()>;
    fn SetDeleteOnDestroy(this: &Self::This);
    fn GetDesc(this: &Self::This) -> D3D12_SHADER_CACHE_SESSION_DESC;
}
impl ::windows_core::Iids for ID3D12ShaderCacheSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12ShaderCacheSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindValue(this, ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvaluesize)).into())
        }
        unsafe extern "system" fn StoreValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StoreValue(this, ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&valuesize)).into())
        }
        unsafe extern "system" fn SetDeleteOnDestroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeleteOnDestroy(this))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D3D12_SHADER_CACHE_SESSION_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetDesc(this))
        }
        ID3D12ShaderCacheSession_Vtbl {
            base__: <ID3D12DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindValue: FindValue::<Identity, Impl, OFFSET>,
            StoreValue: StoreValue::<Identity, Impl, OFFSET>,
            SetDeleteOnDestroy: SetDeleteOnDestroy::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D12ShaderReflection_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D12_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(this: &Self::This, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetInputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetOutputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetPatchConstantParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(this: &Self::This, name: &::windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetMovInstructionCount(this: &Self::This) -> u32;
    fn GetMovcInstructionCount(this: &Self::This) -> u32;
    fn GetConversionInstructionCount(this: &Self::This) -> u32;
    fn GetBitwiseInstructionCount(this: &Self::This) -> u32;
    fn GetGSInputPrimitive(this: &Self::This) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(this: &Self::This) -> u32;
    fn GetMinFeatureLevel(this: &Self::This) -> ::windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(this: &Self::This, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(this: &Self::This) -> u64;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::windows_core::Iids for ID3D12ShaderReflection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12ShaderReflection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDesc(this, ::core::mem::transmute_copy(&resourceindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatchConstantParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVariableByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDescByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMovInstructionCount(this))
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMovcInstructionCount(this))
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionInstructionCount(this))
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBitwiseInstructionCount(this))
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGSInputPrimitive(this))
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSampleFrequencyShader(this))
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumInterfaceSlots(this))
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinFeatureLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadGroupSize(this, ::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez)))
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRequiresFlags(this))
        }
        ID3D12ShaderReflection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, Impl, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, Impl, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, Impl, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>() -> ID3D12ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByName(this, ::core::mem::transmute(&name))
        }
        Self { GetDesc: GetDesc::<Impl>, GetVariableByIndex: GetVariableByIndex::<Impl>, GetVariableByName: GetVariableByName::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D12ShaderReflectionConstantBuffer_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12ShaderReflectionConstantBuffer_Impl> ID3D12ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionConstantBuffer_Vtbl = ID3D12ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D12ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> ::windows_core::PCSTR;
    fn IsEqual(&self, ptype: ::core::option::Option<&ID3D12ShaderReflectionType>) -> ::windows_core::Result<()>;
    fn GetSubType(&self) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetBaseClass(&self) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn IsOfType(&self, ptype: ::core::option::Option<&ID3D12ShaderReflectionType>) -> ::windows_core::Result<()>;
    fn ImplementsInterface(&self, pbase: ::core::option::Option<&ID3D12ShaderReflectionType>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionType_Vtbl {
    pub const fn new<Impl: ID3D12ShaderReflectionType_Impl>() -> ID3D12ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::PCSTR {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeName(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, ptype: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetSubType(this)
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBaseClass(this)
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetNumInterfaces(this)
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetInterfaceByIndex(this, ::core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, ptype: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsOfType(this, ::windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, pbase: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::ImplementsInterface(this, ::windows_core::from_raw_borrowed(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeName: GetMemberTypeName::<Impl>,
            IsEqual: IsEqual::<Impl>,
            GetSubType: GetSubType::<Impl>,
            GetBaseClass: GetBaseClass::<Impl>,
            GetNumInterfaces: GetNumInterfaces::<Impl>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Impl>,
            IsOfType: IsOfType::<Impl>,
            ImplementsInterface: ImplementsInterface::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12ShaderReflectionType_ImplVtbl<T: ID3D12ShaderReflectionType_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12ShaderReflectionType_Impl> ID3D12ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionType_Vtbl = ID3D12ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionType {
    pub fn new<'a, T: ID3D12ShaderReflectionType_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ID3D12ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows_core::Result<()>;
    fn GetType(&self) -> ::core::option::Option<ID3D12ShaderReflectionType>;
    fn GetBuffer(&self) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D12ShaderReflectionVariable_Vtbl {
    pub const fn new<Impl: ID3D12ShaderReflectionVariable_Impl>() -> ID3D12ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetType(this)
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBuffer(this)
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetInterfaceSlot(this, ::core::mem::transmute_copy(&uarrayindex))
        }
        Self { GetDesc: GetDesc::<Impl>, GetType: GetType::<Impl>, GetBuffer: GetBuffer::<Impl>, GetInterfaceSlot: GetInterfaceSlot::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D12ShaderReflectionVariable_ImplVtbl<T: ID3D12ShaderReflectionVariable_Impl>(::std::marker::PhantomData<T>);
impl<T: ID3D12ShaderReflectionVariable_Impl> ID3D12ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionVariable_Vtbl = ID3D12ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D12ShaderReflectionVariable {
    pub fn new<'a, T: ID3D12ShaderReflectionVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12SharingContract_Impl: ::windows_core::BaseImpl {
    fn Present(this: &Self::This, presource: ::core::option::Option<&ID3D12Resource>, subresource: u32, window: super::super::Foundation::HWND);
    fn SharedFenceSignal(this: &Self::This, pfence: ::core::option::Option<&ID3D12Fence>, fencevalue: u64);
    fn BeginCapturableWork(this: &Self::This, guid: *const ::windows_core::GUID);
    fn EndCapturableWork(this: &Self::This, guid: *const ::windows_core::GUID);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12SharingContract {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12SharingContract {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Present<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, window: super::super::Foundation::HWND) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&window)))
        }
        unsafe extern "system" fn SharedFenceSignal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, fencevalue: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SharedFenceSignal(this, ::windows_core::from_raw_borrowed(&pfence), ::core::mem::transmute_copy(&fencevalue)))
        }
        unsafe extern "system" fn BeginCapturableWork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginCapturableWork(this, ::core::mem::transmute_copy(&guid)))
        }
        unsafe extern "system" fn EndCapturableWork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndCapturableWork(this, ::core::mem::transmute_copy(&guid)))
        }
        ID3D12SharingContract_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Present: Present::<Identity, Impl, OFFSET>,
            SharedFenceSignal: SharedFenceSignal::<Identity, Impl, OFFSET>,
            BeginCapturableWork: BeginCapturableWork::<Identity, Impl, OFFSET>,
            EndCapturableWork: EndCapturableWork::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12StateObject_Impl: ::windows_core::BaseImpl + ID3D12Pageable_Impl {}
impl ::windows_core::Iids for ID3D12StateObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D12Pageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12StateObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12StateObject {
    const VTABLE: Self::Vtable = { ID3D12StateObject_Vtbl { base__: <ID3D12Pageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12StateObjectProperties_Impl: ::windows_core::BaseImpl {
    fn GetShaderIdentifier(this: &Self::This, pexportname: &::windows_core::PCWSTR) -> *mut ::core::ffi::c_void;
    fn GetShaderStackSize(this: &Self::This, pexportname: &::windows_core::PCWSTR) -> u64;
    fn GetPipelineStackSize(this: &Self::This) -> u64;
    fn SetPipelineStackSize(this: &Self::This, pipelinestacksizeinbytes: u64);
}
impl ::windows_core::Iids for ID3D12StateObjectProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12StateObjectProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetShaderIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexportname: ::windows_core::PCWSTR) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetShaderIdentifier(this, ::core::mem::transmute(&pexportname)))
        }
        unsafe extern "system" fn GetShaderStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexportname: ::windows_core::PCWSTR) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetShaderStackSize(this, ::core::mem::transmute(&pexportname)))
        }
        unsafe extern "system" fn GetPipelineStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPipelineStackSize(this))
        }
        unsafe extern "system" fn SetPipelineStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipelinestacksizeinbytes: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPipelineStackSize(this, ::core::mem::transmute_copy(&pipelinestacksizeinbytes)))
        }
        ID3D12StateObjectProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetShaderIdentifier: GetShaderIdentifier::<Identity, Impl, OFFSET>,
            GetShaderStackSize: GetShaderStackSize::<Identity, Impl, OFFSET>,
            GetPipelineStackSize: GetPipelineStackSize::<Identity, Impl, OFFSET>,
            SetPipelineStackSize: SetPipelineStackSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12SwapChainAssistant_Impl: ::windows_core::BaseImpl {
    fn GetLUID(this: &Self::This) -> super::super::Foundation::LUID;
    fn GetSwapChainObject(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCurrentResourceAndCommandQueue(this: &Self::This, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows_core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn InsertImplicitSync(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12SwapChainAssistant {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12SwapChainAssistant {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetLUID(this))
        }
        unsafe extern "system" fn GetSwapChainObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSwapChainObject(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetCurrentResourceAndCommandQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riidresource: *const ::windows_core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows_core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentResourceAndCommandQueue(this, ::core::mem::transmute_copy(&riidresource), ::core::mem::transmute_copy(&ppvresource), ::core::mem::transmute_copy(&riidqueue), ::core::mem::transmute_copy(&ppvqueue)).into())
        }
        unsafe extern "system" fn InsertImplicitSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertImplicitSync(this).into())
        }
        ID3D12SwapChainAssistant_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLUID: GetLUID::<Identity, Impl, OFFSET>,
            GetSwapChainObject: GetSwapChainObject::<Identity, Impl, OFFSET>,
            GetCurrentResourceAndCommandQueue: GetCurrentResourceAndCommandQueue::<Identity, Impl, OFFSET>,
            InsertImplicitSync: InsertImplicitSync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12Tools_Impl: ::windows_core::BaseImpl {
    fn EnableShaderInstrumentation(this: &Self::This, benable: super::super::Foundation::BOOL);
    fn ShaderInstrumentationEnabled(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12Tools {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Tools_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12Tools {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableShaderInstrumentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Tools_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableShaderInstrumentation(this, ::core::mem::transmute_copy(&benable)))
        }
        unsafe extern "system" fn ShaderInstrumentationEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12Tools_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShaderInstrumentationEnabled(this))
        }
        ID3D12Tools_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableShaderInstrumentation: EnableShaderInstrumentation::<Identity, Impl, OFFSET>,
            ShaderInstrumentationEnabled: ShaderInstrumentationEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3D12VersionedRootSignatureDeserializer_Impl: ::windows_core::BaseImpl {
    fn GetRootSignatureDescAtVersion(this: &Self::This, converttoversion: D3D_ROOT_SIGNATURE_VERSION) -> ::windows_core::Result<*mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC>;
    fn GetUnconvertedRootSignatureDesc(this: &Self::This) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC;
}
impl ::windows_core::Iids for ID3D12VersionedRootSignatureDeserializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12VersionedRootSignatureDeserializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRootSignatureDescAtVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, converttoversion: D3D_ROOT_SIGNATURE_VERSION, ppdesc: *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootSignatureDescAtVersion(this, ::core::mem::transmute_copy(&converttoversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUnconvertedRootSignatureDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUnconvertedRootSignatureDesc(this))
        }
        ID3D12VersionedRootSignatureDeserializer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRootSignatureDescAtVersion: GetRootSignatureDescAtVersion::<Identity, Impl, OFFSET>,
            GetUnconvertedRootSignatureDesc: GetUnconvertedRootSignatureDesc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D12VirtualizationGuestDevice_Impl: ::windows_core::BaseImpl {
    fn ShareWithHost(this: &Self::This, pobject: ::core::option::Option<&ID3D12DeviceChild>) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn CreateFenceFd(this: &Self::This, pfence: ::core::option::Option<&ID3D12Fence>, fencevalue: u64) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D12VirtualizationGuestDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12VirtualizationGuestDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D12VirtualizationGuestDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShareWithHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12VirtualizationGuestDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShareWithHost(this, ::windows_core::from_raw_borrowed(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFenceFd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D12VirtualizationGuestDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, fencevalue: u64, pfencefd: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFenceFd(this, ::windows_core::from_raw_borrowed(&pfence), ::core::mem::transmute_copy(&fencevalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfencefd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D12VirtualizationGuestDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ShareWithHost: ShareWithHost::<Identity, Impl, OFFSET>,
            CreateFenceFd: CreateFenceFd::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
