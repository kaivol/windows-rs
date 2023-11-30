#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICameraUIControl_Impl: ::windows_core::BaseImpl {
    fn Show(this: &Self::This, pwindow: ::core::option::Option<&::windows_core::IUnknown>, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: ::core::option::Option<&ICameraUIControlEventCallback>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Suspend(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCurrentViewType(this: &Self::This) -> ::windows_core::Result<CameraUIControlViewType>;
    fn GetActiveItem(this: &Self::This, pbstractiveitempath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetSelectedItems(this: &Self::This) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn RemoveCapturedItem(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ICameraUIControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICameraUIControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::windows_core::from_raw_borrowed(&pwindow), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&selectionmode), ::core::mem::transmute_copy(&capturemode), ::core::mem::transmute_copy(&photoformat), ::core::mem::transmute_copy(&videoformat), ::core::mem::transmute_copy(&bhasclosebutton), ::windows_core::from_raw_borrowed(&peventcallback)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Suspend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Suspend(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdeferralrequired, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn GetCurrentViewType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentViewType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pviewtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstractiveitempath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActiveItem(this, ::core::mem::transmute_copy(&pbstractiveitempath)).into())
        }
        unsafe extern "system" fn GetSelectedItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelectedItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppselecteditempaths, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveCapturedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveCapturedItem(this, ::core::mem::transmute(&pszpath)).into())
        }
        ICameraUIControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Show: Show::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            GetCurrentViewType: GetCurrentViewType::<Identity, Impl, OFFSET>,
            GetActiveItem: GetActiveItem::<Identity, Impl, OFFSET>,
            GetSelectedItems: GetSelectedItems::<Identity, Impl, OFFSET>,
            RemoveCapturedItem: RemoveCapturedItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICameraUIControlEventCallback_Impl: ::windows_core::BaseImpl {
    fn OnStartupComplete(this: &Self::This);
    fn OnSuspendComplete(this: &Self::This);
    fn OnItemCaptured(this: &Self::This, pszpath: &::windows_core::PCWSTR);
    fn OnItemDeleted(this: &Self::This, pszpath: &::windows_core::PCWSTR);
    fn OnClosed(this: &Self::This);
}
impl ::windows_core::Iids for ICameraUIControlEventCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICameraUIControlEventCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStartupComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStartupComplete(this))
        }
        unsafe extern "system" fn OnSuspendComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSuspendComplete(this))
        }
        unsafe extern "system" fn OnItemCaptured<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnItemCaptured(this, ::core::mem::transmute(&pszpath)))
        }
        unsafe extern "system" fn OnItemDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnItemDeleted(this, ::core::mem::transmute(&pszpath)))
        }
        unsafe extern "system" fn OnClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClosed(this))
        }
        ICameraUIControlEventCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStartupComplete: OnStartupComplete::<Identity, Impl, OFFSET>,
            OnSuspendComplete: OnSuspendComplete::<Identity, Impl, OFFSET>,
            OnItemCaptured: OnItemCaptured::<Identity, Impl, OFFSET>,
            OnItemDeleted: OnItemDeleted::<Identity, Impl, OFFSET>,
            OnClosed: OnClosed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IClipServiceNotificationHelper_Impl: ::windows_core::BaseImpl {
    fn ShowToast(this: &Self::This, titletext: &::windows_core::BSTR, bodytext: &::windows_core::BSTR, packagename: &::windows_core::BSTR, appid: &::windows_core::BSTR, launchcommand: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IClipServiceNotificationHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClipServiceNotificationHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClipServiceNotificationHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowToast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClipServiceNotificationHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, titletext: ::std::mem::MaybeUninit<::windows_core::BSTR>, bodytext: ::std::mem::MaybeUninit<::windows_core::BSTR>, packagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, appid: ::std::mem::MaybeUninit<::windows_core::BSTR>, launchcommand: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowToast(this, ::core::mem::transmute(&titletext), ::core::mem::transmute(&bodytext), ::core::mem::transmute(&packagename), ::core::mem::transmute(&appid), ::core::mem::transmute(&launchcommand)).into())
        }
        IClipServiceNotificationHelper_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ShowToast: ShowToast::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContainerActivationHelper_Impl: ::windows_core::BaseImpl {
    fn CanActivateClientVM(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContainerActivationHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContainerActivationHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContainerActivationHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanActivateClientVM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContainerActivationHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanActivateClientVM(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContainerActivationHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanActivateClientVM: CanActivateClientVM::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDefaultBrowserSyncSettings_Impl: ::windows_core::BaseImpl {
    fn IsEnabled(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDefaultBrowserSyncSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultBrowserSyncSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDefaultBrowserSyncSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultBrowserSyncSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEnabled(this))
        }
        IDefaultBrowserSyncSettings_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsEnabled: IsEnabled::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDeleteBrowsingHistory_Impl: ::windows_core::BaseImpl {
    fn DeleteBrowsingHistory(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDeleteBrowsingHistory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeleteBrowsingHistory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeleteBrowsingHistory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteBrowsingHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeleteBrowsingHistory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteBrowsingHistory(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        IDeleteBrowsingHistory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteBrowsingHistory: DeleteBrowsingHistory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IEditionUpgradeBroker_Impl: ::windows_core::BaseImpl {
    fn InitializeParentWindow(this: &Self::This, parenthandle: super::Ole::OLE_HANDLE) -> ::windows_core::Result<()>;
    fn UpdateOperatingSystem(this: &Self::This, parameter: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ShowProductKeyUI(this: &Self::This) -> ::windows_core::Result<()>;
    fn CanUpgrade(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for IEditionUpgradeBroker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeBroker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEditionUpgradeBroker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeParentWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeBroker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parenthandle: super::Ole::OLE_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeParentWindow(this, ::core::mem::transmute_copy(&parenthandle)).into())
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeBroker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameter: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOperatingSystem(this, ::core::mem::transmute(&parameter)).into())
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeBroker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowProductKeyUI(this).into())
        }
        unsafe extern "system" fn CanUpgrade<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeBroker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanUpgrade(this).into())
        }
        IEditionUpgradeBroker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeParentWindow: InitializeParentWindow::<Identity, Impl, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, Impl, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, Impl, OFFSET>,
            CanUpgrade: CanUpgrade::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEditionUpgradeHelper_Impl: ::windows_core::BaseImpl {
    fn CanUpgrade(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn UpdateOperatingSystem(this: &Self::This, contentid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ShowProductKeyUI(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetOsProductContentId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetGenuineLocalStatus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEditionUpgradeHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEditionUpgradeHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanUpgrade<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanUpgrade(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOperatingSystem(this, ::core::mem::transmute(&contentid)).into())
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowProductKeyUI(this).into())
        }
        unsafe extern "system" fn GetOsProductContentId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOsProductContentId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGenuineLocalStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEditionUpgradeHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGenuineLocalStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isgenuine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEditionUpgradeHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanUpgrade: CanUpgrade::<Identity, Impl, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, Impl, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, Impl, OFFSET>,
            GetOsProductContentId: GetOsProductContentId::<Identity, Impl, OFFSET>,
            GetGenuineLocalStatus: GetGenuineLocalStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFClipNotificationHelper_Impl: ::windows_core::BaseImpl {
    fn ShowSystemDialog(this: &Self::This, titletext: &::windows_core::BSTR, bodytext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFClipNotificationHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFClipNotificationHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFClipNotificationHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowSystemDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFClipNotificationHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, titletext: ::std::mem::MaybeUninit<::windows_core::BSTR>, bodytext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowSystemDialog(this, ::core::mem::transmute(&titletext), ::core::mem::transmute(&bodytext)).into())
        }
        IFClipNotificationHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ShowSystemDialog: ShowSystemDialog::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsLockModeHelper_Impl: ::windows_core::BaseImpl {
    fn GetSMode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWindowsLockModeHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsLockModeHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsLockModeHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsLockModeHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsLockModeHelper_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSMode: GetSMode::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
