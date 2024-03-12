pub trait IMLOperatorAttributes_Impl: ::windows_core::BaseImpl {
    fn GetAttributeElementCount(this: &Self::This, name: &::windows_core::PCSTR, r#type: MLOperatorAttributeType) -> ::windows_core::Result<u32>;
    fn GetAttribute(this: &Self::This, name: &::windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetStringAttributeElementLength(this: &Self::This, name: &::windows_core::PCSTR, elementindex: u32) -> ::windows_core::Result<u32>;
    fn GetStringAttributeElement(this: &Self::This, name: &::windows_core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: ::windows_core::PSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorAttributes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorAttributes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttributeElementCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeElementCount(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elementcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttribute(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&elementcount), ::core::mem::transmute_copy(&elementbytesize), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringAttributeElementLength(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&elementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributeelementbytesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringAttributeElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: ::windows_core::PSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringAttributeElement(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&elementindex), ::core::mem::transmute_copy(&attributeelementbytesize), ::core::mem::transmute_copy(&attributeelement)).into())
        }
        IMLOperatorAttributes_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttributeElementCount: GetAttributeElementCount::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetStringAttributeElementLength: GetStringAttributeElementLength::<Identity, Impl, OFFSET>,
            GetStringAttributeElement: GetStringAttributeElement::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorKernel_Impl: ::windows_core::BaseImpl {
    fn Compute(this: &Self::This, context: ::core::option::Option<&IMLOperatorKernelContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorKernel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorKernel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compute(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        IMLOperatorKernel_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Compute: Compute::<Identity, Impl, OFFSET> }
    };
}
pub trait IMLOperatorKernelContext_Impl: ::windows_core::BaseImpl {
    fn GetInputTensor(this: &Self::This, inputindex: u32) -> ::windows_core::Result<IMLOperatorTensor>;
    fn GetOutputTensor(this: &Self::This, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows_core::Result<IMLOperatorTensor>;
    fn GetOutputTensor2(this: &Self::This, outputindex: u32) -> ::windows_core::Result<IMLOperatorTensor>;
    fn AllocateTemporaryData(this: &Self::This, size: usize) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetExecutionInterface(this: &Self::This, executionobject: *mut ::core::option::Option<::windows_core::IUnknown>);
}
impl ::windows_core::Iids for IMLOperatorKernelContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorKernelContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputTensor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputTensor(this, ::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputTensor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputTensor(this, ::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensionsizes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputTensor2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputTensor2(this, ::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllocateTemporaryData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocateTemporaryData(this, ::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExecutionInterface(this, ::core::mem::transmute_copy(&executionobject)))
        }
        IMLOperatorKernelContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputTensor: GetInputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor: GetOutputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor2: GetOutputTensor2::<Identity, Impl, OFFSET>,
            AllocateTemporaryData: AllocateTemporaryData::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorKernelCreationContext_Impl: ::windows_core::BaseImpl + IMLOperatorAttributes_Impl {
    fn GetInputCount(this: &Self::This) -> u32;
    fn GetOutputCount(this: &Self::This) -> u32;
    fn IsInputValid(this: &Self::This, inputindex: u32) -> bool;
    fn IsOutputValid(this: &Self::This, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(this: &Self::This, inputindex: u32) -> ::windows_core::Result<MLOperatorEdgeDescription>;
    fn GetOutputEdgeDescription(this: &Self::This, outputindex: u32) -> ::windows_core::Result<MLOperatorEdgeDescription>;
    fn HasTensorShapeDescription(this: &Self::This) -> bool;
    fn GetTensorShapeDescription(this: &Self::This) -> ::windows_core::Result<IMLOperatorTensorShapeDescription>;
    fn GetExecutionInterface(this: &Self::This, executionobject: *mut ::core::option::Option<::windows_core::IUnknown>);
}
impl ::windows_core::Iids for IMLOperatorKernelCreationContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLOperatorAttributes);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorKernelCreationContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCount(this))
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputCount(this))
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsInputValid(this, ::core::mem::transmute_copy(&inputindex)))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsOutputValid(this, ::core::mem::transmute_copy(&outputindex)))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputEdgeDescription(this, ::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputEdgeDescription(this, ::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasTensorShapeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasTensorShapeDescription(this))
        }
        unsafe extern "system" fn GetTensorShapeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shapedescription: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTensorShapeDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shapedescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExecutionInterface(this, ::core::mem::transmute_copy(&executionobject)))
        }
        IMLOperatorKernelCreationContext_Vtbl {
            base__: <IMLOperatorAttributes as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            GetOutputEdgeDescription: GetOutputEdgeDescription::<Identity, Impl, OFFSET>,
            HasTensorShapeDescription: HasTensorShapeDescription::<Identity, Impl, OFFSET>,
            GetTensorShapeDescription: GetTensorShapeDescription::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorKernelFactory_Impl: ::windows_core::BaseImpl {
    fn CreateKernel(this: &Self::This, context: ::core::option::Option<&IMLOperatorKernelCreationContext>) -> ::windows_core::Result<IMLOperatorKernel>;
}
impl ::windows_core::Iids for IMLOperatorKernelFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorKernelFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateKernel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, kernel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateKernel(this, ::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(kernel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMLOperatorKernelFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateKernel: CreateKernel::<Identity, Impl, OFFSET> }
    };
}
pub trait IMLOperatorRegistry_Impl: ::windows_core::BaseImpl {
    fn RegisterOperatorSetSchema(this: &Self::This, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::core::option::Option<&IMLOperatorTypeInferrer>, shapeinferrer: ::core::option::Option<&IMLOperatorShapeInferrer>) -> ::windows_core::Result<()>;
    fn RegisterOperatorKernel(this: &Self::This, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::core::option::Option<&IMLOperatorKernelFactory>, shapeinferrer: ::core::option::Option<&IMLOperatorShapeInferrer>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorRegistry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorRegistry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterOperatorSetSchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterOperatorSetSchema(this, ::core::mem::transmute_copy(&operatorsetid), ::core::mem::transmute_copy(&baselineversion), ::core::mem::transmute_copy(&schema), ::core::mem::transmute_copy(&schemacount), ::windows_core::from_raw_borrowed(&typeinferrer), ::windows_core::from_raw_borrowed(&shapeinferrer)).into())
        }
        unsafe extern "system" fn RegisterOperatorKernel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterOperatorKernel(this, ::core::mem::transmute_copy(&operatorkernel), ::windows_core::from_raw_borrowed(&operatorkernelfactory), ::windows_core::from_raw_borrowed(&shapeinferrer)).into())
        }
        IMLOperatorRegistry_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterOperatorSetSchema: RegisterOperatorSetSchema::<Identity, Impl, OFFSET>,
            RegisterOperatorKernel: RegisterOperatorKernel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorShapeInferenceContext_Impl: ::windows_core::BaseImpl + IMLOperatorAttributes_Impl {
    fn GetInputCount(this: &Self::This) -> u32;
    fn GetOutputCount(this: &Self::This) -> u32;
    fn IsInputValid(this: &Self::This, inputindex: u32) -> bool;
    fn IsOutputValid(this: &Self::This, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(this: &Self::This, inputindex: u32) -> ::windows_core::Result<MLOperatorEdgeDescription>;
    fn GetInputTensorDimensionCount(this: &Self::This, inputindex: u32) -> ::windows_core::Result<u32>;
    fn GetInputTensorShape(this: &Self::This, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::Result<()>;
    fn SetOutputTensorShape(this: &Self::This, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorShapeInferenceContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLOperatorAttributes);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorShapeInferenceContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCount(this))
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputCount(this))
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsInputValid(this, ::core::mem::transmute_copy(&inputindex)))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsOutputValid(this, ::core::mem::transmute_copy(&outputindex)))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputEdgeDescription(this, ::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputTensorDimensionCount(this, ::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimensioncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputTensorShape(this, ::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into())
        }
        unsafe extern "system" fn SetOutputTensorShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputTensorShape(this, ::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into())
        }
        IMLOperatorShapeInferenceContext_Vtbl {
            base__: <IMLOperatorAttributes as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            SetOutputTensorShape: SetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorShapeInferrer_Impl: ::windows_core::BaseImpl {
    fn InferOutputShapes(this: &Self::This, context: ::core::option::Option<&IMLOperatorShapeInferenceContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorShapeInferrer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorShapeInferrer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InferOutputShapes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InferOutputShapes(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        IMLOperatorShapeInferrer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InferOutputShapes: InferOutputShapes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorTensor_Impl: ::windows_core::BaseImpl {
    fn GetDimensionCount(this: &Self::This) -> u32;
    fn GetShape(this: &Self::This, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::Result<()>;
    fn GetTensorDataType(this: &Self::This) -> MLOperatorTensorDataType;
    fn IsCpuData(this: &Self::This) -> bool;
    fn IsDataInterface(this: &Self::This) -> bool;
    fn GetData(this: &Self::This) -> *mut ::core::ffi::c_void;
    fn GetDataInterface(this: &Self::This, datainterface: *mut ::core::option::Option<::windows_core::IUnknown>);
}
impl ::windows_core::Iids for IMLOperatorTensor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorTensor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDimensionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDimensionCount(this))
        }
        unsafe extern "system" fn GetShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetShape(this, ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into())
        }
        unsafe extern "system" fn GetTensorDataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTensorDataType(this))
        }
        unsafe extern "system" fn IsCpuData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCpuData(this))
        }
        unsafe extern "system" fn IsDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDataInterface(this))
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this))
        }
        unsafe extern "system" fn GetDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataInterface(this, ::core::mem::transmute_copy(&datainterface)))
        }
        IMLOperatorTensor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDimensionCount: GetDimensionCount::<Identity, Impl, OFFSET>,
            GetShape: GetShape::<Identity, Impl, OFFSET>,
            GetTensorDataType: GetTensorDataType::<Identity, Impl, OFFSET>,
            IsCpuData: IsCpuData::<Identity, Impl, OFFSET>,
            IsDataInterface: IsDataInterface::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataInterface: GetDataInterface::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorTensorShapeDescription_Impl: ::windows_core::BaseImpl {
    fn GetInputTensorDimensionCount(this: &Self::This, inputindex: u32) -> ::windows_core::Result<u32>;
    fn GetInputTensorShape(this: &Self::This, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::Result<()>;
    fn HasOutputShapeDescription(this: &Self::This) -> bool;
    fn GetOutputTensorDimensionCount(this: &Self::This, outputindex: u32) -> ::windows_core::Result<u32>;
    fn GetOutputTensorShape(this: &Self::This, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorTensorShapeDescription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorTensorShapeDescription {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputTensorDimensionCount(this, ::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimensioncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputTensorShape(this, ::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into())
        }
        unsafe extern "system" fn HasOutputShapeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasOutputShapeDescription(this))
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputTensorDimensionCount(this, ::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimensioncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputTensorShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputTensorShape(this, ::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into())
        }
        IMLOperatorTensorShapeDescription_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            HasOutputShapeDescription: HasOutputShapeDescription::<Identity, Impl, OFFSET>,
            GetOutputTensorDimensionCount: GetOutputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetOutputTensorShape: GetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorTypeInferenceContext_Impl: ::windows_core::BaseImpl + IMLOperatorAttributes_Impl {
    fn GetInputCount(this: &Self::This) -> u32;
    fn GetOutputCount(this: &Self::This) -> u32;
    fn IsInputValid(this: &Self::This, inputindex: u32) -> bool;
    fn IsOutputValid(this: &Self::This, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(this: &Self::This, inputindex: u32) -> ::windows_core::Result<MLOperatorEdgeDescription>;
    fn SetOutputEdgeDescription(this: &Self::This, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorTypeInferenceContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLOperatorAttributes);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorTypeInferenceContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCount(this))
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputCount(this))
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsInputValid(this, ::core::mem::transmute_copy(&inputindex)))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsOutputValid(this, ::core::mem::transmute_copy(&outputindex)))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputEdgeDescription(this, ::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputEdgeDescription(this, ::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&edgedescription)).into())
        }
        IMLOperatorTypeInferenceContext_Vtbl {
            base__: <IMLOperatorAttributes as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            SetOutputEdgeDescription: SetOutputEdgeDescription::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMLOperatorTypeInferrer_Impl: ::windows_core::BaseImpl {
    fn InferOutputTypes(this: &Self::This, context: ::core::option::Option<&IMLOperatorTypeInferenceContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLOperatorTypeInferrer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLOperatorTypeInferrer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InferOutputTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InferOutputTypes(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        IMLOperatorTypeInferrer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InferOutputTypes: InferOutputTypes::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLEvaluationContext_Impl: ::windows_core::BaseImpl {
    fn BindValue(this: &Self::This, pdescriptor: *const WINML_BINDING_DESC) -> ::windows_core::Result<()>;
    fn GetValueByName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut WINML_BINDING_DESC>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IWinMLEvaluationContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinMLEvaluationContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindValue(this, ::core::mem::transmute_copy(&pdescriptor)).into())
        }
        unsafe extern "system" fn GetValueByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValueByName(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IWinMLEvaluationContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindValue: BindValue::<Identity, Impl, OFFSET>,
            GetValueByName: GetValueByName::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWinMLModel_Impl: ::windows_core::BaseImpl {
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<*mut WINML_MODEL_DESC>;
    fn EnumerateMetadata(this: &Self::This, index: u32, pkey: *mut ::windows_core::PCWSTR, pvalue: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumerateModelInputs(this: &Self::This, index: u32) -> ::windows_core::Result<*mut WINML_VARIABLE_DESC>;
    fn EnumerateModelOutputs(this: &Self::This, index: u32) -> ::windows_core::Result<*mut WINML_VARIABLE_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWinMLModel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinMLModel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut ::windows_core::PCWSTR, pvalue: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateMetadata(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn EnumerateModelInputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateModelInputs(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinputdescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateModelOutputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateModelOutputs(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputdescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWinMLModel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Identity, Impl, OFFSET>,
            EnumerateModelInputs: EnumerateModelInputs::<Identity, Impl, OFFSET>,
            EnumerateModelOutputs: EnumerateModelOutputs::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLRuntime_Impl: ::windows_core::BaseImpl {
    fn LoadModel(this: &Self::This, path: &::windows_core::PCWSTR) -> ::windows_core::Result<IWinMLModel>;
    fn CreateEvaluationContext(this: &Self::This, device: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>) -> ::windows_core::Result<IWinMLEvaluationContext>;
    fn EvaluateModel(this: &Self::This, pcontext: ::core::option::Option<&IWinMLEvaluationContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IWinMLRuntime {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinMLRuntime {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, ppmodel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadModel(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmodel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEvaluationContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEvaluationContext(this, ::windows_core::from_raw_borrowed(&device)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EvaluateModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EvaluateModel(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        IWinMLRuntime_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadModel: LoadModel::<Identity, Impl, OFFSET>,
            CreateEvaluationContext: CreateEvaluationContext::<Identity, Impl, OFFSET>,
            EvaluateModel: EvaluateModel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWinMLRuntimeFactory_Impl: ::windows_core::BaseImpl {
    fn CreateRuntime(this: &Self::This, runtimetype: WINML_RUNTIME_TYPE) -> ::windows_core::Result<IWinMLRuntime>;
}
impl ::windows_core::Iids for IWinMLRuntimeFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinMLRuntimeFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRuntime(this, ::core::mem::transmute_copy(&runtimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppruntime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWinMLRuntimeFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateRuntime: CreateRuntime::<Identity, Impl, OFFSET> }
    };
}
