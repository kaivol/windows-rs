pub trait IBackgroundCondition_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IBackgroundCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCondition {
    const VTABLE: Self::Vtable = { IBackgroundCondition_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundTask_Impl: ::windows_core::BaseImpl {
    fn Run(this: &Self::This, taskinstance: ::core::option::Option<&IBackgroundTaskInstance>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundTask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskinstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Run(this, ::windows_core::from_raw_borrowed(&taskinstance)).into())
        }
        IBackgroundTask_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Run: Run::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance_Impl: ::windows_core::BaseImpl {
    fn InstanceId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Task(this: &Self::This) -> ::windows_core::Result<BackgroundTaskRegistration>;
    fn Progress(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetProgress(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn TriggerDetails(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Canceled(this: &Self::This, cancelhandler: ::core::option::Option<&BackgroundTaskCanceledEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn SuspendedCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDeferral(this: &Self::This) -> ::windows_core::Result<BackgroundTaskDeferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IBackgroundTaskInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTaskInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstanceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Task<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Task(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Progress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProgress(this, value).into())
        }
        unsafe extern "system" fn TriggerDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TriggerDetails(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Canceled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cancelhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Canceled(this, ::windows_core::from_raw_borrowed(&cancelhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveCanceled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveCanceled(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn SuspendedCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SuspendedCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeferral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTaskInstance_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstanceId: InstanceId::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            SetProgress: SetProgress::<Identity, Impl, OFFSET>,
            TriggerDetails: TriggerDetails::<Identity, Impl, OFFSET>,
            Canceled: Canceled::<Identity, Impl, OFFSET>,
            RemoveCanceled: RemoveCanceled::<Identity, Impl, OFFSET>,
            SuspendedCount: SuspendedCount::<Identity, Impl, OFFSET>,
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance2_Impl: ::windows_core::BaseImpl + IBackgroundTaskInstance_Impl {
    fn GetThrottleCount(this: &Self::This, counter: BackgroundTaskThrottleCounter) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IBackgroundTaskInstance2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IBackgroundTaskInstance as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTaskInstance2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetThrottleCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThrottleCount(this, counter) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTaskInstance2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetThrottleCount: GetThrottleCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"System\"`"]
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IBackgroundTaskInstance4_Impl: ::windows_core::BaseImpl + IBackgroundTaskInstance_Impl {
    fn User(this: &Self::This) -> ::windows_core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows_core::Iids for IBackgroundTaskInstance4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IBackgroundTaskInstance as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTaskInstance4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTaskInstance4_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, User: User::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration_Impl: ::windows_core::BaseImpl {
    fn TaskId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Progress(this: &Self::This, handler: ::core::option::Option<&BackgroundTaskProgressEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn Completed(this: &Self::This, handler: ::core::option::Option<&BackgroundTaskCompletedEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn Unregister(this: &Self::This, canceltask: bool) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IBackgroundTaskRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTaskRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TaskId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TaskId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Progress(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveProgress(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn Completed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Completed(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveCompleted(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unregister(this, canceltask).into())
        }
        IBackgroundTaskRegistration_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TaskId: TaskId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            RemoveProgress: RemoveProgress::<Identity, Impl, OFFSET>,
            Completed: Completed::<Identity, Impl, OFFSET>,
            RemoveCompleted: RemoveCompleted::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration2_Impl: ::windows_core::BaseImpl + IBackgroundTaskRegistration_Impl {
    fn Trigger(this: &Self::This) -> ::windows_core::Result<IBackgroundTrigger>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IBackgroundTaskRegistration2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IBackgroundTaskRegistration as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTaskRegistration2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Trigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trigger(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTaskRegistration2_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Trigger: Trigger::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration3_Impl: ::windows_core::BaseImpl + IBackgroundTaskRegistration_Impl {
    fn TaskGroup(this: &Self::This) -> ::windows_core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IBackgroundTaskRegistration3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IBackgroundTaskRegistration as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTaskRegistration3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TaskGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TaskGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTaskRegistration3_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TaskGroup: TaskGroup::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundTrigger_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IBackgroundTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTrigger {
    const VTABLE: Self::Vtable = { IBackgroundTrigger_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
