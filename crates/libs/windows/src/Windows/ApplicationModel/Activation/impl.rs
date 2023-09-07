pub trait IActivatedEventArgs_Impl: ::windows_core::BaseImpl {
    fn Kind(this: &Self::This) -> ::windows_core::Result<ActivationKind>;
    fn PreviousExecutionState(this: &Self::This) -> ::windows_core::Result<ApplicationExecutionState>;
    fn SplashScreen(this: &Self::This) -> ::windows_core::Result<SplashScreen>;
}
impl ::windows_core::Iids for IActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PreviousExecutionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreviousExecutionState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SplashScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SplashScreen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Kind: Kind::<Identity, Impl, OFFSET>,
            PreviousExecutionState: PreviousExecutionState::<Identity, Impl, OFFSET>,
            SplashScreen: SplashScreen::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"System\"`"]
#[cfg(feature = "System")]
pub trait IActivatedEventArgsWithUser_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn User(this: &Self::This) -> ::windows_core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows_core::Iids for IActivatedEventArgsWithUser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "System")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgsWithUser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivatedEventArgsWithUser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgsWithUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActivatedEventArgsWithUser_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, User: User::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IApplicationViewActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn CurrentlyShownApplicationViewId(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IApplicationViewActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationViewActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApplicationViewActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentlyShownApplicationViewId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationViewActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentlyShownApplicationViewId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IApplicationViewActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentlyShownApplicationViewId: CurrentlyShownApplicationViewId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppointmentsProviderActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Verb(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IAppointmentsProviderActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppointmentsProviderActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Verb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Verb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppointmentsProviderActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Verb: Verb::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn AddAppointmentOperation(this: &Self::This) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows_core::Iids for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IAppointmentsProviderActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAppointmentOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddAppointmentOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAppointmentOperation: AddAppointmentOperation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn RemoveAppointmentOperation(this: &Self::This) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows_core::Iids for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IAppointmentsProviderActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RemoveAppointmentOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoveAppointmentOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RemoveAppointmentOperation: RemoveAppointmentOperation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn ReplaceAppointmentOperation(this: &Self::This) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl ::windows_core::Iids for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IAppointmentsProviderActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReplaceAppointmentOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplaceAppointmentOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReplaceAppointmentOperation: ReplaceAppointmentOperation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn InstanceStartDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn RoamingId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IAppointmentsProviderActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstanceStartDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstanceStartDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoamingId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoamingId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstanceStartDate: InstanceStartDate::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            RoamingId: RoamingId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn TimeToShow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DateTime>;
    fn Duration(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IAppointmentsProviderActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TimeToShow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimeToShow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Duration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Duration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TimeToShow: TimeToShow::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Background\"`"]
#[cfg(feature = "ApplicationModel_Background")]
pub trait IBackgroundActivatedEventArgs_Impl: ::windows_core::BaseImpl {
    fn TaskInstance(this: &Self::This) -> ::windows_core::Result<super::Background::IBackgroundTaskInstance>;
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows_core::Iids for IBackgroundActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "ApplicationModel_Background")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TaskInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TaskInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TaskInstance: TaskInstance::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBarcodeScannerPreviewActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ConnectionId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IBarcodeScannerPreviewActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBarcodeScannerPreviewActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBarcodeScannerPreviewActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ConnectionId: ConnectionId::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Storage_Provider\"`"]
#[cfg(feature = "Storage_Provider")]
pub trait ICachedFileUpdaterActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn CachedFileUpdaterUI(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
#[cfg(feature = "Storage_Provider")]
impl ::windows_core::Iids for ICachedFileUpdaterActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Storage_Provider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICachedFileUpdaterActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CachedFileUpdaterUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFileUpdaterUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICachedFileUpdaterActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CachedFileUpdaterUI: CachedFileUpdaterUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICameraSettingsActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn VideoDeviceController(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn VideoDeviceExtension(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::Iids for ICameraSettingsActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICameraSettingsActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VideoDeviceController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VideoDeviceController(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VideoDeviceExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VideoDeviceExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICameraSettingsActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VideoDeviceController: VideoDeviceController::<Identity, Impl, OFFSET>,
            VideoDeviceExtension: VideoDeviceExtension::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICommandLineActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Operation(this: &Self::This) -> ::windows_core::Result<CommandLineActivationOperation>;
}
impl ::windows_core::Iids for ICommandLineActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandLineActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandLineActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Operation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandLineActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICommandLineActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Operation: Operation::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Verb(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IContactActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Verb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Verb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Verb: Verb::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactCallActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ServiceUserId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Contact(this: &Self::This) -> ::windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows_core::Iids for IContactCallActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContactActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactCallActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceUserId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Contact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Contact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactCallActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMapActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn Address(this: &Self::This) -> ::windows_core::Result<super::Contacts::ContactAddress>;
    fn Contact(this: &Self::This) -> ::windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows_core::Iids for IContactMapActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContactActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMapActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactMapActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMapActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Contact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMapActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Contact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactMapActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Address: Address::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMessageActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ServiceUserId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Contact(this: &Self::This) -> ::windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows_core::Iids for IContactMessageActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContactActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactMessageActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceUserId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Contact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactMessageActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Contact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactMessageActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPanelActivatedEventArgs_Impl: ::windows_core::BaseImpl {
    fn ContactPanel(this: &Self::This) -> ::windows_core::Result<super::Contacts::ContactPanel>;
    fn Contact(this: &Self::This) -> ::windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows_core::Iids for IContactPanelActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPanelActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactPanelActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContactPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPanelActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContactPanel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Contact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPanelActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Contact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactPanelActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContactPanel: ContactPanel::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts_Provider\"`"]
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub trait IContactPickerActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ContactPickerUI(this: &Self::This) -> ::windows_core::Result<super::Contacts::Provider::ContactPickerUI>;
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl ::windows_core::Iids for IContactPickerActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPickerActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactPickerActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContactPickerUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPickerActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContactPickerUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactPickerActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContactPickerUI: ContactPickerUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPostActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ServiceUserId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Contact(this: &Self::This) -> ::windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows_core::Iids for IContactPostActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContactActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactPostActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceUserId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Contact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPostActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Contact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactPostActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactVideoCallActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ServiceUserId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Contact(this: &Self::This) -> ::windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl ::windows_core::Iids for IContactVideoCallActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContactActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactVideoCallActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceUserId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceUserId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Contact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Contact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactVideoCallActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, Impl, OFFSET>,
            Contact: Contact::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactsProviderActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Verb(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IContactsProviderActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactsProviderActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactsProviderActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Verb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactsProviderActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Verb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactsProviderActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Verb: Verb::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IContinuationActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ContinuationData(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IContinuationActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinuationActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContinuationActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContinuationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinuationActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContinuationData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContinuationActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContinuationData: ContinuationData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDeviceActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn DeviceInformationId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Verb(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IDeviceActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceInformationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceInformationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Verb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Verb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDeviceActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceInformationId: DeviceInformationId::<Identity, Impl, OFFSET>,
            Verb: Verb::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Devices_Enumeration\"`"]
#[cfg(feature = "Devices_Enumeration")]
pub trait IDevicePairingActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn DeviceInformation(this: &Self::This) -> ::windows_core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(feature = "Devices_Enumeration")]
impl ::windows_core::Iids for IDevicePairingActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Devices_Enumeration")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDevicePairingActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDevicePairingActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDevicePairingActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDevicePairingActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceInformation: DeviceInformation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDialReceiverActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn AppName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IDialReceiverActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <ILaunchActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialReceiverActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDialReceiverActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AppName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialReceiverActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDialReceiverActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AppName: AppName::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
pub trait IFileActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Files(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl ::windows_core::Iids for IFileActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Files<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Files(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Verb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Verb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Files: Files::<Identity, Impl, OFFSET>,
            Verb: Verb::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyName_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
pub trait IFileActivatedEventArgsWithNeighboringFiles_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IFileActivatedEventArgs_Impl {
    fn NeighboringFilesQuery(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl ::windows_core::Iids for IFileActivatedEventArgsWithNeighboringFiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IFileActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileActivatedEventArgsWithNeighboringFiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NeighboringFilesQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NeighboringFilesQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NeighboringFilesQuery: NeighboringFilesQuery::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileOpenPickerActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn FileOpenPickerUI(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl ::windows_core::Iids for IFileOpenPickerActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileOpenPickerActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FileOpenPickerUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileOpenPickerUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileOpenPickerActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FileOpenPickerUI: FileOpenPickerUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFileOpenPickerActivatedEventArgs2_Impl: ::windows_core::BaseImpl {
    fn CallerPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IFileOpenPickerActivatedEventArgs2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileOpenPickerActivatedEventArgs2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileOpenPickerActivatedEventArgs2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileOpenPickerContinuationEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Files(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows_core::Iids for IFileOpenPickerContinuationEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContinuationActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileOpenPickerContinuationEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Files<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Files(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileOpenPickerContinuationEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Files: Files::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileSavePickerActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn FileSavePickerUI(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl ::windows_core::Iids for IFileSavePickerActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSavePickerActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FileSavePickerUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSavePickerUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileSavePickerActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FileSavePickerUI: FileSavePickerUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFileSavePickerActivatedEventArgs2_Impl: ::windows_core::BaseImpl {
    fn CallerPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn EnterpriseId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IFileSavePickerActivatedEventArgs2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSavePickerActivatedEventArgs2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnterpriseId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnterpriseId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileSavePickerActivatedEventArgs2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
            EnterpriseId: EnterpriseId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileSavePickerContinuationEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn File(this: &Self::This) -> ::windows_core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows_core::Iids for IFileSavePickerContinuationEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContinuationActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSavePickerContinuationEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn File<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::File(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileSavePickerContinuationEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, File: File::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFolderPickerContinuationEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Folder(this: &Self::This) -> ::windows_core::Result<super::super::Storage::StorageFolder>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows_core::Iids for IFolderPickerContinuationEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContinuationActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFolderPickerContinuationEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFolderPickerContinuationEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Folder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFolderPickerContinuationEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Folder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFolderPickerContinuationEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Folder: Folder::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ILaunchActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Arguments(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn TileId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for ILaunchActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILaunchActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILaunchActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Arguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILaunchActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Arguments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TileId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILaunchActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TileId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILaunchActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Arguments: Arguments::<Identity, Impl, OFFSET>,
            TileId: TileId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ILaunchActivatedEventArgs2_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn TileActivatedInfo(this: &Self::This) -> ::windows_core::Result<TileActivatedInfo>;
}
impl ::windows_core::Iids for ILaunchActivatedEventArgs2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <ILaunchActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILaunchActivatedEventArgs2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILaunchActivatedEventArgs2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TileActivatedInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILaunchActivatedEventArgs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TileActivatedInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILaunchActivatedEventArgs2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TileActivatedInfo: TileActivatedInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ILockScreenActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Info(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::Iids for ILockScreenActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockScreenActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILockScreenActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockScreenActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Info(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILockScreenActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Info: Info::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Calls\"`"]
#[cfg(feature = "ApplicationModel_Calls")]
pub trait ILockScreenCallActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn CallUI(this: &Self::This) -> ::windows_core::Result<super::Calls::LockScreenCallUI>;
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ::windows_core::Iids for ILockScreenCallActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <ILaunchActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_Calls")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILockScreenCallActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILockScreenCallActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CallUI: CallUI::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPhoneCallActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn LineId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IPhoneCallActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhoneCallActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhoneCallActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhoneCallActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPhoneCallActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LineId: LineId::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPickerReturnedActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn PickerOperationId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IPickerReturnedActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPickerReturnedActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PickerOperationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PickerOperationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPickerReturnedActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PickerOperationId: PickerOperationId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrelaunchActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn PrelaunchActivated(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IPrelaunchActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrelaunchActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrelaunchActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrelaunchActivated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrelaunchActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrelaunchActivated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrelaunchActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrelaunchActivated: PrelaunchActivated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Devices_Printers_Extensions\"`"]
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrint3DWorkflowActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Workflow(this: &Self::This) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl ::windows_core::Iids for IPrint3DWorkflowActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrint3DWorkflowActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Workflow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Workflow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrint3DWorkflowActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Workflow: Workflow::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Devices_Printers_Extensions\"`"]
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrintTaskSettingsActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Configuration(this: &Self::This) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl ::windows_core::Iids for IPrintTaskSettingsActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTaskSettingsActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Configuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Configuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintTaskSettingsActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Configuration: Configuration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IProtocolActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Uri(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IProtocolActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtocolActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Uri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Uri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProtocolActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Uri: Uri::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Data(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallerPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"System\"`"]
#[cfg(feature = "System")]
pub trait IProtocolForResultsActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ProtocolForResultsOperation(this: &Self::This) -> ::windows_core::Result<super::super::System::ProtocolForResultsOperation>;
}
#[cfg(feature = "System")]
impl ::windows_core::Iids for IProtocolForResultsActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "System")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtocolForResultsActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProtocolForResultsOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProtocolForResultsOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProtocolForResultsActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProtocolForResultsOperation: ProtocolForResultsOperation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRestrictedLaunchActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn SharedContext(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::Iids for IRestrictedLaunchActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRestrictedLaunchActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SharedContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SharedContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRestrictedLaunchActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SharedContext: SharedContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISearchActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn QueryText(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for ISearchActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryText: QueryText::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Search\"`"]
#[cfg(feature = "ApplicationModel_Search")]
pub trait ISearchActivatedEventArgsWithLinguisticDetails_Impl: ::windows_core::BaseImpl {
    fn LinguisticDetails(this: &Self::This) -> ::windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "ApplicationModel_Search")]
impl ::windows_core::Iids for ISearchActivatedEventArgsWithLinguisticDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "ApplicationModel_Search")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchActivatedEventArgsWithLinguisticDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LinguisticDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LinguisticDetails(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LinguisticDetails: LinguisticDetails::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`"]
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub trait IShareTargetActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ShareOperation(this: &Self::This) -> ::windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl ::windows_core::Iids for IShareTargetActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShareTargetActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IShareTargetActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShareOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShareTargetActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShareOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IShareTargetActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ShareOperation: ShareOperation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStartupTaskActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn TaskId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IStartupTaskActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStartupTaskActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStartupTaskActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TaskId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStartupTaskActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TaskId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStartupTaskActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TaskId: TaskId::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IToastNotificationActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Argument(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn UserInput(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IToastNotificationActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToastNotificationActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IToastNotificationActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Argument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToastNotificationActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Argument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToastNotificationActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserInput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IToastNotificationActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Argument: Argument::<Identity, Impl, OFFSET>,
            UserInput: UserInput::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`"]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub trait IUserDataAccountProviderActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Operation(this: &Self::This) -> ::windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl ::windows_core::Iids for IUserDataAccountProviderActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserDataAccountProviderActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Operation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUserDataAccountProviderActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Operation: Operation::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"UI_ViewManagement\"`"]
#[cfg(feature = "UI_ViewManagement")]
pub trait IViewSwitcherProvider_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ViewSwitcher(this: &Self::This) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows_core::Iids for IViewSwitcherProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "UI_ViewManagement")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewSwitcherProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewSwitcherProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ViewSwitcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewSwitcherProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ViewSwitcher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IViewSwitcherProvider_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ViewSwitcher: ViewSwitcher::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Media_SpeechRecognition\"`"]
#[cfg(feature = "Media_SpeechRecognition")]
pub trait IVoiceCommandActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Result(this: &Self::This) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "Media_SpeechRecognition")]
impl ::windows_core::Iids for IVoiceCommandActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Media_SpeechRecognition")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVoiceCommandActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Result<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Result(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVoiceCommandActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Result: Result::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"ApplicationModel_Wallet\"`, `\"deprecated\"`"]
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
pub trait IWalletActionActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn ItemId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ActionKind(this: &Self::This) -> ::windows_core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
impl ::windows_core::Iids for IWalletActionActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWalletActionActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActionKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionKind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWalletActionActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWalletActionActivatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ItemId: ItemId::<Identity, Impl, OFFSET>,
            ActionKind: ActionKind::<Identity, Impl, OFFSET>,
            ActionId: ActionId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Security_Authentication_Web_Provider\"`"]
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub trait IWebAccountProviderActivatedEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl {
    fn Operation(this: &Self::This) -> ::windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl ::windows_core::Iids for IWebAccountProviderActivatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderActivatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Operation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebAccountProviderActivatedEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Operation: Operation::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Security_Authentication_Web\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
pub trait IWebAuthenticationBrokerContinuationEventArgs_Impl: ::windows_core::BaseImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn WebAuthenticationResult(this: &Self::This) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl ::windows_core::Iids for IWebAuthenticationBrokerContinuationEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IActivatedEventArgs as ::windows_core::ComInterface>::IID, <IContinuationActivatedEventArgs as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAuthenticationBrokerContinuationEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WebAuthenticationResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WebAuthenticationResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WebAuthenticationResult: WebAuthenticationResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
