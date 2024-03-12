#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdates_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DetectNow(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn ShowSettingsDialog(this: &Self::This) -> ::windows_core::Result<()>;
    fn Settings(this: &Self::This) -> ::windows_core::Result<IAutomaticUpdatesSettings>;
    fn ServiceEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EnableService(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAutomaticUpdates {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAutomaticUpdates {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DetectNow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectNow(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn ShowSettingsDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowSettingsDialog(this).into())
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Settings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableService(this).into())
        }
        IAutomaticUpdates_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DetectNow: DetectNow::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            ShowSettingsDialog: ShowSettingsDialog::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            ServiceEnabled: ServiceEnabled::<Identity, Impl, OFFSET>,
            EnableService: EnableService::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdates2_Impl: ::windows_core::BaseImpl + IAutomaticUpdates_Impl {
    fn Results(this: &Self::This) -> ::windows_core::Result<IAutomaticUpdatesResults>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAutomaticUpdates2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAutomaticUpdates);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAutomaticUpdates2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Results<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdates2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Results(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAutomaticUpdates2_Vtbl { base__: <IAutomaticUpdates as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Results: Results::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesResults_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn LastSearchSuccessDate(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn LastInstallationSuccessDate(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAutomaticUpdatesResults {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAutomaticUpdatesResults {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LastSearchSuccessDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastSearchSuccessDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastInstallationSuccessDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAutomaticUpdatesResults_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LastSearchSuccessDate: LastSearchSuccessDate::<Identity, Impl, OFFSET>,
            LastInstallationSuccessDate: LastInstallationSuccessDate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesSettings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn NotificationLevel(this: &Self::This) -> ::windows_core::Result<AutomaticUpdatesNotificationLevel>;
    fn SetNotificationLevel(this: &Self::This, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::Result<()>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Required(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ScheduledInstallationDay(this: &Self::This) -> ::windows_core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn SetScheduledInstallationDay(this: &Self::This, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::Result<()>;
    fn ScheduledInstallationTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetScheduledInstallationTime(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAutomaticUpdatesSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAutomaticUpdatesSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotificationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NotificationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotificationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationLevel(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Required<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Required(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScheduledInstallationDay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduledInstallationDay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScheduledInstallationDay(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ScheduledInstallationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScheduledInstallationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScheduledInstallationTime(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        IAutomaticUpdatesSettings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotificationLevel: NotificationLevel::<Identity, Impl, OFFSET>,
            SetNotificationLevel: SetNotificationLevel::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Required: Required::<Identity, Impl, OFFSET>,
            ScheduledInstallationDay: ScheduledInstallationDay::<Identity, Impl, OFFSET>,
            SetScheduledInstallationDay: SetScheduledInstallationDay::<Identity, Impl, OFFSET>,
            ScheduledInstallationTime: ScheduledInstallationTime::<Identity, Impl, OFFSET>,
            SetScheduledInstallationTime: SetScheduledInstallationTime::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesSettings2_Impl: ::windows_core::BaseImpl + IAutomaticUpdatesSettings_Impl {
    fn IncludeRecommendedUpdates(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncludeRecommendedUpdates(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CheckPermission(this: &Self::This, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAutomaticUpdatesSettings2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAutomaticUpdatesSettings);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAutomaticUpdatesSettings2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IncludeRecommendedUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncludeRecommendedUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIncludeRecommendedUpdates(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn CheckPermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckPermission(this, ::core::mem::transmute_copy(&usertype), ::core::mem::transmute_copy(&permissiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(userhaspermission, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAutomaticUpdatesSettings2_Vtbl {
            base__: <IAutomaticUpdatesSettings as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IncludeRecommendedUpdates: IncludeRecommendedUpdates::<Identity, Impl, OFFSET>,
            SetIncludeRecommendedUpdates: SetIncludeRecommendedUpdates::<Identity, Impl, OFFSET>,
            CheckPermission: CheckPermission::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesSettings3_Impl: ::windows_core::BaseImpl + IAutomaticUpdatesSettings2_Impl {
    fn NonAdministratorsElevated(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNonAdministratorsElevated(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FeaturedUpdatesEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFeaturedUpdatesEnabled(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAutomaticUpdatesSettings3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAutomaticUpdatesSettings2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAutomaticUpdatesSettings3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NonAdministratorsElevated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NonAdministratorsElevated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNonAdministratorsElevated(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FeaturedUpdatesEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFeaturedUpdatesEnabled(this, ::core::mem::transmute_copy(&value)).into())
        }
        IAutomaticUpdatesSettings3_Vtbl {
            base__: <IAutomaticUpdatesSettings2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NonAdministratorsElevated: NonAdministratorsElevated::<Identity, Impl, OFFSET>,
            SetNonAdministratorsElevated: SetNonAdministratorsElevated::<Identity, Impl, OFFSET>,
            FeaturedUpdatesEnabled: FeaturedUpdatesEnabled::<Identity, Impl, OFFSET>,
            SetFeaturedUpdatesEnabled: SetFeaturedUpdatesEnabled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICategory_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CategoryID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Children(this: &Self::This) -> ::windows_core::Result<ICategoryCollection>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Image(this: &Self::This) -> ::windows_core::Result<IImageInformation>;
    fn Order(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<ICategory>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Updates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICategory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICategory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CategoryID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CategoryID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Image(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Order<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Order(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Updates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICategory_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            CategoryID: CategoryID::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            Order: Order::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICategoryCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<ICategory>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICategoryCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICategoryCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICategoryCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallback_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, downloadjob: ::core::option::Option<&IDownloadJob>, callbackargs: ::core::option::Option<&IDownloadCompletedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDownloadCompletedCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadCompletedCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadCompletedCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadCompletedCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&downloadjob), ::windows_core::from_raw_borrowed(&callbackargs)).into())
        }
        IDownloadCompletedCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadCompletedCallbackArgs_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDownloadCompletedCallbackArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadCompletedCallbackArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadCompletedCallbackArgs {
    const VTABLE: Self::Vtable = { IDownloadCompletedCallbackArgs_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadJob_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AsyncState(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsCompleted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Updates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn CleanUp(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProgress(this: &Self::This) -> ::windows_core::Result<IDownloadProgress>;
    fn RequestAbort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDownloadJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AsyncState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCompleted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Updates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CleanUp(this).into())
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProgress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAbort(this).into())
        }
        IDownloadJob_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadProgress_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CurrentUpdateBytesDownloaded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateBytesToDownload(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateIndex(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PercentComplete(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalBytesDownloaded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn TotalBytesToDownload(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn GetUpdateResult(this: &Self::This, updateindex: i32) -> ::windows_core::Result<IUpdateDownloadResult>;
    fn CurrentUpdateDownloadPhase(this: &Self::This) -> ::windows_core::Result<DownloadPhase>;
    fn CurrentUpdatePercentComplete(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDownloadProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdateBytesDownloaded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdateBytesToDownload(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentUpdateIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdateIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PercentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PercentComplete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalBytesDownloaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalBytesDownloaded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalBytesToDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalBytesToDownload(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdateResult(this, ::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPhase) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdateDownloadPhase(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdatePercentComplete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDownloadProgress_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentUpdateBytesDownloaded: CurrentUpdateBytesDownloaded::<Identity, Impl, OFFSET>,
            CurrentUpdateBytesToDownload: CurrentUpdateBytesToDownload::<Identity, Impl, OFFSET>,
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, Impl, OFFSET>,
            PercentComplete: PercentComplete::<Identity, Impl, OFFSET>,
            TotalBytesDownloaded: TotalBytesDownloaded::<Identity, Impl, OFFSET>,
            TotalBytesToDownload: TotalBytesToDownload::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
            CurrentUpdateDownloadPhase: CurrentUpdateDownloadPhase::<Identity, Impl, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallback_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, downloadjob: ::core::option::Option<&IDownloadJob>, callbackargs: ::core::option::Option<&IDownloadProgressChangedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDownloadProgressChangedCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadProgressChangedCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&downloadjob), ::windows_core::from_raw_borrowed(&callbackargs)).into())
        }
        IDownloadProgressChangedCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadProgressChangedCallbackArgs_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Progress(this: &Self::This) -> ::windows_core::Result<IDownloadProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDownloadProgressChangedCallbackArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadProgressChangedCallbackArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Progress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDownloadProgressChangedCallbackArgs_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Progress: Progress::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadResult_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn HResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ResultCode(this: &Self::This) -> ::windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(this: &Self::This, updateindex: i32) -> ::windows_core::Result<IUpdateDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDownloadResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdateResult(this, ::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDownloadResult_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HResult: HResult::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IImageInformation_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AltText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Height(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Source(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Width(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IImageInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AltText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AltText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Height(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Source<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Source(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Width(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IImageInformation_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AltText: AltText::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Source: Source::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationAgent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn RecordInstallationResult(this: &Self::This, installationresultcookie: &::windows_core::BSTR, hresult: i32, extendedreportingdata: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationAgent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationAgent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationAgent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RecordInstallationResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installationresultcookie: ::std::mem::MaybeUninit<::windows_core::BSTR>, hresult: i32, extendedreportingdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordInstallationResult(this, ::core::mem::transmute(&installationresultcookie), ::core::mem::transmute_copy(&hresult), ::windows_core::from_raw_borrowed(&extendedreportingdata)).into())
        }
        IInstallationAgent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RecordInstallationResult: RecordInstallationResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationBehavior_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CanRequestUserInput(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Impact(this: &Self::This) -> ::windows_core::Result<InstallationImpact>;
    fn RebootBehavior(this: &Self::This) -> ::windows_core::Result<InstallationRebootBehavior>;
    fn RequiresNetworkConnectivity(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationBehavior {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationBehavior {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanRequestUserInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanRequestUserInput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Impact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationImpact) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Impact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RebootBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootBehavior(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequiresNetworkConnectivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInstallationBehavior_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanRequestUserInput: CanRequestUserInput::<Identity, Impl, OFFSET>,
            Impact: Impact::<Identity, Impl, OFFSET>,
            RebootBehavior: RebootBehavior::<Identity, Impl, OFFSET>,
            RequiresNetworkConnectivity: RequiresNetworkConnectivity::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallback_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, installationjob: ::core::option::Option<&IInstallationJob>, callbackargs: ::core::option::Option<&IInstallationCompletedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IInstallationCompletedCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationCompletedCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationCompletedCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationCompletedCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installationjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&installationjob), ::windows_core::from_raw_borrowed(&callbackargs)).into())
        }
        IInstallationCompletedCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationCompletedCallbackArgs_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationCompletedCallbackArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationCompletedCallbackArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationCompletedCallbackArgs {
    const VTABLE: Self::Vtable = { IInstallationCompletedCallbackArgs_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationJob_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AsyncState(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsCompleted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Updates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn CleanUp(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProgress(this: &Self::This) -> ::windows_core::Result<IInstallationProgress>;
    fn RequestAbort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AsyncState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCompleted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Updates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CleanUp(this).into())
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProgress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAbort(this).into())
        }
        IInstallationJob_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationProgress_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CurrentUpdateIndex(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentUpdatePercentComplete(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PercentComplete(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetUpdateResult(this: &Self::This, updateindex: i32) -> ::windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentUpdateIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdateIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUpdatePercentComplete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PercentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PercentComplete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdateResult(this, ::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInstallationProgress_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, Impl, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, Impl, OFFSET>,
            PercentComplete: PercentComplete::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallback_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, installationjob: ::core::option::Option<&IInstallationJob>, callbackargs: ::core::option::Option<&IInstallationProgressChangedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IInstallationProgressChangedCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationProgressChangedCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installationjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&installationjob), ::windows_core::from_raw_borrowed(&callbackargs)).into())
        }
        IInstallationProgressChangedCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationProgressChangedCallbackArgs_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Progress(this: &Self::This) -> ::windows_core::Result<IInstallationProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationProgressChangedCallbackArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationProgressChangedCallbackArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Progress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInstallationProgressChangedCallbackArgs_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Progress: Progress::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationResult_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn HResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RebootRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ResultCode(this: &Self::This) -> ::windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(this: &Self::This, updateindex: i32) -> ::windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInstallationResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInstallationResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdateResult(this, ::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInstallationResult_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HResult: HResult::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInvalidProductLicenseException_Impl: ::windows_core::BaseImpl + IUpdateException_Impl {
    fn Product(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInvalidProductLicenseException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateException);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInvalidProductLicenseException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInvalidProductLicenseException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Product<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInvalidProductLicenseException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Product(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInvalidProductLicenseException_Vtbl { base__: <IUpdateException as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Product: Product::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallback_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, searchjob: ::core::option::Option<&ISearchJob>, callbackargs: ::core::option::Option<&ISearchCompletedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISearchCompletedCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCompletedCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchCompletedCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCompletedCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&searchjob), ::windows_core::from_raw_borrowed(&callbackargs)).into())
        }
        ISearchCompletedCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISearchCompletedCallbackArgs_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchCompletedCallbackArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCompletedCallbackArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchCompletedCallbackArgs {
    const VTABLE: Self::Vtable = { ISearchCompletedCallbackArgs_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISearchJob_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AsyncState(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsCompleted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CleanUp(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestAbort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AsyncState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCompleted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CleanUp(this).into())
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAbort(this).into())
        }
        ISearchJob_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISearchResult_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ResultCode(this: &Self::This) -> ::windows_core::Result<OperationResultCode>;
    fn RootCategories(this: &Self::This) -> ::windows_core::Result<ICategoryCollection>;
    fn Updates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn Warnings(this: &Self::This) -> ::windows_core::Result<IUpdateExceptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RootCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootCategories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Updates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Warnings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Warnings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchResult_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            RootCategories: RootCategories::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            Warnings: Warnings::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IStringCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_Item(this: &Self::This, index: i32, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn Insert(this: &Self::This, index: i32, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IStringCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStringCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Item(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Copy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        IStringCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            put_Item: put_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISystemInformation_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn OemHardwareSupportLink(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RebootRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISystemInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISystemInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OemHardwareSupportLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OemHardwareSupportLink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISystemInformation_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OemHardwareSupportLink: OemHardwareSupportLink::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AutoSelectOnWebSites(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BundledUpdates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn CanRequireSource(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Categories(this: &Self::This) -> ::windows_core::Result<ICategoryCollection>;
    fn Deadline(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DeltaCompressedContentAvailable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DeltaCompressedContentPreferred(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EulaAccepted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EulaText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HandlerID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Identity(this: &Self::This) -> ::windows_core::Result<IUpdateIdentity>;
    fn Image(this: &Self::This) -> ::windows_core::Result<IImageInformation>;
    fn InstallationBehavior(this: &Self::This) -> ::windows_core::Result<IInstallationBehavior>;
    fn IsBeta(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsDownloaded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsHidden(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsInstalled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsMandatory(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUninstallable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Languages(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn LastDeploymentChangeTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn MaxDownloadSize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn MinDownloadSize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn MoreInfoUrls(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn MsrcSeverity(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RecommendedCpuSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RecommendedHardDiskSpace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RecommendedMemory(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ReleaseNotes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SecurityBulletinIDs(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn SupersededUpdateIDs(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn SupportUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(this: &Self::This) -> ::windows_core::Result<UpdateType>;
    fn UninstallationNotes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UninstallationBehavior(this: &Self::This) -> ::windows_core::Result<IInstallationBehavior>;
    fn UninstallationSteps(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn KBArticleIDs(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn AcceptEula(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeploymentAction(this: &Self::This) -> ::windows_core::Result<DeploymentAction>;
    fn CopyFromCache(this: &Self::This, path: &::windows_core::BSTR, toextractcabfiles: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DownloadPriority(this: &Self::This) -> ::windows_core::Result<DownloadPriority>;
    fn DownloadContents(this: &Self::This) -> ::windows_core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoSelectOnWebSites(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BundledUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BundledUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanRequireSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanRequireSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Categories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Categories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Deadline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Deadline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeltaCompressedContentAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeltaCompressedContentPreferred(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EulaAccepted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EulaAccepted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EulaText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EulaText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HandlerID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HandlerID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Identity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Identity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Image(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallationBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallationBehavior(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsBeta<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBeta(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDownloaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDownloaded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsHidden(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsHidden(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn IsInstalled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInstalled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsMandatory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMandatory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUninstallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUninstallable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Languages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Languages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDeploymentChangeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxDownloadSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxDownloadSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinDownloadSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinDownloadSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoreInfoUrls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoreInfoUrls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MsrcSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MsrcSeverity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecommendedCpuSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecommendedHardDiskSpace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RecommendedMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecommendedMemory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseNotes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReleaseNotes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SecurityBulletinIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecurityBulletinIDs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupersededUpdateIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupersededUpdateIDs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallationNotes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UninstallationNotes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallationBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UninstallationBehavior(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallationSteps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UninstallationSteps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KBArticleIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KBArticleIDs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AcceptEula<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcceptEula(this).into())
        }
        unsafe extern "system" fn DeploymentAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DeploymentAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeploymentAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyFromCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, toextractcabfiles: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyFromCache(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&toextractcabfiles)).into())
        }
        unsafe extern "system" fn DownloadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DownloadContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadContents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdate_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Title: Title::<Identity, Impl, OFFSET>,
            AutoSelectOnWebSites: AutoSelectOnWebSites::<Identity, Impl, OFFSET>,
            BundledUpdates: BundledUpdates::<Identity, Impl, OFFSET>,
            CanRequireSource: CanRequireSource::<Identity, Impl, OFFSET>,
            Categories: Categories::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
            DeltaCompressedContentAvailable: DeltaCompressedContentAvailable::<Identity, Impl, OFFSET>,
            DeltaCompressedContentPreferred: DeltaCompressedContentPreferred::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            EulaAccepted: EulaAccepted::<Identity, Impl, OFFSET>,
            EulaText: EulaText::<Identity, Impl, OFFSET>,
            HandlerID: HandlerID::<Identity, Impl, OFFSET>,
            Identity: Identity::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            InstallationBehavior: InstallationBehavior::<Identity, Impl, OFFSET>,
            IsBeta: IsBeta::<Identity, Impl, OFFSET>,
            IsDownloaded: IsDownloaded::<Identity, Impl, OFFSET>,
            IsHidden: IsHidden::<Identity, Impl, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, Impl, OFFSET>,
            IsInstalled: IsInstalled::<Identity, Impl, OFFSET>,
            IsMandatory: IsMandatory::<Identity, Impl, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            LastDeploymentChangeTime: LastDeploymentChangeTime::<Identity, Impl, OFFSET>,
            MaxDownloadSize: MaxDownloadSize::<Identity, Impl, OFFSET>,
            MinDownloadSize: MinDownloadSize::<Identity, Impl, OFFSET>,
            MoreInfoUrls: MoreInfoUrls::<Identity, Impl, OFFSET>,
            MsrcSeverity: MsrcSeverity::<Identity, Impl, OFFSET>,
            RecommendedCpuSpeed: RecommendedCpuSpeed::<Identity, Impl, OFFSET>,
            RecommendedHardDiskSpace: RecommendedHardDiskSpace::<Identity, Impl, OFFSET>,
            RecommendedMemory: RecommendedMemory::<Identity, Impl, OFFSET>,
            ReleaseNotes: ReleaseNotes::<Identity, Impl, OFFSET>,
            SecurityBulletinIDs: SecurityBulletinIDs::<Identity, Impl, OFFSET>,
            SupersededUpdateIDs: SupersededUpdateIDs::<Identity, Impl, OFFSET>,
            SupportUrl: SupportUrl::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, Impl, OFFSET>,
            UninstallationBehavior: UninstallationBehavior::<Identity, Impl, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, Impl, OFFSET>,
            KBArticleIDs: KBArticleIDs::<Identity, Impl, OFFSET>,
            AcceptEula: AcceptEula::<Identity, Impl, OFFSET>,
            DeploymentAction: DeploymentAction::<Identity, Impl, OFFSET>,
            CopyFromCache: CopyFromCache::<Identity, Impl, OFFSET>,
            DownloadPriority: DownloadPriority::<Identity, Impl, OFFSET>,
            DownloadContents: DownloadContents::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate2_Impl: ::windows_core::BaseImpl + IUpdate_Impl {
    fn RebootRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPresent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CveIDs(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn CopyToCache(this: &Self::This, pfiles: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdate2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdate);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdate2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPresent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPresent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CveIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CveIDs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyToCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiles: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyToCache(this, ::windows_core::from_raw_borrowed(&pfiles)).into())
        }
        IUpdate2_Vtbl {
            base__: <IUpdate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            IsPresent: IsPresent::<Identity, Impl, OFFSET>,
            CveIDs: CveIDs::<Identity, Impl, OFFSET>,
            CopyToCache: CopyToCache::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate3_Impl: ::windows_core::BaseImpl + IUpdate2_Impl {
    fn BrowseOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdate3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdate2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdate3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BrowseOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BrowseOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdate3_Vtbl { base__: <IUpdate2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BrowseOnly: BrowseOnly::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate4_Impl: ::windows_core::BaseImpl + IUpdate3_Impl {
    fn PerUser(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdate4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdate3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdate4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PerUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PerUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdate4_Vtbl { base__: <IUpdate3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, PerUser: PerUser::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate5_Impl: ::windows_core::BaseImpl + IUpdate4_Impl {
    fn AutoSelection(this: &Self::This) -> ::windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(this: &Self::This) -> ::windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdate5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdate4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdate5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AutoSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AutoDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdate5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoDownload(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdate5_Vtbl {
            base__: <IUpdate4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AutoSelection: AutoSelection::<Identity, Impl, OFFSET>,
            AutoDownload: AutoDownload::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IUpdate>;
    fn put_Item(this: &Self::This, index: i32, value: ::core::option::Option<&IUpdate>) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(this: &Self::This, value: ::core::option::Option<&IUpdate>) -> ::windows_core::Result<i32>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn Insert(this: &Self::This, index: i32, value: ::core::option::Option<&IUpdate>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Item(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Copy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        IUpdateCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            put_Item: put_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadContent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateDownloadContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateDownloadContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateDownloadContent_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadContent2_Impl: ::windows_core::BaseImpl + IUpdateDownloadContent_Impl {
    fn IsDeltaCompressedContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateDownloadContent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateDownloadContent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateDownloadContent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDeltaCompressedContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDeltaCompressedContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateDownloadContent2_Vtbl {
            base__: <IUpdateDownloadContent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDeltaCompressedContent: IsDeltaCompressedContent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadContentCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IUpdateDownloadContent>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateDownloadContentCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateDownloadContentCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateDownloadContentCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadResult_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn HResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ResultCode(this: &Self::This) -> ::windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateDownloadResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateDownloadResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloadResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateDownloadResult_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HResult: HResult::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloader_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ClientApplicationID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsForced(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsForced(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<DownloadPriority>;
    fn SetPriority(this: &Self::This, value: DownloadPriority) -> ::windows_core::Result<()>;
    fn Updates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn SetUpdates(this: &Self::This, value: ::core::option::Option<&IUpdateCollection>) -> ::windows_core::Result<()>;
    fn BeginDownload(this: &Self::This, onprogresschanged: ::core::option::Option<&::windows_core::IUnknown>, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<IDownloadJob>;
    fn Download(this: &Self::This) -> ::windows_core::Result<IDownloadResult>;
    fn EndDownload(this: &Self::This, value: ::core::option::Option<&IDownloadJob>) -> ::windows_core::Result<IDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateDownloader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateDownloader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientApplicationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientApplicationID(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn IsForced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsForced(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsForced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsForced(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: DownloadPriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Updates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpdates(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginDownload(this, ::windows_core::from_raw_borrowed(&onprogresschanged), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Download(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndDownload(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateDownloader_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            IsForced: IsForced::<Identity, Impl, OFFSET>,
            SetIsForced: SetIsForced::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            SetUpdates: SetUpdates::<Identity, Impl, OFFSET>,
            BeginDownload: BeginDownload::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            EndDownload: EndDownload::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateException_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Message(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Context(this: &Self::This) -> ::windows_core::Result<UpdateExceptionContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateExceptionContext) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Context(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateException_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Message: Message::<Identity, Impl, OFFSET>,
            HResult: HResult::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateExceptionCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IUpdateException>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateExceptionCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateExceptionCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateExceptionCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateHistoryEntry_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Operation(this: &Self::This) -> ::windows_core::Result<UpdateOperation>;
    fn ResultCode(this: &Self::This) -> ::windows_core::Result<OperationResultCode>;
    fn HResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Date(this: &Self::This) -> ::windows_core::Result<f64>;
    fn UpdateIdentity(this: &Self::This) -> ::windows_core::Result<IUpdateIdentity>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UnmappedResultCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ClientApplicationID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServerSelection(this: &Self::This) -> ::windows_core::Result<ServerSelection>;
    fn ServiceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UninstallationSteps(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn UninstallationNotes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SupportUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateHistoryEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateHistoryEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Operation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateOperation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Date<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Date(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdateIdentity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnmappedResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnmappedResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientApplicationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallationSteps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UninstallationSteps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallationNotes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UninstallationNotes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateHistoryEntry_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Operation: Operation::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            HResult: HResult::<Identity, Impl, OFFSET>,
            Date: Date::<Identity, Impl, OFFSET>,
            UpdateIdentity: UpdateIdentity::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            UnmappedResultCode: UnmappedResultCode::<Identity, Impl, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            ServerSelection: ServerSelection::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, Impl, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, Impl, OFFSET>,
            SupportUrl: SupportUrl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateHistoryEntry2_Impl: ::windows_core::BaseImpl + IUpdateHistoryEntry_Impl {
    fn Categories(this: &Self::This) -> ::windows_core::Result<ICategoryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateHistoryEntry2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateHistoryEntry);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateHistoryEntry2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Categories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntry2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Categories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateHistoryEntry2_Vtbl { base__: <IUpdateHistoryEntry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Categories: Categories::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateHistoryEntryCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IUpdateHistoryEntry>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateHistoryEntryCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateHistoryEntryCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateHistoryEntryCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateIdentity_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn RevisionNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UpdateID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateIdentity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateIdentity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateIdentity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RevisionNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RevisionNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdateID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateIdentity_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RevisionNumber: RevisionNumber::<Identity, Impl, OFFSET>,
            UpdateID: UpdateID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstallationResult_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn HResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RebootRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ResultCode(this: &Self::This) -> ::windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateInstallationResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateInstallationResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateInstallationResult_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HResult: HResult::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ClientApplicationID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsForced(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsForced(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ParentHwnd(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn SetParentHwnd(this: &Self::This, value: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn SetParentWindow(this: &Self::This, value: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ParentWindow(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Updates(this: &Self::This) -> ::windows_core::Result<IUpdateCollection>;
    fn SetUpdates(this: &Self::This, value: ::core::option::Option<&IUpdateCollection>) -> ::windows_core::Result<()>;
    fn BeginInstall(this: &Self::This, onprogresschanged: ::core::option::Option<&::windows_core::IUnknown>, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<IInstallationJob>;
    fn BeginUninstall(this: &Self::This, onprogresschanged: ::core::option::Option<&::windows_core::IUnknown>, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<IInstallationJob>;
    fn EndInstall(this: &Self::This, value: ::core::option::Option<&IInstallationJob>) -> ::windows_core::Result<IInstallationResult>;
    fn EndUninstall(this: &Self::This, value: ::core::option::Option<&IInstallationJob>) -> ::windows_core::Result<IInstallationResult>;
    fn Install(this: &Self::This) -> ::windows_core::Result<IInstallationResult>;
    fn RunWizard(this: &Self::This, dialogtitle: &::windows_core::BSTR) -> ::windows_core::Result<IInstallationResult>;
    fn IsBusy(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Uninstall(this: &Self::This) -> ::windows_core::Result<IInstallationResult>;
    fn AllowSourcePrompts(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowSourcePrompts(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RebootRequiredBeforeInstallation(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateInstaller {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateInstaller {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientApplicationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientApplicationID(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn IsForced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsForced(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsForced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsForced(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ParentHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParentHwnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParentHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParentHwnd(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetParentWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParentWindow(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn ParentWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParentWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Updates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpdates(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn BeginInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginInstall(this, ::windows_core::from_raw_borrowed(&onprogresschanged), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginUninstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginUninstall(this, ::windows_core::from_raw_borrowed(&onprogresschanged), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndInstall(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndUninstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndUninstall(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Install(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RunWizard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dialogtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunWizard(this, ::core::mem::transmute(&dialogtitle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsBusy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBusy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Uninstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Uninstall(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllowSourcePrompts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowSourcePrompts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowSourcePrompts(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootRequiredBeforeInstallation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateInstaller_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            IsForced: IsForced::<Identity, Impl, OFFSET>,
            SetIsForced: SetIsForced::<Identity, Impl, OFFSET>,
            ParentHwnd: ParentHwnd::<Identity, Impl, OFFSET>,
            SetParentHwnd: SetParentHwnd::<Identity, Impl, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, Impl, OFFSET>,
            ParentWindow: ParentWindow::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            SetUpdates: SetUpdates::<Identity, Impl, OFFSET>,
            BeginInstall: BeginInstall::<Identity, Impl, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, Impl, OFFSET>,
            EndInstall: EndInstall::<Identity, Impl, OFFSET>,
            EndUninstall: EndUninstall::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            RunWizard: RunWizard::<Identity, Impl, OFFSET>,
            IsBusy: IsBusy::<Identity, Impl, OFFSET>,
            Uninstall: Uninstall::<Identity, Impl, OFFSET>,
            AllowSourcePrompts: AllowSourcePrompts::<Identity, Impl, OFFSET>,
            SetAllowSourcePrompts: SetAllowSourcePrompts::<Identity, Impl, OFFSET>,
            RebootRequiredBeforeInstallation: RebootRequiredBeforeInstallation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller2_Impl: ::windows_core::BaseImpl + IUpdateInstaller_Impl {
    fn ForceQuiet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetForceQuiet(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateInstaller2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateInstaller);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateInstaller2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ForceQuiet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForceQuiet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForceQuiet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForceQuiet(this, ::core::mem::transmute_copy(&value)).into())
        }
        IUpdateInstaller2_Vtbl {
            base__: <IUpdateInstaller as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ForceQuiet: ForceQuiet::<Identity, Impl, OFFSET>,
            SetForceQuiet: SetForceQuiet::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller3_Impl: ::windows_core::BaseImpl + IUpdateInstaller2_Impl {
    fn AttemptCloseAppsIfNecessary(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttemptCloseAppsIfNecessary(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateInstaller3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateInstaller2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateInstaller3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AttemptCloseAppsIfNecessary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttemptCloseAppsIfNecessary(this, ::core::mem::transmute_copy(&value)).into())
        }
        IUpdateInstaller3_Vtbl {
            base__: <IUpdateInstaller2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AttemptCloseAppsIfNecessary: AttemptCloseAppsIfNecessary::<Identity, Impl, OFFSET>,
            SetAttemptCloseAppsIfNecessary: SetAttemptCloseAppsIfNecessary::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller4_Impl: ::windows_core::BaseImpl + IUpdateInstaller3_Impl {
    fn Commit(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateInstaller4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateInstaller3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateInstaller4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateInstaller4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        IUpdateInstaller4_Vtbl { base__: <IUpdateInstaller3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Commit: Commit::<Identity, Impl, OFFSET> }
    };
}
pub trait IUpdateLockdown_Impl: ::windows_core::BaseImpl {
    fn LockDown(this: &Self::This, flags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUpdateLockdown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateLockdown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateLockdown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LockDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateLockdown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockDown(this, ::core::mem::transmute_copy(&flags)).into())
        }
        IUpdateLockdown_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LockDown: LockDown::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSearcher_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CanAutomaticallyUpgradeService(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCanAutomaticallyUpgradeService(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ClientApplicationID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IncludePotentiallySupersededUpdates(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncludePotentiallySupersededUpdates(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ServerSelection(this: &Self::This) -> ::windows_core::Result<ServerSelection>;
    fn SetServerSelection(this: &Self::This, value: ServerSelection) -> ::windows_core::Result<()>;
    fn BeginSearch(this: &Self::This, criteria: &::windows_core::BSTR, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<ISearchJob>;
    fn EndSearch(this: &Self::This, searchjob: ::core::option::Option<&ISearchJob>) -> ::windows_core::Result<ISearchResult>;
    fn EscapeString(this: &Self::This, unescaped: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn QueryHistory(this: &Self::This, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection>;
    fn Search(this: &Self::This, criteria: &::windows_core::BSTR) -> ::windows_core::Result<ISearchResult>;
    fn Online(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOnline(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetTotalHistoryCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ServiceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceID(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateSearcher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateSearcher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanAutomaticallyUpgradeService(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCanAutomaticallyUpgradeService(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientApplicationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientApplicationID(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncludePotentiallySupersededUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIncludePotentiallySupersededUpdates(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ServerSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ServerSelection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerSelection(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn BeginSearch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginSearch(this, ::core::mem::transmute(&criteria), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndSearch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchjob: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndSearch(this, ::windows_core::from_raw_borrowed(&searchjob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EscapeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unescaped: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EscapeString(this, ::core::mem::transmute(&unescaped)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: i32, count: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHistory(this, ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Search<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Search(this, ::core::mem::transmute(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Online<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Online(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOnline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOnline(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetTotalHistoryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTotalHistoryCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceID(this, ::core::mem::transmute(&value)).into())
        }
        IUpdateSearcher_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanAutomaticallyUpgradeService: CanAutomaticallyUpgradeService::<Identity, Impl, OFFSET>,
            SetCanAutomaticallyUpgradeService: SetCanAutomaticallyUpgradeService::<Identity, Impl, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            IncludePotentiallySupersededUpdates: IncludePotentiallySupersededUpdates::<Identity, Impl, OFFSET>,
            SetIncludePotentiallySupersededUpdates: SetIncludePotentiallySupersededUpdates::<Identity, Impl, OFFSET>,
            ServerSelection: ServerSelection::<Identity, Impl, OFFSET>,
            SetServerSelection: SetServerSelection::<Identity, Impl, OFFSET>,
            BeginSearch: BeginSearch::<Identity, Impl, OFFSET>,
            EndSearch: EndSearch::<Identity, Impl, OFFSET>,
            EscapeString: EscapeString::<Identity, Impl, OFFSET>,
            QueryHistory: QueryHistory::<Identity, Impl, OFFSET>,
            Search: Search::<Identity, Impl, OFFSET>,
            Online: Online::<Identity, Impl, OFFSET>,
            SetOnline: SetOnline::<Identity, Impl, OFFSET>,
            GetTotalHistoryCount: GetTotalHistoryCount::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            SetServiceID: SetServiceID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSearcher2_Impl: ::windows_core::BaseImpl + IUpdateSearcher_Impl {
    fn IgnoreDownloadPriority(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIgnoreDownloadPriority(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateSearcher2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateSearcher);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateSearcher2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IgnoreDownloadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IgnoreDownloadPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIgnoreDownloadPriority(this, ::core::mem::transmute_copy(&value)).into())
        }
        IUpdateSearcher2_Vtbl {
            base__: <IUpdateSearcher as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IgnoreDownloadPriority: IgnoreDownloadPriority::<Identity, Impl, OFFSET>,
            SetIgnoreDownloadPriority: SetIgnoreDownloadPriority::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSearcher3_Impl: ::windows_core::BaseImpl + IUpdateSearcher2_Impl {
    fn SearchScope(this: &Self::This) -> ::windows_core::Result<SearchScope>;
    fn SetSearchScope(this: &Self::This, value: SearchScope) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateSearcher3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateSearcher2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateSearcher3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SearchScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut SearchScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchScope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSearchScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSearcher3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SearchScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSearchScope(this, ::core::mem::transmute_copy(&value)).into())
        }
        IUpdateSearcher3_Vtbl {
            base__: <IUpdateSearcher2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SearchScope: SearchScope::<Identity, Impl, OFFSET>,
            SetSearchScope: SetSearchScope::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateService_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ContentValidationCert(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ExpirationDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn IsManaged(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsRegisteredWithAU(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IssueDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn OffersWindowsUpdates(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RedirectUrls(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn ServiceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsScanPackageService(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CanRegisterWithAU(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ServiceUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetupPrefix(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentValidationCert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentValidationCert(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpirationDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsManaged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsManaged(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRegisteredWithAU<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRegisteredWithAU(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IssueDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IssueDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OffersWindowsUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OffersWindowsUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectUrls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectUrls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsScanPackageService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsScanPackageService(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanRegisterWithAU<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanRegisterWithAU(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetupPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetupPrefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateService_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            ContentValidationCert: ContentValidationCert::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            IsManaged: IsManaged::<Identity, Impl, OFFSET>,
            IsRegisteredWithAU: IsRegisteredWithAU::<Identity, Impl, OFFSET>,
            IssueDate: IssueDate::<Identity, Impl, OFFSET>,
            OffersWindowsUpdates: OffersWindowsUpdates::<Identity, Impl, OFFSET>,
            RedirectUrls: RedirectUrls::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            IsScanPackageService: IsScanPackageService::<Identity, Impl, OFFSET>,
            CanRegisterWithAU: CanRegisterWithAU::<Identity, Impl, OFFSET>,
            ServiceUrl: ServiceUrl::<Identity, Impl, OFFSET>,
            SetupPrefix: SetupPrefix::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateService2_Impl: ::windows_core::BaseImpl + IUpdateService_Impl {
    fn IsDefaultAUService(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateService2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateService);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateService2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDefaultAUService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateService2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDefaultAUService(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateService2_Vtbl { base__: <IUpdateService as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsDefaultAUService: IsDefaultAUService::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IUpdateService>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateServiceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateServiceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateServiceCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Services(this: &Self::This) -> ::windows_core::Result<IUpdateServiceCollection>;
    fn AddService(this: &Self::This, serviceid: &::windows_core::BSTR, authorizationcabpath: &::windows_core::BSTR) -> ::windows_core::Result<IUpdateService>;
    fn RegisterServiceWithAU(this: &Self::This, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveService(this: &Self::This, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UnregisterServiceWithAU(this: &Self::This, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddScanPackageService(this: &Self::This, servicename: &::windows_core::BSTR, scanfilelocation: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<IUpdateService>;
    fn SetOption(this: &Self::This, optionname: &::windows_core::BSTR, optionvalue: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateServiceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateServiceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Services<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Services(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, authorizationcabpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddService(this, ::core::mem::transmute(&serviceid), ::core::mem::transmute(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterServiceWithAU<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterServiceWithAU(this, ::core::mem::transmute(&serviceid)).into())
        }
        unsafe extern "system" fn RemoveService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveService(this, ::core::mem::transmute(&serviceid)).into())
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterServiceWithAU(this, ::core::mem::transmute(&serviceid)).into())
        }
        unsafe extern "system" fn AddScanPackageService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, scanfilelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddScanPackageService(this, ::core::mem::transmute(&servicename), ::core::mem::transmute(&scanfilelocation), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionvalue: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOption(this, ::core::mem::transmute(&optionname), ::core::mem::transmute(&optionvalue)).into())
        }
        IUpdateServiceManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Services: Services::<Identity, Impl, OFFSET>,
            AddService: AddService::<Identity, Impl, OFFSET>,
            RegisterServiceWithAU: RegisterServiceWithAU::<Identity, Impl, OFFSET>,
            RemoveService: RemoveService::<Identity, Impl, OFFSET>,
            UnregisterServiceWithAU: UnregisterServiceWithAU::<Identity, Impl, OFFSET>,
            AddScanPackageService: AddScanPackageService::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceManager2_Impl: ::windows_core::BaseImpl + IUpdateServiceManager_Impl {
    fn ClientApplicationID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryServiceRegistration(this: &Self::This, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<IUpdateServiceRegistration>;
    fn AddService2(this: &Self::This, serviceid: &::windows_core::BSTR, flags: i32, authorizationcabpath: &::windows_core::BSTR) -> ::windows_core::Result<IUpdateServiceRegistration>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateServiceManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateServiceManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateServiceManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientApplicationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientApplicationID(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn QueryServiceRegistration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryServiceRegistration(this, ::core::mem::transmute(&serviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddService2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, authorizationcabpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddService2(this, ::core::mem::transmute(&serviceid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateServiceManager2_Vtbl {
            base__: <IUpdateServiceManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Identity, Impl, OFFSET>,
            AddService2: AddService2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceRegistration_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn RegistrationState(this: &Self::This) -> ::windows_core::Result<UpdateServiceRegistrationState>;
    fn ServiceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsPendingRegistrationWithAU(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Service(this: &Self::This) -> ::windows_core::Result<IUpdateService2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateServiceRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateServiceRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegistrationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegistrationState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPendingRegistrationWithAU(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Service<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Service(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateServiceRegistration_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegistrationState: RegistrationState::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            IsPendingRegistrationWithAU: IsPendingRegistrationWithAU::<Identity, Impl, OFFSET>,
            Service: Service::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSession_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ClientApplicationID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WebProxy(this: &Self::This) -> ::windows_core::Result<IWebProxy>;
    fn SetWebProxy(this: &Self::This, value: ::core::option::Option<&IWebProxy>) -> ::windows_core::Result<()>;
    fn CreateUpdateSearcher(this: &Self::This) -> ::windows_core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(this: &Self::This) -> ::windows_core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(this: &Self::This) -> ::windows_core::Result<IUpdateInstaller>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientApplicationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientApplicationID(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WebProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WebProxy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWebProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWebProxy(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn CreateUpdateSearcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUpdateSearcher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateUpdateDownloader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUpdateDownloader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateUpdateInstaller<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUpdateInstaller(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateSession_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            WebProxy: WebProxy::<Identity, Impl, OFFSET>,
            SetWebProxy: SetWebProxy::<Identity, Impl, OFFSET>,
            CreateUpdateSearcher: CreateUpdateSearcher::<Identity, Impl, OFFSET>,
            CreateUpdateDownloader: CreateUpdateDownloader::<Identity, Impl, OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSession2_Impl: ::windows_core::BaseImpl + IUpdateSession_Impl {
    fn UserLocale(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetUserLocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateSession2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateSession);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateSession2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserLocale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUserLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserLocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        IUpdateSession2_Vtbl {
            base__: <IUpdateSession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserLocale: UserLocale::<Identity, Impl, OFFSET>,
            SetUserLocale: SetUserLocale::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSession3_Impl: ::windows_core::BaseImpl + IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(this: &Self::This) -> ::windows_core::Result<IUpdateServiceManager2>;
    fn QueryHistory(this: &Self::This, criteria: &::windows_core::BSTR, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUpdateSession3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdateSession2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUpdateSession3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateUpdateServiceManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUpdateServiceManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUpdateSession3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, startindex: i32, count: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHistory(this, ::core::mem::transmute(&criteria), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUpdateSession3_Vtbl {
            base__: <IUpdateSession2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateUpdateServiceManager: CreateUpdateServiceManager::<Identity, Impl, OFFSET>,
            QueryHistory: QueryHistory::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWebProxy_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Address(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAddress(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BypassList(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn SetBypassList(this: &Self::This, value: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
    fn BypassProxyOnLocal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBypassProxyOnLocal(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetUserName(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetPassword(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PromptForCredentials(this: &Self::This, parentwindow: ::core::option::Option<&::windows_core::IUnknown>, title: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PromptForCredentialsFromHwnd(this: &Self::This, parentwindow: super::super::Foundation::HWND, title: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AutoDetect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoDetect(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWebProxy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebProxy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddress(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn BypassList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BypassList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBypassList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBypassList(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn BypassProxyOnLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BypassProxyOnLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBypassProxyOnLocal(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn PromptForCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentwindow: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PromptForCredentials(this, ::windows_core::from_raw_borrowed(&parentwindow), ::core::mem::transmute(&title)).into())
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PromptForCredentialsFromHwnd(this, ::core::mem::transmute_copy(&parentwindow), ::core::mem::transmute(&title)).into())
        }
        unsafe extern "system" fn AutoDetect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoDetect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoDetect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoDetect(this, ::core::mem::transmute_copy(&value)).into())
        }
        IWebProxy_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
            BypassList: BypassList::<Identity, Impl, OFFSET>,
            SetBypassList: SetBypassList::<Identity, Impl, OFFSET>,
            BypassProxyOnLocal: BypassProxyOnLocal::<Identity, Impl, OFFSET>,
            SetBypassProxyOnLocal: SetBypassProxyOnLocal::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            UserName: UserName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            PromptForCredentials: PromptForCredentials::<Identity, Impl, OFFSET>,
            PromptForCredentialsFromHwnd: PromptForCredentialsFromHwnd::<Identity, Impl, OFFSET>,
            AutoDetect: AutoDetect::<Identity, Impl, OFFSET>,
            SetAutoDetect: SetAutoDetect::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate_Impl: ::windows_core::BaseImpl + IUpdate_Impl {
    fn DriverClass(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverHardwareID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverManufacturer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverModel(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverProvider(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverVerDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn DeviceProblemNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceStatus(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUpdate);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DriverClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverHardwareID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverHardwareID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverManufacturer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverModel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverVerDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverVerDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceProblemNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDriverUpdate_Vtbl {
            base__: <IUpdate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DriverClass: DriverClass::<Identity, Impl, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, Impl, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, Impl, OFFSET>,
            DriverModel: DriverModel::<Identity, Impl, OFFSET>,
            DriverProvider: DriverProvider::<Identity, Impl, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, Impl, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, Impl, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate2_Impl: ::windows_core::BaseImpl + IWindowsDriverUpdate_Impl {
    fn RebootRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPresent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CveIDs(this: &Self::This) -> ::windows_core::Result<IStringCollection>;
    fn CopyToCache(this: &Self::This, pfiles: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdate2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowsDriverUpdate);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdate2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RebootRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPresent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPresent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CveIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CveIDs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyToCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiles: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyToCache(this, ::windows_core::from_raw_borrowed(&pfiles)).into())
        }
        IWindowsDriverUpdate2_Vtbl {
            base__: <IWindowsDriverUpdate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            IsPresent: IsPresent::<Identity, Impl, OFFSET>,
            CveIDs: CveIDs::<Identity, Impl, OFFSET>,
            CopyToCache: CopyToCache::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate3_Impl: ::windows_core::BaseImpl + IWindowsDriverUpdate2_Impl {
    fn BrowseOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdate3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowsDriverUpdate2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdate3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BrowseOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BrowseOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDriverUpdate3_Vtbl { base__: <IWindowsDriverUpdate2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BrowseOnly: BrowseOnly::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate4_Impl: ::windows_core::BaseImpl + IWindowsDriverUpdate3_Impl {
    fn WindowsDriverUpdateEntries(this: &Self::This) -> ::windows_core::Result<IWindowsDriverUpdateEntryCollection>;
    fn PerUser(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdate4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowsDriverUpdate3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdate4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowsDriverUpdateEntries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PerUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PerUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDriverUpdate4_Vtbl {
            base__: <IWindowsDriverUpdate3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WindowsDriverUpdateEntries: WindowsDriverUpdateEntries::<Identity, Impl, OFFSET>,
            PerUser: PerUser::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate5_Impl: ::windows_core::BaseImpl + IWindowsDriverUpdate4_Impl {
    fn AutoSelection(this: &Self::This) -> ::windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(this: &Self::This) -> ::windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdate5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowsDriverUpdate4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdate5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AutoSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AutoDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoDownload(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDriverUpdate5_Vtbl {
            base__: <IWindowsDriverUpdate4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AutoSelection: AutoSelection::<Identity, Impl, OFFSET>,
            AutoDownload: AutoDownload::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdateEntry_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DriverClass(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverHardwareID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverManufacturer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverModel(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverProvider(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverVerDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn DeviceProblemNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceStatus(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdateEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdateEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DriverClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverHardwareID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverHardwareID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverManufacturer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverModel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverVerDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverVerDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceProblemNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDriverUpdateEntry_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DriverClass: DriverClass::<Identity, Impl, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, Impl, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, Impl, OFFSET>,
            DriverModel: DriverModel::<Identity, Impl, OFFSET>,
            DriverProvider: DriverProvider::<Identity, Impl, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, Impl, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, Impl, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdateEntryCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IWindowsDriverUpdateEntry>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsDriverUpdateEntryCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDriverUpdateEntryCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDriverUpdateEntryCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsUpdateAgentInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetInfo(this: &Self::This, varinfoidentifier: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsUpdateAgentInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsUpdateAgentInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinfoidentifier: super::Variant::VARIANT, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInfo(this, ::core::mem::transmute(&varinfoidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsUpdateAgentInfo_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetInfo: GetInfo::<Identity, Impl, OFFSET> }
    };
}
