#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ILearningModelDeviceFactoryNative_Impl: ::windows_core::BaseImpl {
    fn CreateFromD3D12CommandQueue(this: &Self::This, value: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for ILearningModelDeviceFactoryNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelDeviceFactoryNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelDeviceFactoryNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromD3D12CommandQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelDeviceFactoryNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFromD3D12CommandQueue(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILearningModelDeviceFactoryNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromD3D12CommandQueue: CreateFromD3D12CommandQueue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_AI_MachineLearning_WinML\"`"]
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
pub trait ILearningModelOperatorProviderNative_Impl: ::windows_core::BaseImpl {
    fn GetRegistry(this: &Self::This) -> ::windows_core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>;
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl ::windows_core::Iids for ILearningModelOperatorProviderNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelOperatorProviderNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelOperatorProviderNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRegistry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelOperatorProviderNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegistry(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperatorregistry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILearningModelOperatorProviderNative_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetRegistry: GetRegistry::<Identity, Impl, OFFSET> }
    };
}
pub trait ILearningModelSessionOptionsNative_Impl: ::windows_core::BaseImpl {
    fn SetIntraOpNumThreadsOverride(this: &Self::This, intraopnumthreads: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILearningModelSessionOptionsNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelSessionOptionsNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelSessionOptionsNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIntraOpNumThreadsOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelSessionOptionsNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIntraOpNumThreadsOverride(this, ::core::mem::transmute_copy(&intraopnumthreads)).into())
        }
        ILearningModelSessionOptionsNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIntraOpNumThreadsOverride: SetIntraOpNumThreadsOverride::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILearningModelSessionOptionsNative1_Impl: ::windows_core::BaseImpl {
    fn SetIntraOpThreadSpinning(this: &Self::This, allowspinning: u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILearningModelSessionOptionsNative1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelSessionOptionsNative1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelSessionOptionsNative1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIntraOpThreadSpinning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelSessionOptionsNative1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowspinning: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIntraOpThreadSpinning(this, ::core::mem::transmute_copy(&allowspinning)).into())
        }
        ILearningModelSessionOptionsNative1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIntraOpThreadSpinning: SetIntraOpThreadSpinning::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorNative_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::Result<()>;
    fn GetD3D12Resource(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for ITensorNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensorNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITensorNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensorNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)).into())
        }
        unsafe extern "system" fn GetD3D12Resource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensorNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetD3D12Resource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITensorNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetD3D12Resource: GetD3D12Resource::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorStaticsNative_Impl: ::windows_core::BaseImpl {
    fn CreateFromD3D12Resource(this: &Self::This, value: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for ITensorStaticsNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensorStaticsNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITensorStaticsNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromD3D12Resource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensorStaticsNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromD3D12Resource(this, ::windows_core::from_raw_borrowed(&value), ::core::mem::transmute_copy(&shape), ::core::mem::transmute_copy(&shapecount), ::core::mem::transmute_copy(&result)).into())
        }
        ITensorStaticsNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromD3D12Resource: CreateFromD3D12Resource::<Identity, Impl, OFFSET>,
        }
    };
}
