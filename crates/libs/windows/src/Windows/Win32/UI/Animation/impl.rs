pub trait IUIAnimationInterpolator_Impl: ::windows_core::BaseImpl {
    fn SetInitialValueAndVelocity(this: &Self::This, initialvalue: f64, initialvelocity: f64) -> ::windows_core::Result<()>;
    fn SetDuration(this: &Self::This, duration: f64) -> ::windows_core::Result<()>;
    fn GetDuration(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetFinalValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn InterpolateValue(this: &Self::This, offset: f64) -> ::windows_core::Result<f64>;
    fn InterpolateVelocity(this: &Self::This, offset: f64) -> ::windows_core::Result<f64>;
    fn GetDependencies(this: &Self::This, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationInterpolator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationInterpolator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialValueAndVelocity(this, ::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&initialvelocity)).into())
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuration(this, ::core::mem::transmute_copy(&duration)).into())
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(duration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFinalValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterpolateValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterpolateValue(this, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterpolateVelocity(this, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDependencies(this, ::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)).into())
        }
        IUIAnimationInterpolator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, Impl, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationInterpolator2_Impl: ::windows_core::BaseImpl {
    fn GetDimension(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetInitialValueAndVelocity(this: &Self::This, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn SetDuration(this: &Self::This, duration: f64) -> ::windows_core::Result<()>;
    fn GetDuration(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetFinalValue(this: &Self::This, value: *mut f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn InterpolateValue(this: &Self::This, offset: f64, value: *mut f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn InterpolateVelocity(this: &Self::This, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetPrimitiveInterpolation(this: &Self::This, interpolation: ::core::option::Option<&IUIAnimationPrimitiveInterpolation>, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetDependencies(this: &Self::This, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationInterpolator2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationInterpolator2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDimension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDimension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialValueAndVelocity(this, ::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&initialvelocity), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuration(this, ::core::mem::transmute_copy(&duration)).into())
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(duration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFinalValue(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn InterpolateValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InterpolateValue(this, ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InterpolateVelocity(this, ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&velocity), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolation: *mut ::core::ffi::c_void, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrimitiveInterpolation(this, ::windows_core::from_raw_borrowed(&interpolation), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDependencies(this, ::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)).into())
        }
        IUIAnimationInterpolator2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, Impl, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, Impl, OFFSET>,
            GetPrimitiveInterpolation: GetPrimitiveInterpolation::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationLoopIterationChangeHandler2_Impl: ::windows_core::BaseImpl {
    fn OnLoopIterationChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard2>, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationLoopIterationChangeHandler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationLoopIterationChangeHandler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLoopIterationChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLoopIterationChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&newiterationcount), ::core::mem::transmute_copy(&olditerationcount)).into())
        }
        IUIAnimationLoopIterationChangeHandler2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLoopIterationChanged: OnLoopIterationChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationManager_Impl: ::windows_core::BaseImpl {
    fn CreateAnimationVariable(this: &Self::This, initialvalue: f64) -> ::windows_core::Result<IUIAnimationVariable>;
    fn ScheduleTransition(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable>, transition: ::core::option::Option<&IUIAnimationTransition>, timenow: f64) -> ::windows_core::Result<()>;
    fn CreateStoryboard(this: &Self::This) -> ::windows_core::Result<IUIAnimationStoryboard>;
    fn FinishAllStoryboards(this: &Self::This, completiondeadline: f64) -> ::windows_core::Result<()>;
    fn AbandonAllStoryboards(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::Result<()>;
    fn GetVariableFromTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<IUIAnimationVariable>;
    fn GetStoryboardFromTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<IUIAnimationStoryboard>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(this: &Self::This, mode: UI_ANIMATION_MODE) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetManagerEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationManagerEventHandler>) -> ::windows_core::Result<()>;
    fn SetCancelPriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison>) -> ::windows_core::Result<()>;
    fn SetTrimPriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison>) -> ::windows_core::Result<()>;
    fn SetCompressPriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison>) -> ::windows_core::Result<()>;
    fn SetConcludePriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison>) -> ::windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(this: &Self::This, delay: f64) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateAnimationVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAnimationVariable(this, ::core::mem::transmute_copy(&initialvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScheduleTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScheduleTransition(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition), ::core::mem::transmute_copy(&timenow)).into())
        }
        unsafe extern "system" fn CreateStoryboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStoryboard(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyboard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAllStoryboards(this, ::core::mem::transmute_copy(&completiondeadline)).into())
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbandonAllStoryboards(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::core::mem::transmute_copy(&timenow), ::core::mem::transmute_copy(&updateresult)).into())
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVariableFromTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStoryboardFromTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyboard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAnimationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAnimationMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManagerEventHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCancelPriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrimPriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompressPriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConcludePriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultLongestAcceptableDelay(this, ::core::mem::transmute_copy(&delay)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        IUIAnimationManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateAnimationVariable: CreateAnimationVariable::<Identity, Impl, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, Impl, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, Impl, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, Impl, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, Impl, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, Impl, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, Impl, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, Impl, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, Impl, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAnimationManager2_Impl: ::windows_core::BaseImpl {
    fn CreateAnimationVectorVariable(this: &Self::This, initialvalue: *const f64, cdimension: u32) -> ::windows_core::Result<IUIAnimationVariable2>;
    fn CreateAnimationVariable(this: &Self::This, initialvalue: f64) -> ::windows_core::Result<IUIAnimationVariable2>;
    fn ScheduleTransition(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable2>, transition: ::core::option::Option<&IUIAnimationTransition2>, timenow: f64) -> ::windows_core::Result<()>;
    fn CreateStoryboard(this: &Self::This) -> ::windows_core::Result<IUIAnimationStoryboard2>;
    fn FinishAllStoryboards(this: &Self::This, completiondeadline: f64) -> ::windows_core::Result<()>;
    fn AbandonAllStoryboards(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::Result<()>;
    fn GetVariableFromTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<IUIAnimationVariable2>;
    fn GetStoryboardFromTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<IUIAnimationStoryboard2>;
    fn EstimateNextEventTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(this: &Self::This, mode: UI_ANIMATION_MODE) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetManagerEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationManagerEventHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCancelPriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison2>) -> ::windows_core::Result<()>;
    fn SetTrimPriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison2>) -> ::windows_core::Result<()>;
    fn SetCompressPriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison2>) -> ::windows_core::Result<()>;
    fn SetConcludePriorityComparison(this: &Self::This, comparison: ::core::option::Option<&IUIAnimationPriorityComparison2>) -> ::windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(this: &Self::This, delay: f64) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAnimationManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateAnimationVectorVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAnimationVectorVariable(this, ::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAnimationVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAnimationVariable(this, ::core::mem::transmute_copy(&initialvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScheduleTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScheduleTransition(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition), ::core::mem::transmute_copy(&timenow)).into())
        }
        unsafe extern "system" fn CreateStoryboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStoryboard(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyboard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAllStoryboards(this, ::core::mem::transmute_copy(&completiondeadline)).into())
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbandonAllStoryboards(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::core::mem::transmute_copy(&timenow), ::core::mem::transmute_copy(&updateresult)).into())
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVariableFromTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStoryboardFromTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyboard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EstimateNextEventTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EstimateNextEventTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(seconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAnimationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAnimationMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManagerEventHandler(this, ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into())
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCancelPriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrimPriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompressPriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConcludePriorityComparison(this, ::windows_core::from_raw_borrowed(&comparison)).into())
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultLongestAcceptableDelay(this, ::core::mem::transmute_copy(&delay)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        IUIAnimationManager2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateAnimationVectorVariable: CreateAnimationVectorVariable::<Identity, Impl, OFFSET>,
            CreateAnimationVariable: CreateAnimationVariable::<Identity, Impl, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, Impl, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, Impl, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, Impl, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, Impl, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, Impl, OFFSET>,
            EstimateNextEventTime: EstimateNextEventTime::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, Impl, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, Impl, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, Impl, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, Impl, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationManagerEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnManagerStatusChanged(this: &Self::This, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationManagerEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManagerEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationManagerEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManagerEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnManagerStatusChanged(this, ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into())
        }
        IUIAnimationManagerEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnManagerStatusChanged: OnManagerStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationManagerEventHandler2_Impl: ::windows_core::BaseImpl {
    fn OnManagerStatusChanged(this: &Self::This, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationManagerEventHandler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManagerEventHandler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationManagerEventHandler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationManagerEventHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnManagerStatusChanged(this, ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into())
        }
        IUIAnimationManagerEventHandler2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnManagerStatusChanged: OnManagerStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationPrimitiveInterpolation_Impl: ::windows_core::BaseImpl {
    fn AddCubic(this: &Self::This, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::Result<()>;
    fn AddSinusoidal(this: &Self::This, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationPrimitiveInterpolation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationPrimitiveInterpolation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddCubic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddCubic(this, ::core::mem::transmute_copy(&dimension), ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into())
        }
        unsafe extern "system" fn AddSinusoidal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSinusoidal(this, ::core::mem::transmute_copy(&dimension), ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into())
        }
        IUIAnimationPrimitiveInterpolation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddCubic: AddCubic::<Identity, Impl, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationPriorityComparison_Impl: ::windows_core::BaseImpl {
    fn HasPriority(this: &Self::This, scheduledstoryboard: ::core::option::Option<&IUIAnimationStoryboard>, newstoryboard: ::core::option::Option<&IUIAnimationStoryboard>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationPriorityComparison {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPriorityComparison_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationPriorityComparison {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HasPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPriorityComparison_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasPriority(this, ::windows_core::from_raw_borrowed(&scheduledstoryboard), ::windows_core::from_raw_borrowed(&newstoryboard), ::core::mem::transmute_copy(&priorityeffect)).into())
        }
        IUIAnimationPriorityComparison_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, HasPriority: HasPriority::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationPriorityComparison2_Impl: ::windows_core::BaseImpl {
    fn HasPriority(this: &Self::This, scheduledstoryboard: ::core::option::Option<&IUIAnimationStoryboard2>, newstoryboard: ::core::option::Option<&IUIAnimationStoryboard2>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationPriorityComparison2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPriorityComparison2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationPriorityComparison2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HasPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationPriorityComparison2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasPriority(this, ::windows_core::from_raw_borrowed(&scheduledstoryboard), ::windows_core::from_raw_borrowed(&newstoryboard), ::core::mem::transmute_copy(&priorityeffect)).into())
        }
        IUIAnimationPriorityComparison2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, HasPriority: HasPriority::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationStoryboard_Impl: ::windows_core::BaseImpl {
    fn AddTransition(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable>, transition: ::core::option::Option<&IUIAnimationTransition>) -> ::windows_core::Result<()>;
    fn AddKeyframeAtOffset(this: &Self::This, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(this: &Self::This, transition: ::core::option::Option<&IUIAnimationTransition>) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable>, transition: ::core::option::Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable>, transition: ::core::option::Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::Result<()>;
    fn RepeatBetweenKeyframes(this: &Self::This, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows_core::Result<()>;
    fn HoldVariable(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable>) -> ::windows_core::Result<()>;
    fn SetLongestAcceptableDelay(this: &Self::This, delay: f64) -> ::windows_core::Result<()>;
    fn Schedule(this: &Self::This, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_core::Result<()>;
    fn Conclude(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish(this: &Self::This, completiondeadline: f64) -> ::windows_core::Result<()>;
    fn Abandon(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<()>;
    fn GetTag(this: &Self::This, object: *mut ::core::option::Option<::windows_core::IUnknown>, id: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStoryboardEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationStoryboardEventHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationStoryboard {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationStoryboard {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTransition(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition)).into())
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddKeyframeAtOffset(this, ::core::mem::transmute_copy(&existingkeyframe), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(keyframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddKeyframeAfterTransition(this, ::windows_core::from_raw_borrowed(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(keyframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTransitionAtKeyframe(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition), ::core::mem::transmute_copy(&startkeyframe)).into())
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTransitionBetweenKeyframes(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition), ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe)).into())
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RepeatBetweenKeyframes(this, ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe), ::core::mem::transmute_copy(&repetitioncount)).into())
        }
        unsafe extern "system" fn HoldVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HoldVariable(this, ::windows_core::from_raw_borrowed(&variable)).into())
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLongestAcceptableDelay(this, ::core::mem::transmute_copy(&delay)).into())
        }
        unsafe extern "system" fn Schedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Schedule(this, ::core::mem::transmute_copy(&timenow), ::core::mem::transmute_copy(&schedulingresult)).into())
        }
        unsafe extern "system" fn Conclude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Conclude(this).into())
        }
        unsafe extern "system" fn Finish<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish(this, ::core::mem::transmute_copy(&completiondeadline)).into())
        }
        unsafe extern "system" fn Abandon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abandon(this).into())
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTag(this, ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetElapsedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetElapsedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elapsedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStoryboardEventHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAnimationStoryboard_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTransition: AddTransition::<Identity, Impl, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, Impl, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, Impl, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, Impl, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, Impl, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, Impl, OFFSET>,
            HoldVariable: HoldVariable::<Identity, Impl, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            Conclude: Conclude::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, Impl, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAnimationStoryboard2_Impl: ::windows_core::BaseImpl {
    fn AddTransition(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable2>, transition: ::core::option::Option<&IUIAnimationTransition2>) -> ::windows_core::Result<()>;
    fn AddKeyframeAtOffset(this: &Self::This, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(this: &Self::This, transition: ::core::option::Option<&IUIAnimationTransition2>) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable2>, transition: ::core::option::Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable2>, transition: ::core::option::Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::Result<()>;
    fn RepeatBetweenKeyframes(this: &Self::This, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::core::option::Option<&IUIAnimationLoopIterationChangeHandler2>, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn HoldVariable(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable2>) -> ::windows_core::Result<()>;
    fn SetLongestAcceptableDelay(this: &Self::This, delay: f64) -> ::windows_core::Result<()>;
    fn SetSkipDuration(this: &Self::This, secondsduration: f64) -> ::windows_core::Result<()>;
    fn Schedule(this: &Self::This, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_core::Result<()>;
    fn Conclude(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish(this: &Self::This, completiondeadline: f64) -> ::windows_core::Result<()>;
    fn Abandon(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<()>;
    fn GetTag(this: &Self::This, object: *mut ::core::option::Option<::windows_core::IUnknown>, id: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStoryboardEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationStoryboardEventHandler2>, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAnimationStoryboard2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationStoryboard2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTransition(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition)).into())
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddKeyframeAtOffset(this, ::core::mem::transmute_copy(&existingkeyframe), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(keyframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddKeyframeAfterTransition(this, ::windows_core::from_raw_borrowed(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(keyframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTransitionAtKeyframe(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition), ::core::mem::transmute_copy(&startkeyframe)).into())
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTransitionBetweenKeyframes(this, ::windows_core::from_raw_borrowed(&variable), ::windows_core::from_raw_borrowed(&transition), ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe)).into())
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut ::core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RepeatBetweenKeyframes(this, ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe), ::core::mem::transmute_copy(&crepetition), ::core::mem::transmute_copy(&repeatmode), ::windows_core::from_raw_borrowed(&piterationchangehandler), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into())
        }
        unsafe extern "system" fn HoldVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HoldVariable(this, ::windows_core::from_raw_borrowed(&variable)).into())
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLongestAcceptableDelay(this, ::core::mem::transmute_copy(&delay)).into())
        }
        unsafe extern "system" fn SetSkipDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSkipDuration(this, ::core::mem::transmute_copy(&secondsduration)).into())
        }
        unsafe extern "system" fn Schedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Schedule(this, ::core::mem::transmute_copy(&timenow), ::core::mem::transmute_copy(&schedulingresult)).into())
        }
        unsafe extern "system" fn Conclude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Conclude(this).into())
        }
        unsafe extern "system" fn Finish<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish(this, ::core::mem::transmute_copy(&completiondeadline)).into())
        }
        unsafe extern "system" fn Abandon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abandon(this).into())
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTag(this, ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetElapsedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetElapsedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elapsedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStoryboardEventHandler(this, ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&fregisterstatuschangefornextanimationevent), ::core::mem::transmute_copy(&fregisterupdatefornextanimationevent)).into())
        }
        IUIAnimationStoryboard2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTransition: AddTransition::<Identity, Impl, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, Impl, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, Impl, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, Impl, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, Impl, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, Impl, OFFSET>,
            HoldVariable: HoldVariable::<Identity, Impl, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            SetSkipDuration: SetSkipDuration::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            Conclude: Conclude::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, Impl, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationStoryboardEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnStoryboardStatusChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::Result<()>;
    fn OnStoryboardUpdated(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationStoryboardEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationStoryboardEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStoryboardStatusChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into())
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStoryboardUpdated(this, ::windows_core::from_raw_borrowed(&storyboard)).into())
        }
        IUIAnimationStoryboardEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, Impl, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationStoryboardEventHandler2_Impl: ::windows_core::BaseImpl {
    fn OnStoryboardStatusChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard2>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::Result<()>;
    fn OnStoryboardUpdated(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard2>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationStoryboardEventHandler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationStoryboardEventHandler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStoryboardStatusChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into())
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStoryboardUpdated(this, ::windows_core::from_raw_borrowed(&storyboard)).into())
        }
        IUIAnimationStoryboardEventHandler2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, Impl, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTimer_Impl: ::windows_core::BaseImpl {
    fn SetTimerUpdateHandler(this: &Self::This, updatehandler: ::core::option::Option<&IUIAnimationTimerUpdateHandler>, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows_core::Result<()>;
    fn SetTimerEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationTimerEventHandler>) -> ::windows_core::Result<()>;
    fn Enable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disable(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetFrameRateThreshold(this: &Self::This, framespersecond: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationTimer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTimer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTimerUpdateHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatehandler: *mut ::core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimerUpdateHandler(this, ::windows_core::from_raw_borrowed(&updatehandler), ::core::mem::transmute_copy(&idlebehavior)).into())
        }
        unsafe extern "system" fn SetTimerEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimerEventHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this).into())
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disable(this).into())
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEnabled(this).into())
        }
        unsafe extern "system" fn GetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(seconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameRateThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameRateThreshold(this, ::core::mem::transmute_copy(&framespersecond)).into())
        }
        IUIAnimationTimer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTimerUpdateHandler: SetTimerUpdateHandler::<Identity, Impl, OFFSET>,
            SetTimerEventHandler: SetTimerEventHandler::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            SetFrameRateThreshold: SetFrameRateThreshold::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTimerClientEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnTimerClientStatusChanged(this: &Self::This, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationTimerClientEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTimerClientEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTimerClientStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTimerClientStatusChanged(this, ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into())
        }
        IUIAnimationTimerClientEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTimerEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnPreUpdate(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnPostUpdate(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnRenderingTooSlow(this: &Self::This, framespersecond: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationTimerEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTimerEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPreUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPreUpdate(this).into())
        }
        unsafe extern "system" fn OnPostUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPostUpdate(this).into())
        }
        unsafe extern "system" fn OnRenderingTooSlow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRenderingTooSlow(this, ::core::mem::transmute_copy(&framespersecond)).into())
        }
        IUIAnimationTimerEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPreUpdate: OnPreUpdate::<Identity, Impl, OFFSET>,
            OnPostUpdate: OnPostUpdate::<Identity, Impl, OFFSET>,
            OnRenderingTooSlow: OnRenderingTooSlow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTimerUpdateHandler_Impl: ::windows_core::BaseImpl {
    fn OnUpdate(this: &Self::This, timenow: f64) -> ::windows_core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn SetTimerClientEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationTimerClientEventHandler>) -> ::windows_core::Result<()>;
    fn ClearTimerClientEventHandler(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationTimerUpdateHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTimerUpdateHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnUpdate(this, ::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimerClientEventHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearTimerClientEventHandler(this).into())
        }
        IUIAnimationTimerUpdateHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnUpdate: OnUpdate::<Identity, Impl, OFFSET>,
            SetTimerClientEventHandler: SetTimerClientEventHandler::<Identity, Impl, OFFSET>,
            ClearTimerClientEventHandler: ClearTimerClientEventHandler::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTransition_Impl: ::windows_core::BaseImpl {
    fn SetInitialValue(this: &Self::This, value: f64) -> ::windows_core::Result<()>;
    fn SetInitialVelocity(this: &Self::This, velocity: f64) -> ::windows_core::Result<()>;
    fn IsDurationKnown(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDuration(this: &Self::This) -> ::windows_core::Result<f64>;
}
impl ::windows_core::Iids for IUIAnimationTransition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTransition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInitialValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialValue(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialVelocity(this, ::core::mem::transmute_copy(&velocity)).into())
        }
        unsafe extern "system" fn IsDurationKnown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDurationKnown(this).into())
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(duration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAnimationTransition_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInitialValue: SetInitialValue::<Identity, Impl, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, Impl, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTransition2_Impl: ::windows_core::BaseImpl {
    fn GetDimension(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetInitialValue(this: &Self::This, value: f64) -> ::windows_core::Result<()>;
    fn SetInitialVectorValue(this: &Self::This, value: *const f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn SetInitialVelocity(this: &Self::This, velocity: f64) -> ::windows_core::Result<()>;
    fn SetInitialVectorVelocity(this: &Self::This, velocity: *const f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn IsDurationKnown(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDuration(this: &Self::This) -> ::windows_core::Result<f64>;
}
impl ::windows_core::Iids for IUIAnimationTransition2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTransition2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDimension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDimension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialValue(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetInitialVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialVectorValue(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialVelocity(this, ::core::mem::transmute_copy(&velocity)).into())
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialVectorVelocity(this, ::core::mem::transmute_copy(&velocity), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn IsDurationKnown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDurationKnown(this).into())
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(duration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAnimationTransition2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            SetInitialValue: SetInitialValue::<Identity, Impl, OFFSET>,
            SetInitialVectorValue: SetInitialVectorValue::<Identity, Impl, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, Impl, OFFSET>,
            SetInitialVectorVelocity: SetInitialVectorVelocity::<Identity, Impl, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTransitionFactory_Impl: ::windows_core::BaseImpl {
    fn CreateTransition(this: &Self::This, interpolator: ::core::option::Option<&IUIAnimationInterpolator>) -> ::windows_core::Result<IUIAnimationTransition>;
}
impl ::windows_core::Iids for IUIAnimationTransitionFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTransitionFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransition(this, ::windows_core::from_raw_borrowed(&interpolator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAnimationTransitionFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTransition: CreateTransition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTransitionFactory2_Impl: ::windows_core::BaseImpl {
    fn CreateTransition(this: &Self::This, interpolator: ::core::option::Option<&IUIAnimationInterpolator2>) -> ::windows_core::Result<IUIAnimationTransition2>;
}
impl ::windows_core::Iids for IUIAnimationTransitionFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTransitionFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransition(this, ::windows_core::from_raw_borrowed(&interpolator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAnimationTransitionFactory2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTransition: CreateTransition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTransitionLibrary_Impl: ::windows_core::BaseImpl {
    fn CreateInstantaneousTransition(this: &Self::This, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateConstantTransition(this: &Self::This, duration: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateDiscreteTransition(this: &Self::This, delay: f64, finalvalue: f64, hold: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransition(this: &Self::This, duration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransitionFromSpeed(this: &Self::This, speed: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromVelocity(this: &Self::This, duration: f64, period: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromRange(this: &Self::This, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateAccelerateDecelerateTransition(this: &Self::This, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateReversalTransition(this: &Self::This, duration: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateCubicTransition(this: &Self::This, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateSmoothStopTransition(this: &Self::This, maximumduration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition>;
    fn CreateParabolicTransitionFromAcceleration(this: &Self::This, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows_core::Result<IUIAnimationTransition>;
}
impl ::windows_core::Iids for IUIAnimationTransitionLibrary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTransitionLibrary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstantaneousTransition(this, ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateConstantTransition(this, ::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDiscreteTransition(this, ::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearTransitionFromSpeed(this, ::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSinusoidalTransitionFromVelocity(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSinusoidalTransitionFromRange(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&minimumvalue), ::core::mem::transmute_copy(&maximumvalue), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&slope)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAccelerateDecelerateTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&accelerationratio), ::core::mem::transmute_copy(&decelerationratio)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReversalTransition(this, ::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCubicTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSmoothStopTransition(this, ::core::mem::transmute_copy(&maximumduration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateParabolicTransitionFromAcceleration(this, ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&acceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAnimationTransitionLibrary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, Impl, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, Impl, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, Impl, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, Impl, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, Impl, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationTransitionLibrary2_Impl: ::windows_core::BaseImpl {
    fn CreateInstantaneousTransition(this: &Self::This, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateInstantaneousVectorTransition(this: &Self::This, finalvalue: *const f64, cdimension: u32) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateConstantTransition(this: &Self::This, duration: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteTransition(this: &Self::This, delay: f64, finalvalue: f64, hold: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteVectorTransition(this: &Self::This, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransition(this: &Self::This, duration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransition(this: &Self::This, duration: f64, finalvalue: *const f64, cdimension: u32) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransitionFromSpeed(this: &Self::This, speed: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransitionFromSpeed(this: &Self::This, speed: f64, finalvalue: *const f64, cdimension: u32) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromVelocity(this: &Self::This, duration: f64, period: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromRange(this: &Self::This, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateAccelerateDecelerateTransition(this: &Self::This, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateReversalTransition(this: &Self::This, duration: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicTransition(this: &Self::This, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicVectorTransition(this: &Self::This, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateSmoothStopTransition(this: &Self::This, maximumduration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateParabolicTransitionFromAcceleration(this: &Self::This, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearTransition(this: &Self::This, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearVectorTransition(this: &Self::This, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows_core::Result<IUIAnimationTransition2>;
}
impl ::windows_core::Iids for IUIAnimationTransitionLibrary2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationTransitionLibrary2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstantaneousTransition(this, ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstantaneousVectorTransition(this, ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateConstantTransition(this, ::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDiscreteTransition(this, ::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDiscreteVectorTransition(this, ::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearVectorTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearTransitionFromSpeed(this, ::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearVectorTransitionFromSpeed(this, ::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSinusoidalTransitionFromVelocity(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSinusoidalTransitionFromRange(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&minimumvalue), ::core::mem::transmute_copy(&maximumvalue), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&slope)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAccelerateDecelerateTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&accelerationratio), ::core::mem::transmute_copy(&decelerationratio)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReversalTransition(this, ::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCubicTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCubicVectorTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSmoothStopTransition(this, ::core::mem::transmute_copy(&maximumduration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateParabolicTransitionFromAcceleration(this, ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&acceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCubicBezierLinearTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&x1), ::core::mem::transmute_copy(&y1), ::core::mem::transmute_copy(&x2), ::core::mem::transmute_copy(&y2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCubicBezierLinearVectorTransition(this, ::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension), ::core::mem::transmute_copy(&x1), ::core::mem::transmute_copy(&y1), ::core::mem::transmute_copy(&x2), ::core::mem::transmute_copy(&y2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAnimationTransitionLibrary2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, Impl, OFFSET>,
            CreateInstantaneousVectorTransition: CreateInstantaneousVectorTransition::<Identity, Impl, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteVectorTransition: CreateDiscreteVectorTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, Impl, OFFSET>,
            CreateLinearVectorTransition: CreateLinearVectorTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateLinearVectorTransitionFromSpeed: CreateLinearVectorTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, Impl, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, Impl, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, Impl, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, Impl, OFFSET>,
            CreateCubicVectorTransition: CreateCubicVectorTransition::<Identity, Impl, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, Impl, OFFSET>,
            CreateCubicBezierLinearTransition: CreateCubicBezierLinearTransition::<Identity, Impl, OFFSET>,
            CreateCubicBezierLinearVectorTransition: CreateCubicBezierLinearVectorTransition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationVariable_Impl: ::windows_core::BaseImpl {
    fn GetValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetFinalValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetPreviousValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetIntegerValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetFinalIntegerValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetPreviousIntegerValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetCurrentStoryboard(this: &Self::This) -> ::windows_core::Result<IUIAnimationStoryboard>;
    fn SetLowerBound(this: &Self::This, bound: f64) -> ::windows_core::Result<()>;
    fn SetUpperBound(this: &Self::This, bound: f64) -> ::windows_core::Result<()>;
    fn SetRoundingMode(this: &Self::This, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::Result<()>;
    fn SetTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<()>;
    fn GetTag(this: &Self::This, object: *mut ::core::option::Option<::windows_core::IUnknown>, id: *mut u32) -> ::windows_core::Result<()>;
    fn SetVariableChangeHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationVariableChangeHandler>) -> ::windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationVariableIntegerChangeHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationVariable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFinalValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(finalvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviousValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previousvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIntegerValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFinalIntegerValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(finalvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousIntegerValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previousvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentStoryboard(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyboard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLowerBound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowerBound(this, ::core::mem::transmute_copy(&bound)).into())
        }
        unsafe extern "system" fn SetUpperBound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpperBound(this, ::core::mem::transmute_copy(&bound)).into())
        }
        unsafe extern "system" fn SetRoundingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRoundingMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTag(this, ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVariableChangeHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVariableIntegerChangeHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAnimationVariable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, Impl, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, Impl, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, Impl, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, Impl, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, Impl, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_DirectComposition\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
pub trait IUIAnimationVariable2_Impl: ::windows_core::BaseImpl {
    fn GetDimension(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetVectorValue(this: &Self::This, value: *mut f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetCurve(this: &Self::This, animation: ::core::option::Option<&super::super::Graphics::DirectComposition::IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn GetVectorCurve(this: &Self::This, animation: *const ::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetFinalValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetFinalVectorValue(this: &Self::This, finalvalue: *mut f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetPreviousValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetPreviousVectorValue(this: &Self::This, previousvalue: *mut f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetIntegerValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetIntegerVectorValue(this: &Self::This, value: *mut i32, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetFinalIntegerValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetFinalIntegerVectorValue(this: &Self::This, finalvalue: *mut i32, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetPreviousIntegerValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetPreviousIntegerVectorValue(this: &Self::This, previousvalue: *mut i32, cdimension: u32) -> ::windows_core::Result<()>;
    fn GetCurrentStoryboard(this: &Self::This) -> ::windows_core::Result<IUIAnimationStoryboard2>;
    fn SetLowerBound(this: &Self::This, bound: f64) -> ::windows_core::Result<()>;
    fn SetLowerBoundVector(this: &Self::This, bound: *const f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn SetUpperBound(this: &Self::This, bound: f64) -> ::windows_core::Result<()>;
    fn SetUpperBoundVector(this: &Self::This, bound: *const f64, cdimension: u32) -> ::windows_core::Result<()>;
    fn SetRoundingMode(this: &Self::This, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::Result<()>;
    fn SetTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<()>;
    fn GetTag(this: &Self::This, object: *mut ::core::option::Option<::windows_core::IUnknown>, id: *mut u32) -> ::windows_core::Result<()>;
    fn SetVariableChangeHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationVariableChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationVariableIntegerChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetVariableCurveChangeHandler(this: &Self::This, handler: ::core::option::Option<&IUIAnimationVariableCurveChangeHandler2>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
impl ::windows_core::Iids for IUIAnimationVariable2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariable2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDimension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDimension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVectorValue(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetCurve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurve(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn GetVectorCurve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *const *mut ::core::ffi::c_void, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVectorCurve(this, ::core::mem::transmute_copy(&animation), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFinalValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(finalvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFinalVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFinalVectorValue(this, ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetPreviousValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previousvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviousVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreviousVectorValue(this, ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIntegerValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIntegerVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIntegerVectorValue(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFinalIntegerValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(finalvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFinalIntegerVectorValue(this, ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousIntegerValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previousvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreviousIntegerVectorValue(this, ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentStoryboard(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyboard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLowerBound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowerBound(this, ::core::mem::transmute_copy(&bound)).into())
        }
        unsafe extern "system" fn SetLowerBoundVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowerBoundVector(this, ::core::mem::transmute_copy(&bound), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn SetUpperBound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpperBound(this, ::core::mem::transmute_copy(&bound)).into())
        }
        unsafe extern "system" fn SetUpperBoundVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpperBoundVector(this, ::core::mem::transmute_copy(&bound), ::core::mem::transmute_copy(&cdimension)).into())
        }
        unsafe extern "system" fn SetRoundingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRoundingMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTag(this, ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVariableChangeHandler(this, ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into())
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVariableIntegerChangeHandler(this, ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into())
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVariableCurveChangeHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAnimationVariable2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetVectorValue: GetVectorValue::<Identity, Impl, OFFSET>,
            GetCurve: GetCurve::<Identity, Impl, OFFSET>,
            GetVectorCurve: GetVectorCurve::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            GetFinalVectorValue: GetFinalVectorValue::<Identity, Impl, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, Impl, OFFSET>,
            GetPreviousVectorValue: GetPreviousVectorValue::<Identity, Impl, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, Impl, OFFSET>,
            GetIntegerVectorValue: GetIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerVectorValue: GetFinalIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerVectorValue: GetPreviousIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, Impl, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, Impl, OFFSET>,
            SetLowerBoundVector: SetLowerBoundVector::<Identity, Impl, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, Impl, OFFSET>,
            SetUpperBoundVector: SetUpperBoundVector::<Identity, Impl, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableCurveChangeHandler: SetVariableCurveChangeHandler::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationVariableChangeHandler_Impl: ::windows_core::BaseImpl {
    fn OnValueChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard>, variable: ::core::option::Option<&IUIAnimationVariable>, newvalue: f64, previousvalue: f64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationVariableChangeHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariableChangeHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: f64, previousvalue: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnValueChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::windows_core::from_raw_borrowed(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue)).into())
        }
        IUIAnimationVariableChangeHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationVariableChangeHandler2_Impl: ::windows_core::BaseImpl {
    fn OnValueChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard2>, variable: ::core::option::Option<&IUIAnimationVariable2>, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationVariableChangeHandler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariableChangeHandler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnValueChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::windows_core::from_raw_borrowed(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into())
        }
        IUIAnimationVariableChangeHandler2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationVariableCurveChangeHandler2_Impl: ::windows_core::BaseImpl {
    fn OnCurveChanged(this: &Self::This, variable: ::core::option::Option<&IUIAnimationVariable2>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationVariableCurveChangeHandler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariableCurveChangeHandler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCurveChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCurveChanged(this, ::windows_core::from_raw_borrowed(&variable)).into())
        }
        IUIAnimationVariableCurveChangeHandler2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnCurveChanged: OnCurveChanged::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationVariableIntegerChangeHandler_Impl: ::windows_core::BaseImpl {
    fn OnIntegerValueChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard>, variable: ::core::option::Option<&IUIAnimationVariable>, newvalue: i32, previousvalue: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationVariableIntegerChangeHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariableIntegerChangeHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: i32, previousvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIntegerValueChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::windows_core::from_raw_borrowed(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue)).into())
        }
        IUIAnimationVariableIntegerChangeHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnIntegerValueChanged: OnIntegerValueChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIAnimationVariableIntegerChangeHandler2_Impl: ::windows_core::BaseImpl {
    fn OnIntegerValueChanged(this: &Self::This, storyboard: ::core::option::Option<&IUIAnimationStoryboard2>, variable: ::core::option::Option<&IUIAnimationVariable2>, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAnimationVariableIntegerChangeHandler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAnimationVariableIntegerChangeHandler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIntegerValueChanged(this, ::windows_core::from_raw_borrowed(&storyboard), ::windows_core::from_raw_borrowed(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into())
        }
        IUIAnimationVariableIntegerChangeHandler2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnIntegerValueChanged: OnIntegerValueChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
