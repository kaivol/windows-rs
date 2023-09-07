#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPhotoAcquire_Impl: ::windows_core::BaseImpl {
    fn CreatePhotoSource(this: &Self::This, pszdevice: &::windows_core::PCWSTR) -> ::windows_core::Result<IPhotoAcquireSource>;
    fn Acquire(this: &Self::This, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: &::windows_core::PCWSTR, pphotoacquireprogresscb: ::core::option::Option<&IPhotoAcquireProgressCB>) -> ::windows_core::Result<()>;
    fn EnumResults(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPhotoAcquire {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquire {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePhotoSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdevice: ::windows_core::PCWSTR, ppphotoacquiresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePhotoSource(this, ::core::mem::transmute(&pszdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquiresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: ::windows_core::PCWSTR, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this, ::windows_core::from_raw_borrowed(&pphotoacquiresource), ::core::mem::transmute_copy(&fshowprogress), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&pszapplicationname), ::windows_core::from_raw_borrowed(&pphotoacquireprogresscb)).into())
        }
        unsafe extern "system" fn EnumResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumfilepaths, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPhotoAcquire_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePhotoSource: CreatePhotoSource::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            EnumResults: EnumResults::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireDeviceSelectionDialog_Impl: ::windows_core::BaseImpl {
    fn SetTitle(this: &Self::This, psztitle: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetSubmitButtonText(this: &Self::This, pszsubmitbuttontext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DoModal(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut ::windows_core::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPhotoAcquireDeviceSelectionDialog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquireDeviceSelectionDialog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&psztitle)).into())
        }
        unsafe extern "system" fn SetSubmitButtonText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubmitButtonText(this, ::core::mem::transmute(&pszsubmitbuttontext)).into())
        }
        unsafe extern "system" fn DoModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoModal(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwdeviceflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&pndevicetype)).into())
        }
        IPhotoAcquireDeviceSelectionDialog_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            SetSubmitButtonText: SetSubmitButtonText::<Identity, Impl, OFFSET>,
            DoModal: DoModal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquireItem_Impl: ::windows_core::BaseImpl {
    fn GetItemName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetThumbnail(this: &Self::This, sizethumbnail: &super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
    fn GetProperty(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn CanDelete(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetSubItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSubItemAt(this: &Self::This, nitemindex: u32) -> ::windows_core::Result<IPhotoAcquireItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IPhotoAcquireItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquireItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstritemname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstritemname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnail(this, ::core::mem::transmute(&sizethumbnail)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phbmpthumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv)).into())
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanDelete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcandelete, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn GetSubItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubItemAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubItemAt(this, ::core::mem::transmute_copy(&nitemindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquireitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPhotoAcquireItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            CanDelete: CanDelete::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetSubItemCount: GetSubItemCount::<Identity, Impl, OFFSET>,
            GetSubItemAt: GetSubItemAt::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireOptionsDialog_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pszregistryroot: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Create(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn Destroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn DoModal(this: &Self::This, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows_core::Result<()>;
    fn SaveData(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPhotoAcquireOptionsDialog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquireOptionsDialog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszregistryroot: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pszregistryroot)).into())
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnddialog, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this).into())
        }
        unsafe extern "system" fn DoModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoModal(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppnreturncode)).into())
        }
        unsafe extern "system" fn SaveData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveData(this).into())
        }
        IPhotoAcquireOptionsDialog_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            DoModal: DoModal::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquirePlugin_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>, pphotoacquireprogresscb: ::core::option::Option<&IPhotoAcquireProgressCB>) -> ::windows_core::Result<()>;
    fn ProcessItem(this: &Self::This, dwacquirestage: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>, poriginalitemstream: ::core::option::Option<&super::super::System::Com::IStream>, pszfinalfilename: &::windows_core::PCWSTR, ppropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn TransferComplete(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn DisplayConfigureDialog(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IPhotoAcquirePlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquirePlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pphotoacquiresource), ::windows_core::from_raw_borrowed(&pphotoacquireprogresscb)).into())
        }
        unsafe extern "system" fn ProcessItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: *mut ::core::ffi::c_void, poriginalitemstream: *mut ::core::ffi::c_void, pszfinalfilename: ::windows_core::PCWSTR, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessItem(this, ::core::mem::transmute_copy(&dwacquirestage), ::windows_core::from_raw_borrowed(&pphotoacquireitem), ::windows_core::from_raw_borrowed(&poriginalitemstream), ::core::mem::transmute(&pszfinalfilename), ::windows_core::from_raw_borrowed(&ppropertystore)).into())
        }
        unsafe extern "system" fn TransferComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferComplete(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn DisplayConfigureDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayConfigureDialog(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        IPhotoAcquirePlugin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ProcessItem: ProcessItem::<Identity, Impl, OFFSET>,
            TransferComplete: TransferComplete::<Identity, Impl, OFFSET>,
            DisplayConfigureDialog: DisplayConfigureDialog::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPhotoAcquireProgressCB_Impl: ::windows_core::BaseImpl {
    fn Cancelled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn StartEnumeration(this: &Self::This, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>) -> ::windows_core::Result<()>;
    fn FoundItem(this: &Self::This, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>) -> ::windows_core::Result<()>;
    fn EndEnumeration(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn StartTransfer(this: &Self::This, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>) -> ::windows_core::Result<()>;
    fn StartItemTransfer(this: &Self::This, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>) -> ::windows_core::Result<()>;
    fn DirectoryCreated(this: &Self::This, pszdirectory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UpdateTransferPercent(this: &Self::This, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows_core::Result<()>;
    fn EndItemTransfer(this: &Self::This, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn EndTransfer(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn StartDelete(this: &Self::This, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>) -> ::windows_core::Result<()>;
    fn StartItemDelete(this: &Self::This, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>) -> ::windows_core::Result<()>;
    fn UpdateDeletePercent(this: &Self::This, npercent: u32) -> ::windows_core::Result<()>;
    fn EndItemDelete(this: &Self::This, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn EndDelete(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn EndSession(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetDeleteAfterAcquire(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ErrorAdvise(this: &Self::This, hr: ::windows_core::HRESULT, pszerrormessage: &::windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows_core::Result<ERROR_ADVISE_RESULT>;
    fn GetUserInput(this: &Self::This, riidtype: *const ::windows_core::GUID, punknown: ::core::option::Option<&::windows_core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPhotoAcquireProgressCB {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquireProgressCB {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancelled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cancelled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancelled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartEnumeration(this, ::windows_core::from_raw_borrowed(&pphotoacquiresource)).into())
        }
        unsafe extern "system" fn FoundItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FoundItem(this, ::windows_core::from_raw_borrowed(&pphotoacquireitem)).into())
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEnumeration(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn StartTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartTransfer(this, ::windows_core::from_raw_borrowed(&pphotoacquiresource)).into())
        }
        unsafe extern "system" fn StartItemTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartItemTransfer(this, ::core::mem::transmute_copy(&nitemindex), ::windows_core::from_raw_borrowed(&pphotoacquireitem)).into())
        }
        unsafe extern "system" fn DirectoryCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DirectoryCreated(this, ::core::mem::transmute(&pszdirectory)).into())
        }
        unsafe extern "system" fn UpdateTransferPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateTransferPercent(this, ::core::mem::transmute_copy(&foverall), ::core::mem::transmute_copy(&npercent)).into())
        }
        unsafe extern "system" fn EndItemTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndItemTransfer(this, ::core::mem::transmute_copy(&nitemindex), ::windows_core::from_raw_borrowed(&pphotoacquireitem), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn EndTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndTransfer(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn StartDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartDelete(this, ::windows_core::from_raw_borrowed(&pphotoacquiresource)).into())
        }
        unsafe extern "system" fn StartItemDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartItemDelete(this, ::core::mem::transmute_copy(&nitemindex), ::windows_core::from_raw_borrowed(&pphotoacquireitem)).into())
        }
        unsafe extern "system" fn UpdateDeletePercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDeletePercent(this, ::core::mem::transmute_copy(&npercent)).into())
        }
        unsafe extern "system" fn EndItemDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndItemDelete(this, ::core::mem::transmute_copy(&nitemindex), ::windows_core::from_raw_borrowed(&pphotoacquireitem), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn EndDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDelete(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn EndSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSession(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeleteAfterAcquire(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdeleteafteracquire, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ErrorAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, pszerrormessage: ::windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorAdvise(this, ::core::mem::transmute_copy(&hr), ::core::mem::transmute(&pszerrormessage), ::core::mem::transmute_copy(&nmessagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnerroradviseresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows_core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserInput(this, ::core::mem::transmute_copy(&riidtype), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&ppropvarresult), ::core::mem::transmute_copy(&ppropvardefault)).into())
        }
        IPhotoAcquireProgressCB_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancelled: Cancelled::<Identity, Impl, OFFSET>,
            StartEnumeration: StartEnumeration::<Identity, Impl, OFFSET>,
            FoundItem: FoundItem::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            StartTransfer: StartTransfer::<Identity, Impl, OFFSET>,
            StartItemTransfer: StartItemTransfer::<Identity, Impl, OFFSET>,
            DirectoryCreated: DirectoryCreated::<Identity, Impl, OFFSET>,
            UpdateTransferPercent: UpdateTransferPercent::<Identity, Impl, OFFSET>,
            EndItemTransfer: EndItemTransfer::<Identity, Impl, OFFSET>,
            EndTransfer: EndTransfer::<Identity, Impl, OFFSET>,
            StartDelete: StartDelete::<Identity, Impl, OFFSET>,
            StartItemDelete: StartItemDelete::<Identity, Impl, OFFSET>,
            UpdateDeletePercent: UpdateDeletePercent::<Identity, Impl, OFFSET>,
            EndItemDelete: EndItemDelete::<Identity, Impl, OFFSET>,
            EndDelete: EndDelete::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
            GetDeleteAfterAcquire: GetDeleteAfterAcquire::<Identity, Impl, OFFSET>,
            ErrorAdvise: ErrorAdvise::<Identity, Impl, OFFSET>,
            GetUserInput: GetUserInput::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireSettings_Impl: ::windows_core::BaseImpl {
    fn InitializeFromRegistry(this: &Self::This, pszregistrykey: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFlags(this: &Self::This, dwphotoacquireflags: u32) -> ::windows_core::Result<()>;
    fn SetOutputFilenameTemplate(this: &Self::This, psztemplate: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetSequencePaddingWidth(this: &Self::This, dwwidth: u32) -> ::windows_core::Result<()>;
    fn SetSequenceZeroPadding(this: &Self::This, fzeropad: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetGroupTag(this: &Self::This, pszgrouptag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetAcquisitionTime(this: &Self::This, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOutputFilenameTemplate(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSequencePaddingWidth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSequenceZeroPadding(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetGroupTag(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAcquisitionTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPhotoAcquireSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquireSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFromRegistry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszregistrykey: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromRegistry(this, ::core::mem::transmute(&pszregistrykey)).into())
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&dwphotoacquireflags)).into())
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztemplate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputFilenameTemplate(this, ::core::mem::transmute(&psztemplate)).into())
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSequencePaddingWidth(this, ::core::mem::transmute_copy(&dwwidth)).into())
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSequenceZeroPadding(this, ::core::mem::transmute_copy(&fzeropad)).into())
        }
        unsafe extern "system" fn SetGroupTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszgrouptag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupTag(this, ::core::mem::transmute(&pszgrouptag)).into())
        }
        unsafe extern "system" fn SetAcquisitionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAcquisitionTime(this, ::core::mem::transmute_copy(&pftacquisitiontime)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwphotoacquireflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputFilenameTemplate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSequencePaddingWidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSequenceZeroPadding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfzeropad, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGroupTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGroupTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrgrouptag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAcquisitionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAcquisitionTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftacquisitiontime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPhotoAcquireSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFromRegistry: InitializeFromRegistry::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            SetOutputFilenameTemplate: SetOutputFilenameTemplate::<Identity, Impl, OFFSET>,
            SetSequencePaddingWidth: SetSequencePaddingWidth::<Identity, Impl, OFFSET>,
            SetSequenceZeroPadding: SetSequenceZeroPadding::<Identity, Impl, OFFSET>,
            SetGroupTag: SetGroupTag::<Identity, Impl, OFFSET>,
            SetAcquisitionTime: SetAcquisitionTime::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetOutputFilenameTemplate: GetOutputFilenameTemplate::<Identity, Impl, OFFSET>,
            GetSequencePaddingWidth: GetSequencePaddingWidth::<Identity, Impl, OFFSET>,
            GetSequenceZeroPadding: GetSequenceZeroPadding::<Identity, Impl, OFFSET>,
            GetGroupTag: GetGroupTag::<Identity, Impl, OFFSET>,
            GetAcquisitionTime: GetAcquisitionTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoAcquireSource_Impl: ::windows_core::BaseImpl {
    fn GetFriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDeviceIcons(this: &Self::This, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<()>;
    fn InitializeItemList(this: &Self::This, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::core::option::Option<&IPhotoAcquireProgressCB>, pnitemcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetItemAt(this: &Self::This, nindex: u32) -> ::windows_core::Result<IPhotoAcquireItem>;
    fn GetPhotoAcquireSettings(this: &Self::This) -> ::windows_core::Result<IPhotoAcquireSettings>;
    fn GetDeviceId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BindToObject(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPhotoAcquireSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoAcquireSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceIcons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceIcons(this, ::core::mem::transmute_copy(&nsize), ::core::mem::transmute_copy(&phlargeicon), ::core::mem::transmute_copy(&phsmallicon)).into())
        }
        unsafe extern "system" fn InitializeItemList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeItemList(this, ::core::mem::transmute_copy(&fforceenumeration), ::windows_core::from_raw_borrowed(&pphotoacquireprogresscb), ::core::mem::transmute_copy(&pnitemcount)).into())
        }
        unsafe extern "system" fn GetItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnitemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemAt(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquireitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPhotoAcquireSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquiresettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BindToObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToObject(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IPhotoAcquireSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetDeviceIcons: GetDeviceIcons::<Identity, Impl, OFFSET>,
            InitializeItemList: InitializeItemList::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
            GetItemAt: GetItemAt::<Identity, Impl, OFFSET>,
            GetPhotoAcquireSettings: GetPhotoAcquireSettings::<Identity, Impl, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoProgressActionCB_Impl: ::windows_core::BaseImpl {
    fn DoAction(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPhotoProgressActionCB {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressActionCB_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoProgressActionCB {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressActionCB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoAction(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        IPhotoProgressActionCB_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DoAction: DoAction::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoProgressDialog_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetWindow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn Destroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetTitle(this: &Self::This, psztitle: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ShowCheckbox(this: &Self::This, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCheckboxText(this: &Self::This, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetCheckboxCheck(this: &Self::This, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCheckboxTooltip(this: &Self::This, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsCheckboxChecked(this: &Self::This, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCaption(this: &Self::This, psztitle: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetImage(this: &Self::This, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
    fn SetPercentComplete(this: &Self::This, npercent: i32) -> ::windows_core::Result<()>;
    fn SetProgressText(this: &Self::This, pszprogresstext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetActionLinkCallback(this: &Self::This, pphotoprogressactioncb: ::core::option::Option<&IPhotoProgressActionCB>) -> ::windows_core::Result<()>;
    fn SetActionLinkText(this: &Self::This, pszcaption: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ShowActionLink(this: &Self::This, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsCancelled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetUserInput(this: &Self::This, riidtype: *const ::windows_core::GUID, punknown: ::core::option::Option<&::windows_core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPhotoProgressDialog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPhotoProgressDialog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwndprogressdialog, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this).into())
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&psztitle)).into())
        }
        unsafe extern "system" fn ShowCheckbox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowCheckbox(this, ::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn SetCheckboxText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCheckboxText(this, ::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute(&pszcheckboxtext)).into())
        }
        unsafe extern "system" fn SetCheckboxCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCheckboxCheck(this, ::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&fchecked)).into())
        }
        unsafe extern "system" fn SetCheckboxTooltip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCheckboxTooltip(this, ::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute(&pszcheckboxtooltiptext)).into())
        }
        unsafe extern "system" fn IsCheckboxChecked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCheckboxChecked(this, ::core::mem::transmute_copy(&ncheckboxid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfchecked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCaption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCaption(this, ::core::mem::transmute(&psztitle)).into())
        }
        unsafe extern "system" fn SetImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImage(this, ::core::mem::transmute_copy(&nimagetype), ::core::mem::transmute_copy(&hicon), ::core::mem::transmute_copy(&hbitmap)).into())
        }
        unsafe extern "system" fn SetPercentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPercentComplete(this, ::core::mem::transmute_copy(&npercent)).into())
        }
        unsafe extern "system" fn SetProgressText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszprogresstext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProgressText(this, ::core::mem::transmute(&pszprogresstext)).into())
        }
        unsafe extern "system" fn SetActionLinkCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionLinkCallback(this, ::windows_core::from_raw_borrowed(&pphotoprogressactioncb)).into())
        }
        unsafe extern "system" fn SetActionLinkText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcaption: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionLinkText(this, ::core::mem::transmute(&pszcaption)).into())
        }
        unsafe extern "system" fn ShowActionLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowActionLink(this, ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn IsCancelled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCancelled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancelled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows_core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserInput(this, ::core::mem::transmute_copy(&riidtype), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&ppropvarresult), ::core::mem::transmute_copy(&ppropvardefault)).into())
        }
        IPhotoProgressDialog_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            ShowCheckbox: ShowCheckbox::<Identity, Impl, OFFSET>,
            SetCheckboxText: SetCheckboxText::<Identity, Impl, OFFSET>,
            SetCheckboxCheck: SetCheckboxCheck::<Identity, Impl, OFFSET>,
            SetCheckboxTooltip: SetCheckboxTooltip::<Identity, Impl, OFFSET>,
            IsCheckboxChecked: IsCheckboxChecked::<Identity, Impl, OFFSET>,
            SetCaption: SetCaption::<Identity, Impl, OFFSET>,
            SetImage: SetImage::<Identity, Impl, OFFSET>,
            SetPercentComplete: SetPercentComplete::<Identity, Impl, OFFSET>,
            SetProgressText: SetProgressText::<Identity, Impl, OFFSET>,
            SetActionLinkCallback: SetActionLinkCallback::<Identity, Impl, OFFSET>,
            SetActionLinkText: SetActionLinkText::<Identity, Impl, OFFSET>,
            ShowActionLink: ShowActionLink::<Identity, Impl, OFFSET>,
            IsCancelled: IsCancelled::<Identity, Impl, OFFSET>,
            GetUserInput: GetUserInput::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IUserInputString_Impl: ::windows_core::BaseImpl {
    fn GetSubmitButtonText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPrompt(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetStringId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetStringType(this: &Self::This) -> ::windows_core::Result<USER_INPUT_STRING_TYPE>;
    fn GetTooltipText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetMaxLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDefault(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetMruCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMruEntryAt(this: &Self::This, nindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetImage(this: &Self::This, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IUserInputString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserInputString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSubmitButtonText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubmitButtonText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmitbuttontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrompt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrompt(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprompttitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstringid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstringid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnstringtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTooltipText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTooltipText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtooltiptext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchmaxlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdefault: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMruCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMruCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnmrucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMruEntryAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMruEntryAt(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmruentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImage(this, ::core::mem::transmute_copy(&nsize), ::core::mem::transmute_copy(&phbitmap), ::core::mem::transmute_copy(&phicon)).into())
        }
        IUserInputString_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSubmitButtonText: GetSubmitButtonText::<Identity, Impl, OFFSET>,
            GetPrompt: GetPrompt::<Identity, Impl, OFFSET>,
            GetStringId: GetStringId::<Identity, Impl, OFFSET>,
            GetStringType: GetStringType::<Identity, Impl, OFFSET>,
            GetTooltipText: GetTooltipText::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetDefault: GetDefault::<Identity, Impl, OFFSET>,
            GetMruCount: GetMruCount::<Identity, Impl, OFFSET>,
            GetMruEntryAt: GetMruEntryAt::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
