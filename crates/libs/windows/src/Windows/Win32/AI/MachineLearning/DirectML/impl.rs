#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLBindingTable_Impl: ::windows_core::BaseImpl + IDMLDeviceChild_Impl {
    fn BindInputs(this: &Self::This, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindOutputs(this: &Self::This, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindTemporaryResource(this: &Self::This, binding: *const DML_BINDING_DESC);
    fn BindPersistentResource(this: &Self::This, binding: *const DML_BINDING_DESC);
    fn Reset(this: &Self::This, desc: *const DML_BINDING_TABLE_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IDMLBindingTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDeviceChild);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLBindingTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindInputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindInputs(this, ::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings)))
        }
        unsafe extern "system" fn BindOutputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindOutputs(this, ::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings)))
        }
        unsafe extern "system" fn BindTemporaryResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindTemporaryResource(this, ::core::mem::transmute_copy(&binding)))
        }
        unsafe extern "system" fn BindPersistentResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindPersistentResource(this, ::core::mem::transmute_copy(&binding)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this, ::core::mem::transmute_copy(&desc)).into())
        }
        IDMLBindingTable_Vtbl {
            base__: <IDMLDeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindInputs: BindInputs::<Identity, Impl, OFFSET>,
            BindOutputs: BindOutputs::<Identity, Impl, OFFSET>,
            BindTemporaryResource: BindTemporaryResource::<Identity, Impl, OFFSET>,
            BindPersistentResource: BindPersistentResource::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLCommandRecorder_Impl: ::windows_core::BaseImpl + IDMLDeviceChild_Impl {
    fn RecordDispatch(this: &Self::This, commandlist: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandList>, dispatchable: ::core::option::Option<&IDMLDispatchable>, bindings: ::core::option::Option<&IDMLBindingTable>);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IDMLCommandRecorder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDeviceChild);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLCommandRecorder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLCommandRecorder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RecordDispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLCommandRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandlist: *mut ::core::ffi::c_void, dispatchable: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordDispatch(this, ::windows_core::from_raw_borrowed(&commandlist), ::windows_core::from_raw_borrowed(&dispatchable), ::windows_core::from_raw_borrowed(&bindings)))
        }
        IDMLCommandRecorder_Vtbl { base__: <IDMLDeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RecordDispatch: RecordDispatch::<Identity, Impl, OFFSET> }
    };
}
pub trait IDMLCompiledOperator_Impl: ::windows_core::BaseImpl + IDMLDispatchable_Impl {}
impl ::windows_core::Iids for IDMLCompiledOperator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDispatchable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLCompiledOperator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLCompiledOperator {
    const VTABLE: Self::Vtable = { IDMLCompiledOperator_Vtbl { base__: <IDMLDispatchable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDebugDevice_Impl: ::windows_core::BaseImpl {
    fn SetMuteDebugOutput(this: &Self::This, mute: super::super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDMLDebugDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDebugDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLDebugDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDebugDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMuteDebugOutput(this, ::core::mem::transmute_copy(&mute)))
        }
        IDMLDebugDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLDevice_Impl: ::windows_core::BaseImpl + IDMLObject_Impl {
    fn CheckFeatureSupport(this: &Self::This, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateOperator(this: &Self::This, desc: *const DML_OPERATOR_DESC, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CompileOperator(this: &Self::This, op: ::core::option::Option<&IDMLOperator>, flags: DML_EXECUTION_FLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateOperatorInitializer(this: &Self::This, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateCommandRecorder(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateBindingTable(this: &Self::This, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Evict(this: &Self::This, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows_core::Result<()>;
    fn MakeResident(this: &Self::This, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows_core::Result<()>;
    fn GetDeviceRemovedReason(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetParentDevice(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IDMLDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLObject);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckFeatureSupport(this, ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&featurequerydatasize), ::core::mem::transmute_copy(&featurequerydata), ::core::mem::transmute_copy(&featuresupportdatasize), ::core::mem::transmute_copy(&featuresupportdata)).into())
        }
        unsafe extern "system" fn CreateOperator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateOperator(this, ::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CompileOperator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, op: *mut ::core::ffi::c_void, flags: DML_EXECUTION_FLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompileOperator(this, ::windows_core::from_raw_borrowed(&op), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateOperatorInitializer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateOperatorInitializer(this, ::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateCommandRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCommandRecorder(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateBindingTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBindingTable(this, ::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn Evict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Evict(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into())
        }
        unsafe extern "system" fn MakeResident<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeResident(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into())
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceRemovedReason(this).into())
        }
        unsafe extern "system" fn GetParentDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParentDevice(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IDMLDevice_Vtbl {
            base__: <IDMLObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            CreateOperator: CreateOperator::<Identity, Impl, OFFSET>,
            CompileOperator: CompileOperator::<Identity, Impl, OFFSET>,
            CreateOperatorInitializer: CreateOperatorInitializer::<Identity, Impl, OFFSET>,
            CreateCommandRecorder: CreateCommandRecorder::<Identity, Impl, OFFSET>,
            CreateBindingTable: CreateBindingTable::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            MakeResident: MakeResident::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetParentDevice: GetParentDevice::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLDevice1_Impl: ::windows_core::BaseImpl + IDMLDevice_Impl {
    fn CompileGraph(this: &Self::This, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IDMLDevice1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDevice);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLDevice1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompileGraph<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompileGraph(this, ::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IDMLDevice1_Vtbl { base__: <IDMLDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CompileGraph: CompileGraph::<Identity, Impl, OFFSET> }
    };
}
pub trait IDMLDeviceChild_Impl: ::windows_core::BaseImpl + IDMLObject_Impl {
    fn GetDevice(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDMLDeviceChild {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLObject);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDeviceChild_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLDeviceChild {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IDMLDeviceChild_Vtbl { base__: <IDMLObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    };
}
pub trait IDMLDispatchable_Impl: ::windows_core::BaseImpl + IDMLPageable_Impl {
    fn GetBindingProperties(this: &Self::This) -> DML_BINDING_PROPERTIES;
}
impl ::windows_core::Iids for IDMLDispatchable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLPageable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDispatchable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLDispatchable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBindingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLDispatchable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetBindingProperties(this))
        }
        IDMLDispatchable_Vtbl { base__: <IDMLPageable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBindingProperties: GetBindingProperties::<Identity, Impl, OFFSET> }
    };
}
pub trait IDMLObject_Impl: ::windows_core::BaseImpl {
    fn GetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, data: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDMLObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&data)).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        IDMLObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDMLOperator_Impl: ::windows_core::BaseImpl + IDMLDeviceChild_Impl {}
impl ::windows_core::Iids for IDMLOperator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLOperator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLOperator {
    const VTABLE: Self::Vtable = { IDMLOperator_Vtbl { base__: <IDMLDeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDMLOperatorInitializer_Impl: ::windows_core::BaseImpl + IDMLDispatchable_Impl {
    fn Reset(this: &Self::This, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDMLOperatorInitializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDispatchable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLOperatorInitializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLOperatorInitializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLOperatorInitializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this, ::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators)).into())
        }
        IDMLOperatorInitializer_Vtbl { base__: <IDMLDispatchable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Reset: Reset::<Identity, Impl, OFFSET> }
    };
}
pub trait IDMLPageable_Impl: ::windows_core::BaseImpl + IDMLDeviceChild_Impl {}
impl ::windows_core::Iids for IDMLPageable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDMLDeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMLPageable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMLPageable {
    const VTABLE: Self::Vtable = { IDMLPageable_Vtbl { base__: <IDMLDeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
