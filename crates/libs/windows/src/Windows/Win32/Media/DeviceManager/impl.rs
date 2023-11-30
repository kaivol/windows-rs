pub trait IComponentAuthenticate_Impl: ::windows_core::BaseImpl {
    fn SACAuth(this: &Self::This, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows_core::Result<()>;
    fn SACGetProtocols(this: &Self::This, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComponentAuthenticate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentAuthenticate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComponentAuthenticate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SACAuth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentAuthenticate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SACAuth(this, ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&dwpass), ::core::mem::transmute_copy(&pbdatain), ::core::mem::transmute_copy(&dwdatainlen), ::core::mem::transmute_copy(&ppbdataout), ::core::mem::transmute_copy(&pdwdataoutlen)).into())
        }
        unsafe extern "system" fn SACGetProtocols<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentAuthenticate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SACGetProtocols(this, ::core::mem::transmute_copy(&ppdwprotocols), ::core::mem::transmute_copy(&pdwprotocolcount)).into())
        }
        IComponentAuthenticate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SACAuth: SACAuth::<Identity, Impl, OFFSET>,
            SACGetProtocols: SACGetProtocols::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDevice_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetManufacturer(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSerialNumber(this: &Self::This, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn GetPowerSource(this: &Self::This, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDeviceIcon(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumStorage(this: &Self::This) -> ::windows_core::Result<IMDSPEnumStorage>;
    fn GetFormatSupport(this: &Self::This, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()>;
    fn SendOpaqueCommand(this: &Self::This, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IMDSPDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetManufacturer(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerialNumber(this, ::core::mem::transmute_copy(&pserialnumber), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn GetPowerSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPowerSource(this, ::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceIcon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStorage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormatSupport(this, ::core::mem::transmute_copy(&pformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)).into())
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOpaqueCommand(this, ::core::mem::transmute_copy(&pcommand)).into())
        }
        IMDSPDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetManufacturer: GetManufacturer::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetPowerSource: GetPowerSource::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport: GetFormatSupport::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Ole\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice2_Impl: ::windows_core::BaseImpl + IMDSPDevice_Impl {
    fn GetStorage(this: &Self::This, pszstoragename: &::windows_core::PCWSTR) -> ::windows_core::Result<IMDSPStorage>;
    fn GetFormatSupport2(this: &Self::This, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::Result<()>;
    fn GetSpecifyPropertyPages(this: &Self::This, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::Result<()>;
    fn GetCanonicalName(this: &Self::This, pwszpnpname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl ::windows_core::Iids for IMDSPDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDSPDevice);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorage(this, ::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormatSupport2(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)).into())
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecifyPropertyPages(this, ::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)).into())
        }
        unsafe extern "system" fn GetCanonicalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCanonicalName(this, ::core::mem::transmute_copy(&pwszpnpname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        IMDSPDevice2_Vtbl {
            base__: <IMDSPDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Identity, Impl, OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Identity, Impl, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMDSPDevice3_Impl: ::windows_core::BaseImpl + IMDSPDevice2_Impl {
    fn GetProperty(this: &Self::This, pwszpropname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(this: &Self::This, pwszpropname: &::windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetFormatCapability(this: &Self::This, format: WMDM_FORMATCODE) -> ::windows_core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(this: &Self::This, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn FindStorage(this: &Self::This, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMDSPDevice3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDSPDevice2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPDevice3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute(&pwszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&pwszpropname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetFormatCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatCapability(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceIoControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceIoControl(this, ::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&pnoutbuffersize)).into())
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindStorage(this, ::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPDevice3_Vtbl {
            base__: <IMDSPDevice2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetFormatCapability: GetFormatCapability::<Identity, Impl, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDeviceControl_Impl: ::windows_core::BaseImpl {
    fn GetDCStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Play(this: &Self::This) -> ::windows_core::Result<()>;
    fn Record(this: &Self::This, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, fumode: u32, noffset: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IMDSPDeviceControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPDeviceControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDCStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDCStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilitiesmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Play(this).into())
        }
        unsafe extern "system" fn Record<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Record(this, ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&noffset)).into())
        }
        IMDSPDeviceControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDCStatus: GetDCStatus::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            Record: Record::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPDirectTransfer_Impl: ::windows_core::BaseImpl {
    fn TransferToDevice(this: &Self::This, pwszsourcefilepath: &::windows_core::PCWSTR, psourceoperation: ::core::option::Option<&IWMDMOperation>, fuflags: u32, pwszdestinationname: &::windows_core::PCWSTR, psourcemetadata: ::core::option::Option<&IWMDMMetaData>, ptransferprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<IMDSPStorage>;
}
impl ::windows_core::Iids for IMDSPDirectTransfer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDirectTransfer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPDirectTransfer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransferToDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPDirectTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsourcefilepath: ::windows_core::PCWSTR, psourceoperation: *mut ::core::ffi::c_void, fuflags: u32, pwszdestinationname: ::windows_core::PCWSTR, psourcemetadata: *mut ::core::ffi::c_void, ptransferprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransferToDevice(this, ::core::mem::transmute(&pwszsourcefilepath), ::windows_core::from_raw_borrowed(&psourceoperation), ::core::mem::transmute_copy(&fuflags), ::core::mem::transmute(&pwszdestinationname), ::windows_core::from_raw_borrowed(&psourcemetadata), ::windows_core::from_raw_borrowed(&ptransferprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPDirectTransfer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransferToDevice: TransferToDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPEnumDevice_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppdevice: *mut ::core::option::Option<IMDSPDevice>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<u32>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IMDSPEnumDevice>;
}
impl ::windows_core::Iids for IMDSPEnumDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPEnumDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Skip(this, ::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPEnumDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPEnumStorage_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppstorage: *mut ::core::option::Option<IMDSPStorage>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<u32>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IMDSPEnumStorage>;
}
impl ::windows_core::Iids for IMDSPEnumStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPEnumStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Skip(this, ::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPEnumStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPObject_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, fumode: u32) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, fumode: u32, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, fuflags: u32, dwoffset: u32) -> ::windows_core::Result<()>;
    fn Rename(this: &Self::This, pwsznewname: &::windows_core::PCWSTR, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
    fn Move(this: &Self::This, fumode: u32, pprogress: ::core::option::Option<&IWMDMProgress>, ptarget: ::core::option::Option<&IMDSPStorage>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMDSPObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute_copy(&fumode)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&fumode), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&dwoffset)).into())
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsznewname: ::windows_core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute(&pwsznewname), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute_copy(&fumode), ::windows_core::from_raw_borrowed(&pprogress), ::windows_core::from_raw_borrowed(&ptarget)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IMDSPObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPObject2_Impl: ::windows_core::BaseImpl + IMDSPObject_Impl {
    fn ReadOnClearChannel(this: &Self::This, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
    fn WriteOnClearChannel(this: &Self::This, pdata: *const u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMDSPObject2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDSPObject);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPObject2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadOnClearChannel(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        unsafe extern "system" fn WriteOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObject2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteOnClearChannel(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        IMDSPObject2_Vtbl {
            base__: <IMDSPObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadOnClearChannel: ReadOnClearChannel::<Identity, Impl, OFFSET>,
            WriteOnClearChannel: WriteOnClearChannel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPObjectInfo_Impl: ::windows_core::BaseImpl {
    fn GetPlayLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetPlayLength(this: &Self::This, dwlength: u32) -> ::windows_core::Result<()>;
    fn GetPlayOffset(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetPlayOffset(this: &Self::This, dwoffset: u32) -> ::windows_core::Result<()>;
    fn GetTotalLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLastPlayPosition(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLongestPlayPosition(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMDSPObjectInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPObjectInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPlayLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPlayLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlayLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayLength(this, ::core::mem::transmute_copy(&dwlength)).into())
        }
        unsafe extern "system" fn GetPlayOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPlayOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlayOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayOffset(this, ::core::mem::transmute_copy(&dwoffset)).into())
        }
        unsafe extern "system" fn GetTotalLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTotalLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastPlayPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLongestPlayPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlongestpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPObjectInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPlayLength: GetPlayLength::<Identity, Impl, OFFSET>,
            SetPlayLength: SetPlayLength::<Identity, Impl, OFFSET>,
            GetPlayOffset: GetPlayOffset::<Identity, Impl, OFFSET>,
            SetPlayOffset: SetPlayOffset::<Identity, Impl, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, Impl, OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Identity, Impl, OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPRevoked_Impl: ::windows_core::BaseImpl {
    fn GetRevocationURL(this: &Self::This, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMDSPRevoked {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPRevoked_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPRevoked {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRevocationURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPRevoked_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRevocationURL(this, ::core::mem::transmute_copy(&ppwszrevocationurl), ::core::mem::transmute_copy(&pdwbufferlen)).into())
        }
        IMDSPRevoked_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRevocationURL: GetRevocationURL::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPStorage_Impl: ::windows_core::BaseImpl {
    fn SetAttributes(this: &Self::This, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetStorageGlobals(this: &Self::This) -> ::windows_core::Result<IMDSPStorageGlobals>;
    fn GetAttributes(this: &Self::This, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetDate(this: &Self::This) -> ::windows_core::Result<WMDMDATETIME>;
    fn GetSize(this: &Self::This, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetRights(this: &Self::This, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn CreateStorage(this: &Self::This, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<IMDSPStorage>;
    fn EnumStorage(this: &Self::This) -> ::windows_core::Result<IMDSPEnumStorage>;
    fn SendOpaqueCommand(this: &Self::This, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IMDSPStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributes(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorageGlobals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorageglobals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributes(this, ::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetimeutc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)).into())
        }
        unsafe extern "system" fn GetRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRights(this, ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn CreateStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: ::windows_core::PCWSTR, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStorage(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute(&pwszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStorage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOpaqueCommand(this, ::core::mem::transmute_copy(&pcommand)).into())
        }
        IMDSPStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAttributes: SetAttributes::<Identity, Impl, OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDate: GetDate::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetRights: GetRights::<Identity, Impl, OFFSET>,
            CreateStorage: CreateStorage::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage2_Impl: ::windows_core::BaseImpl + IMDSPStorage_Impl {
    fn GetStorage(this: &Self::This, pszstoragename: &::windows_core::PCWSTR) -> ::windows_core::Result<IMDSPStorage>;
    fn CreateStorage2(this: &Self::This, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: &::windows_core::PCWSTR, qwfilesize: u64) -> ::windows_core::Result<IMDSPStorage>;
    fn SetAttributes2(this: &Self::This, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::Result<()>;
    fn GetAttributes2(this: &Self::This, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IMDSPStorage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDSPStorage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPStorage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorage(this, ::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStorage2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: ::windows_core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStorage2(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&qwfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttributes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributes2(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into())
        }
        unsafe extern "system" fn GetAttributes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributes2(this, ::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into())
        }
        IMDSPStorage2_Vtbl {
            base__: <IMDSPStorage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            CreateStorage2: CreateStorage2::<Identity, Impl, OFFSET>,
            SetAttributes2: SetAttributes2::<Identity, Impl, OFFSET>,
            GetAttributes2: GetAttributes2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage3_Impl: ::windows_core::BaseImpl + IMDSPStorage2_Impl {
    fn GetMetadata(this: &Self::This, pmetadata: ::core::option::Option<&IWMDMMetaData>) -> ::windows_core::Result<()>;
    fn SetMetadata(this: &Self::This, pmetadata: ::core::option::Option<&IWMDMMetaData>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IMDSPStorage3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDSPStorage2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPStorage3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetadata(this, ::windows_core::from_raw_borrowed(&pmetadata)).into())
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMetadata(this, ::windows_core::from_raw_borrowed(&pmetadata)).into())
        }
        IMDSPStorage3_Vtbl {
            base__: <IMDSPStorage2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetadata: GetMetadata::<Identity, Impl, OFFSET>,
            SetMetadata: SetMetadata::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage4_Impl: ::windows_core::BaseImpl + IMDSPStorage3_Impl {
    fn SetReferences(this: &Self::This, dwrefs: u32, ppispstorage: *const ::core::option::Option<IMDSPStorage>) -> ::windows_core::Result<()>;
    fn GetReferences(this: &Self::This, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows_core::Result<()>;
    fn CreateStorageWithMetadata(this: &Self::This, dwattributes: u32, pwszname: &::windows_core::PCWSTR, pmetadata: ::core::option::Option<&IWMDMMetaData>, qwfilesize: u64) -> ::windows_core::Result<IMDSPStorage>;
    fn GetSpecifiedMetadata(this: &Self::This, cproperties: u32, ppwszpropnames: *const ::windows_core::PCWSTR, pmetadata: ::core::option::Option<&IWMDMMetaData>) -> ::windows_core::Result<()>;
    fn FindStorage(this: &Self::This, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<IMDSPStorage>;
    fn GetParent(this: &Self::This) -> ::windows_core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IMDSPStorage4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDSPStorage3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPStorage4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReferences(this, ::core::mem::transmute_copy(&dwrefs), ::core::mem::transmute_copy(&ppispstorage)).into())
        }
        unsafe extern "system" fn GetReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReferences(this, ::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppispstorage)).into())
        }
        unsafe extern "system" fn CreateStorageWithMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: ::windows_core::PCWSTR, pmetadata: *mut ::core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStorageWithMetadata(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute(&pwszname), ::windows_core::from_raw_borrowed(&pmetadata), ::core::mem::transmute_copy(&qwfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows_core::PCWSTR, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecifiedMetadata(this, ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppwszpropnames), ::windows_core::from_raw_borrowed(&pmetadata)).into())
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindStorage(this, ::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPStorage4_Vtbl {
            base__: <IMDSPStorage3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetReferences: SetReferences::<Identity, Impl, OFFSET>,
            GetReferences: GetReferences::<Identity, Impl, OFFSET>,
            CreateStorageWithMetadata: CreateStorageWithMetadata::<Identity, Impl, OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDSPStorageGlobals_Impl: ::windows_core::BaseImpl {
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSerialNumber(this: &Self::This, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn GetTotalSize(this: &Self::This, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetTotalFree(this: &Self::This, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetTotalBad(this: &Self::This, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Initialize(this: &Self::This, fumode: u32, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IMDSPDevice>;
    fn GetRootStorage(this: &Self::This) -> ::windows_core::Result<IMDSPStorage>;
}
impl ::windows_core::Iids for IMDSPStorageGlobals {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDSPStorageGlobals {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerialNumber(this, ::core::mem::transmute_copy(&pserialnum), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn GetTotalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTotalSize(this, ::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)).into())
        }
        unsafe extern "system" fn GetTotalFree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTotalFree(this, ::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)).into())
        }
        unsafe extern "system" fn GetTotalBad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTotalBad(this, ::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&fumode), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootStorage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDSPStorageGlobals_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetTotalSize: GetTotalSize::<Identity, Impl, OFFSET>,
            GetTotalFree: GetTotalFree::<Identity, Impl, OFFSET>,
            GetTotalBad: GetTotalBad::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetRootStorage: GetRootStorage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDServiceProvider_Impl: ::windows_core::BaseImpl {
    fn GetDeviceCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumDevices(this: &Self::This) -> ::windows_core::Result<IMDSPEnumDevice>;
}
impl ::windows_core::Iids for IMDServiceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDServiceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDServiceProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceCount: GetDeviceCount::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDServiceProvider2_Impl: ::windows_core::BaseImpl + IMDServiceProvider_Impl {
    fn CreateDevice(this: &Self::This, pwszdevicepath: &::windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMDServiceProvider2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDServiceProvider);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDServiceProvider2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute(&pwszdevicepath), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pppdevicearray)).into())
        }
        IMDServiceProvider2_Vtbl { base__: <IMDServiceProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice: CreateDevice::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMDServiceProvider3_Impl: ::windows_core::BaseImpl + IMDServiceProvider2_Impl {
    fn SetDeviceEnumPreference(this: &Self::This, dwenumpref: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMDServiceProvider3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMDServiceProvider2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDServiceProvider3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDServiceProvider3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceEnumPreference(this, ::core::mem::transmute_copy(&dwenumpref)).into())
        }
        IMDServiceProvider3_Vtbl {
            base__: <IMDServiceProvider2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureAuthenticate_Impl: ::windows_core::BaseImpl {
    fn GetSecureQuery(this: &Self::This) -> ::windows_core::Result<ISCPSecureQuery>;
}
impl ::windows_core::Iids for ISCPSecureAuthenticate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureAuthenticate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureAuthenticate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSecureQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureAuthenticate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecureQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurequery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISCPSecureAuthenticate_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSecureQuery: GetSecureQuery::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureAuthenticate2_Impl: ::windows_core::BaseImpl + ISCPSecureAuthenticate_Impl {
    fn GetSCPSession(this: &Self::This) -> ::windows_core::Result<ISCPSession>;
}
impl ::windows_core::Iids for ISCPSecureAuthenticate2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISCPSecureAuthenticate);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureAuthenticate2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureAuthenticate2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSCPSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureAuthenticate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscpsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSCPSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscpsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISCPSecureAuthenticate2_Vtbl { base__: <ISCPSecureAuthenticate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSCPSession: GetSCPSession::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureExchange_Impl: ::windows_core::BaseImpl {
    fn TransferContainerData(this: &Self::This, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn ObjectData(this: &Self::This, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn TransferComplete(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISCPSecureExchange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureExchange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransferContainerData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferContainerData(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pfureadyflags), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn ObjectData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ObjectData(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn TransferComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferComplete(this).into())
        }
        ISCPSecureExchange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransferContainerData: TransferContainerData::<Identity, Impl, OFFSET>,
            ObjectData: ObjectData::<Identity, Impl, OFFSET>,
            TransferComplete: TransferComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureExchange2_Impl: ::windows_core::BaseImpl + ISCPSecureExchange_Impl {
    fn TransferContainerData2(this: &Self::This, pdata: *const u8, dwsize: u32, pprogresscallback: ::core::option::Option<&IWMDMProgress3>, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISCPSecureExchange2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISCPSecureExchange);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureExchange2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransferContainerData2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferContainerData2(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::windows_core::from_raw_borrowed(&pprogresscallback), ::core::mem::transmute_copy(&pfureadyflags), ::core::mem::transmute_copy(&abmac)).into())
        }
        ISCPSecureExchange2_Vtbl {
            base__: <ISCPSecureExchange as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransferContainerData2: TransferContainerData2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureExchange3_Impl: ::windows_core::BaseImpl + ISCPSecureExchange2_Impl {
    fn TransferContainerDataOnClearChannel(this: &Self::This, pdevice: ::core::option::Option<&IMDSPDevice>, pdata: *const u8, dwsize: u32, pprogresscallback: ::core::option::Option<&IWMDMProgress3>) -> ::windows_core::Result<u32>;
    fn GetObjectDataOnClearChannel(this: &Self::This, pdevice: ::core::option::Option<&IMDSPDevice>, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
    fn TransferCompleteForDevice(this: &Self::This, pdevice: ::core::option::Option<&IMDSPDevice>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISCPSecureExchange3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISCPSecureExchange2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureExchange3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransferContainerDataOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransferContainerDataOnClearChannel(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::windows_core::from_raw_borrowed(&pprogresscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfureadyflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectDataOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectDataOnClearChannel(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        unsafe extern "system" fn TransferCompleteForDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferCompleteForDevice(this, ::windows_core::from_raw_borrowed(&pdevice)).into())
        }
        ISCPSecureExchange3_Vtbl {
            base__: <ISCPSecureExchange2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransferContainerDataOnClearChannel: TransferContainerDataOnClearChannel::<Identity, Impl, OFFSET>,
            GetObjectDataOnClearChannel: GetObjectDataOnClearChannel::<Identity, Impl, OFFSET>,
            TransferCompleteForDevice: TransferCompleteForDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureQuery_Impl: ::windows_core::BaseImpl {
    fn GetDataDemands(this: &Self::This, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn ExamineData(this: &Self::This, fuflags: u32, pwszextension: &::windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn MakeDecision(this: &Self::This, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::core::option::Option<&IMDSPStorageGlobals>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn GetRights(this: &Self::This, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::core::option::Option<&IMDSPStorageGlobals>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISCPSecureQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDataDemands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataDemands(this, ::core::mem::transmute_copy(&pfuflags), ::core::mem::transmute_copy(&pdwminrightsdata), ::core::mem::transmute_copy(&pdwminexaminedata), ::core::mem::transmute_copy(&pdwmindecidedata), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn ExamineData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: ::windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExamineData(this, ::core::mem::transmute_copy(&fuflags), ::core::mem::transmute(&pwszextension), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn MakeDecision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeDecision(this, ::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwappsec), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::windows_core::from_raw_borrowed(&pstorageglobals), ::core::mem::transmute_copy(&ppexchange), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn GetRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRights(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::windows_core::from_raw_borrowed(&pstgglobals), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into())
        }
        ISCPSecureQuery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDataDemands: GetDataDemands::<Identity, Impl, OFFSET>,
            ExamineData: ExamineData::<Identity, Impl, OFFSET>,
            MakeDecision: MakeDecision::<Identity, Impl, OFFSET>,
            GetRights: GetRights::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureQuery2_Impl: ::windows_core::BaseImpl + ISCPSecureQuery_Impl {
    fn MakeDecision2(this: &Self::This, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::core::option::Option<&IMDSPStorageGlobals>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: ::core::option::Option<&::windows_core::IUnknown>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISCPSecureQuery2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISCPSecureQuery);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureQuery2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MakeDecision2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MakeDecision2(
                    this,
                    ::core::mem::transmute_copy(&fuflags),
                    ::core::mem::transmute_copy(&pdata),
                    ::core::mem::transmute_copy(&dwsize),
                    ::core::mem::transmute_copy(&dwappsec),
                    ::core::mem::transmute_copy(&pbspsessionkey),
                    ::core::mem::transmute_copy(&dwsessionkeylen),
                    ::windows_core::from_raw_borrowed(&pstorageglobals),
                    ::core::mem::transmute_copy(&pappcertapp),
                    ::core::mem::transmute_copy(&dwappcertapplen),
                    ::core::mem::transmute_copy(&pappcertsp),
                    ::core::mem::transmute_copy(&dwappcertsplen),
                    ::core::mem::transmute_copy(&pszrevocationurl),
                    ::core::mem::transmute_copy(&pdwrevocationurllen),
                    ::core::mem::transmute_copy(&pdwrevocationbitflag),
                    ::core::mem::transmute_copy(&pqwfilesize),
                    ::windows_core::from_raw_borrowed(&punknown),
                    ::core::mem::transmute_copy(&ppexchange),
                    ::core::mem::transmute_copy(&abmac),
                )
                .into()
            })
        }
        ISCPSecureQuery2_Vtbl { base__: <ISCPSecureQuery as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, MakeDecision2: MakeDecision2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSecureQuery3_Impl: ::windows_core::BaseImpl + ISCPSecureQuery2_Impl {
    fn GetRightsOnClearChannel(this: &Self::This, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::core::option::Option<&IMDSPStorageGlobals>, pprogresscallback: ::core::option::Option<&IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::Result<()>;
    fn MakeDecisionOnClearChannel(this: &Self::This, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::core::option::Option<&IMDSPStorageGlobals>, pprogresscallback: ::core::option::Option<&IWMDMProgress3>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: ::core::option::Option<&::windows_core::IUnknown>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISCPSecureQuery3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISCPSecureQuery2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSecureQuery3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRightsOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRightsOnClearChannel(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::windows_core::from_raw_borrowed(&pstgglobals), ::windows_core::from_raw_borrowed(&pprogresscallback), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)).into())
        }
        unsafe extern "system" fn MakeDecisionOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSecureQuery3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MakeDecisionOnClearChannel(
                    this,
                    ::core::mem::transmute_copy(&fuflags),
                    ::core::mem::transmute_copy(&pdata),
                    ::core::mem::transmute_copy(&dwsize),
                    ::core::mem::transmute_copy(&dwappsec),
                    ::core::mem::transmute_copy(&pbspsessionkey),
                    ::core::mem::transmute_copy(&dwsessionkeylen),
                    ::windows_core::from_raw_borrowed(&pstorageglobals),
                    ::windows_core::from_raw_borrowed(&pprogresscallback),
                    ::core::mem::transmute_copy(&pappcertapp),
                    ::core::mem::transmute_copy(&dwappcertapplen),
                    ::core::mem::transmute_copy(&pappcertsp),
                    ::core::mem::transmute_copy(&dwappcertsplen),
                    ::core::mem::transmute_copy(&pszrevocationurl),
                    ::core::mem::transmute_copy(&pdwrevocationurllen),
                    ::core::mem::transmute_copy(&pdwrevocationbitflag),
                    ::core::mem::transmute_copy(&pqwfilesize),
                    ::windows_core::from_raw_borrowed(&punknown),
                    ::core::mem::transmute_copy(&ppexchange),
                )
                .into()
            })
        }
        ISCPSecureQuery3_Vtbl {
            base__: <ISCPSecureQuery2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRightsOnClearChannel: GetRightsOnClearChannel::<Identity, Impl, OFFSET>,
            MakeDecisionOnClearChannel: MakeDecisionOnClearChannel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISCPSession_Impl: ::windows_core::BaseImpl {
    fn BeginSession(this: &Self::This, pidevice: ::core::option::Option<&IMDSPDevice>, pctx: *const u8, dwsizectx: u32) -> ::windows_core::Result<()>;
    fn EndSession(this: &Self::This, pctx: *const u8, dwsizectx: u32) -> ::windows_core::Result<()>;
    fn GetSecureQuery(this: &Self::This) -> ::windows_core::Result<ISCPSecureQuery>;
}
impl ::windows_core::Iids for ISCPSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISCPSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidevice: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginSession(this, ::windows_core::from_raw_borrowed(&pidevice), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into())
        }
        unsafe extern "system" fn EndSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSession(this, ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into())
        }
        unsafe extern "system" fn GetSecureQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecureQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurequery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISCPSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
            GetSecureQuery: GetSecureQuery::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDevice_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetManufacturer(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSerialNumber(this: &Self::This, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn GetPowerSource(this: &Self::This, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDeviceIcon(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumStorage(this: &Self::This) -> ::windows_core::Result<IWMDMEnumStorage>;
    fn GetFormatSupport(this: &Self::This, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()>;
    fn SendOpaqueCommand(this: &Self::This, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IWMDMDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetManufacturer(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerialNumber(this, ::core::mem::transmute_copy(&pserialnumber), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn GetPowerSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPowerSource(this, ::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceIcon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStorage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormatSupport(this, ::core::mem::transmute_copy(&ppformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)).into())
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOpaqueCommand(this, ::core::mem::transmute_copy(&pcommand)).into())
        }
        IWMDMDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetManufacturer: GetManufacturer::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetPowerSource: GetPowerSource::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport: GetFormatSupport::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Ole\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice2_Impl: ::windows_core::BaseImpl + IWMDMDevice_Impl {
    fn GetStorage(this: &Self::This, pszstoragename: &::windows_core::PCWSTR) -> ::windows_core::Result<IWMDMStorage>;
    fn GetFormatSupport2(this: &Self::This, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::Result<()>;
    fn GetSpecifyPropertyPages(this: &Self::This, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::Result<()>;
    fn GetCanonicalName(this: &Self::This, pwszpnpname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl ::windows_core::Iids for IWMDMDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMDevice);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorage(this, ::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormatSupport2(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)).into())
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecifyPropertyPages(this, ::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)).into())
        }
        unsafe extern "system" fn GetCanonicalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCanonicalName(this, ::core::mem::transmute_copy(&pwszpnpname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        IWMDMDevice2_Vtbl {
            base__: <IWMDMDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Identity, Impl, OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Identity, Impl, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMDMDevice3_Impl: ::windows_core::BaseImpl + IWMDMDevice2_Impl {
    fn GetProperty(this: &Self::This, pwszpropname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(this: &Self::This, pwszpropname: &::windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetFormatCapability(this: &Self::This, format: WMDM_FORMATCODE) -> ::windows_core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(this: &Self::This, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn FindStorage(this: &Self::This, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMDMDevice3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMDevice2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMDevice3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute(&pwszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&pwszpropname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetFormatCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatCapability(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceIoControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceIoControl(this, ::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&pnoutbuffersize)).into())
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindStorage(this, ::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDMDevice3_Vtbl {
            base__: <IWMDMDevice2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetFormatCapability: GetFormatCapability::<Identity, Impl, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDeviceControl_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Play(this: &Self::This) -> ::windows_core::Result<()>;
    fn Record(this: &Self::This, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, fumode: u32, noffset: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IWMDMDeviceControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMDeviceControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilitiesmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Play(this).into())
        }
        unsafe extern "system" fn Record<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Record(this, ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&noffset)).into())
        }
        IWMDMDeviceControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            Record: Record::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMDeviceSession_Impl: ::windows_core::BaseImpl {
    fn BeginSession(this: &Self::This, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_core::Result<()>;
    fn EndSession(this: &Self::This, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMDeviceSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMDeviceSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginSession(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into())
        }
        unsafe extern "system" fn EndSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMDeviceSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSession(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into())
        }
        IWMDMDeviceSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMEnumDevice_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppdevice: *mut ::core::option::Option<IWMDMDevice>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<u32>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IWMDMEnumDevice>;
}
impl ::windows_core::Iids for IWMDMEnumDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMEnumDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Skip(this, ::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDMEnumDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMEnumStorage_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppstorage: *mut ::core::option::Option<IWMDMStorage>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<u32>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IWMDMEnumStorage>;
}
impl ::windows_core::Iids for IWMDMEnumStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMEnumStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Skip(this, ::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDMEnumStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMLogger_Impl: ::windows_core::BaseImpl {
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Enable(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLogFileName(this: &Self::This, pszfilename: ::windows_core::PSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn SetLogFileName(this: &Self::This, pszfilename: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn LogString(this: &Self::This, dwflags: u32, pszsrcname: &::windows_core::PCSTR, pszlog: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn LogDword(this: &Self::This, dwflags: u32, pszsrcname: &::windows_core::PCSTR, pszlogformat: &::windows_core::PCSTR, dwlog: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetSizeParams(this: &Self::This, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows_core::Result<()>;
    fn SetSizeParams(this: &Self::This, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMDMLogger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMLogger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn GetLogFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLogFileName(this, ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn SetLogFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogFileName(this, ::core::mem::transmute(&pszfilename)).into())
        }
        unsafe extern "system" fn LogString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows_core::PCSTR, pszlog: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogString(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszsrcname), ::core::mem::transmute(&pszlog)).into())
        }
        unsafe extern "system" fn LogDword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows_core::PCSTR, pszlogformat: ::windows_core::PCSTR, dwlog: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogDword(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszsrcname), ::core::mem::transmute(&pszlogformat), ::core::mem::transmute_copy(&dwlog)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn GetSizeParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSizeParams(this, ::core::mem::transmute_copy(&pdwmaxsize), ::core::mem::transmute_copy(&pdwshrinktosize)).into())
        }
        unsafe extern "system" fn SetSizeParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSizeParams(this, ::core::mem::transmute_copy(&dwmaxsize), ::core::mem::transmute_copy(&dwshrinktosize)).into())
        }
        IWMDMLogger_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            GetLogFileName: GetLogFileName::<Identity, Impl, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, Impl, OFFSET>,
            LogString: LogString::<Identity, Impl, OFFSET>,
            LogDword: LogDword::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetSizeParams: GetSizeParams::<Identity, Impl, OFFSET>,
            SetSizeParams: SetSizeParams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMMetaData_Impl: ::windows_core::BaseImpl {
    fn AddItem(this: &Self::This, r#type: WMDM_TAG_DATATYPE, pwsztagname: &::windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows_core::Result<()>;
    fn QueryByName(this: &Self::This, pwsztagname: &::windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::Result<()>;
    fn QueryByIndex(this: &Self::This, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::Result<()>;
    fn GetItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IWMDMMetaData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMMetaData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: ::windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwsztagname), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&ilength)).into())
        }
        unsafe extern "system" fn QueryByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztagname: ::windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryByName(this, ::core::mem::transmute(&pwsztagname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn QueryByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryByIndex(this, ::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppwszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcblength)).into())
        }
        unsafe extern "system" fn GetItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDMMetaData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            QueryByName: QueryByName::<Identity, Impl, OFFSET>,
            QueryByIndex: QueryByIndex::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMNotification_Impl: ::windows_core::BaseImpl {
    fn WMDMMessage(this: &Self::This, dwmessagetype: u32, pwszcanonicalname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WMDMMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMDMMessage(this, ::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute(&pwszcanonicalname)).into())
        }
        IWMDMNotification_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, WMDMMessage: WMDMMessage::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMObjectInfo_Impl: ::windows_core::BaseImpl {
    fn GetPlayLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetPlayLength(this: &Self::This, dwlength: u32) -> ::windows_core::Result<()>;
    fn GetPlayOffset(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetPlayOffset(this: &Self::This, dwoffset: u32) -> ::windows_core::Result<()>;
    fn GetTotalLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLastPlayPosition(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLongestPlayPosition(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IWMDMObjectInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMObjectInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPlayLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPlayLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlayLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayLength(this, ::core::mem::transmute_copy(&dwlength)).into())
        }
        unsafe extern "system" fn GetPlayOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPlayOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlayOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayOffset(this, ::core::mem::transmute_copy(&dwoffset)).into())
        }
        unsafe extern "system" fn GetTotalLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTotalLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastPlayPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLongestPlayPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlongestpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDMObjectInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPlayLength: GetPlayLength::<Identity, Impl, OFFSET>,
            SetPlayLength: SetPlayLength::<Identity, Impl, OFFSET>,
            GetPlayOffset: GetPlayOffset::<Identity, Impl, OFFSET>,
            SetPlayOffset: SetPlayOffset::<Identity, Impl, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, Impl, OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Identity, Impl, OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation_Impl: ::windows_core::BaseImpl {
    fn BeginRead(this: &Self::This) -> ::windows_core::Result<()>;
    fn BeginWrite(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetObjectName(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn SetObjectName(this: &Self::This, pwszname: &::windows_core::PCWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetObjectAttributes(this: &Self::This, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn SetObjectAttributes(this: &Self::This, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetObjectTotalSize(this: &Self::This, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()>;
    fn SetObjectTotalSize(this: &Self::This, dwsize: u32, dwsizehigh: u32) -> ::windows_core::Result<()>;
    fn TransferObjectData(this: &Self::This, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn End(this: &Self::This, phcompletioncode: *const ::windows_core::HRESULT, pnewobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IWMDMOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginRead(this).into())
        }
        unsafe extern "system" fn BeginWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginWrite(this).into())
        }
        unsafe extern "system" fn GetObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectName(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectAttributes(this, ::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectAttributes(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn GetObjectTotalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectTotalSize(this, ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&pdwsizehigh)).into())
        }
        unsafe extern "system" fn SetObjectTotalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectTotalSize(this, ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwsizehigh)).into())
        }
        unsafe extern "system" fn TransferObjectData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferObjectData(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows_core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this, ::core::mem::transmute_copy(&phcompletioncode), ::windows_core::from_raw_borrowed(&pnewobject)).into())
        }
        IWMDMOperation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginRead: BeginRead::<Identity, Impl, OFFSET>,
            BeginWrite: BeginWrite::<Identity, Impl, OFFSET>,
            GetObjectName: GetObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, Impl, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, Impl, OFFSET>,
            GetObjectTotalSize: GetObjectTotalSize::<Identity, Impl, OFFSET>,
            SetObjectTotalSize: SetObjectTotalSize::<Identity, Impl, OFFSET>,
            TransferObjectData: TransferObjectData::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMOperation2_Impl: ::windows_core::BaseImpl + IWMDMOperation_Impl {
    fn SetObjectAttributes2(this: &Self::This, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::Result<()>;
    fn GetObjectAttributes2(this: &Self::This, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IWMDMOperation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMOperation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMOperation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetObjectAttributes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectAttributes2(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pvideoformat)).into())
        }
        unsafe extern "system" fn GetObjectAttributes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectAttributes2(this, ::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into())
        }
        IWMDMOperation2_Vtbl {
            base__: <IWMDMOperation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetObjectAttributes2: SetObjectAttributes2::<Identity, Impl, OFFSET>,
            GetObjectAttributes2: GetObjectAttributes2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation3_Impl: ::windows_core::BaseImpl + IWMDMOperation_Impl {
    fn TransferObjectDataOnClearChannel(this: &Self::This, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IWMDMOperation3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMOperation);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMOperation3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransferObjectDataOnClearChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMOperation3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferObjectDataOnClearChannel(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        IWMDMOperation3_Vtbl {
            base__: <IWMDMOperation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransferObjectDataOnClearChannel: TransferObjectDataOnClearChannel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMProgress_Impl: ::windows_core::BaseImpl {
    fn Begin(this: &Self::This, dwestimatedticks: u32) -> ::windows_core::Result<()>;
    fn Progress(this: &Self::This, dwtranspiredticks: u32) -> ::windows_core::Result<()>;
    fn End(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin(this, ::core::mem::transmute_copy(&dwestimatedticks)).into())
        }
        unsafe extern "system" fn Progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Progress(this, ::core::mem::transmute_copy(&dwtranspiredticks)).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this).into())
        }
        IWMDMProgress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin: Begin::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMProgress2_Impl: ::windows_core::BaseImpl + IWMDMProgress_Impl {
    fn End2(this: &Self::This, hrcompletioncode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMProgress2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMProgress);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMProgress2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn End2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End2(this, ::core::mem::transmute_copy(&hrcompletioncode)).into())
        }
        IWMDMProgress2_Vtbl { base__: <IWMDMProgress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, End2: End2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMProgress3_Impl: ::windows_core::BaseImpl + IWMDMProgress2_Impl {
    fn Begin3(this: &Self::This, eventid: &::windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
    fn Progress3(this: &Self::This, eventid: &::windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
    fn End3(this: &Self::This, eventid: &::windows_core::GUID, hrcompletioncode: ::windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMProgress3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMProgress2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMProgress3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: ::windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin3(this, ::core::mem::transmute(&eventid), ::core::mem::transmute_copy(&dwestimatedticks), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn Progress3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: ::windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Progress3(this, ::core::mem::transmute(&eventid), ::core::mem::transmute_copy(&dwtranspiredticks), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn End3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: ::windows_core::GUID, hrcompletioncode: ::windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End3(this, ::core::mem::transmute(&eventid), ::core::mem::transmute_copy(&hrcompletioncode), ::core::mem::transmute_copy(&pcontext)).into())
        }
        IWMDMProgress3_Vtbl {
            base__: <IWMDMProgress2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin3: Begin3::<Identity, Impl, OFFSET>,
            Progress3: Progress3::<Identity, Impl, OFFSET>,
            End3: End3::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMRevoked_Impl: ::windows_core::BaseImpl {
    fn GetRevocationURL(this: &Self::This, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMRevoked {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMRevoked_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMRevoked {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRevocationURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMRevoked_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRevocationURL(this, ::core::mem::transmute_copy(&ppwszrevocationurl), ::core::mem::transmute_copy(&pdwbufferlen), ::core::mem::transmute_copy(&pdwrevokedbitflag)).into())
        }
        IWMDMRevoked_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRevocationURL: GetRevocationURL::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio\"`"]
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMStorage_Impl: ::windows_core::BaseImpl {
    fn SetAttributes(this: &Self::This, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetStorageGlobals(this: &Self::This) -> ::windows_core::Result<IWMDMStorageGlobals>;
    fn GetAttributes(this: &Self::This, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetDate(this: &Self::This) -> ::windows_core::Result<WMDMDATETIME>;
    fn GetSize(this: &Self::This, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetRights(this: &Self::This, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn EnumStorage(this: &Self::This) -> ::windows_core::Result<IWMDMEnumStorage>;
    fn SendOpaqueCommand(this: &Self::This, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows_core::Iids for IWMDMStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributes(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorageGlobals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorageglobals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributes(this, ::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into())
        }
        unsafe extern "system" fn GetDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetimeutc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)).into())
        }
        unsafe extern "system" fn GetRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRights(this, ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStorage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penumstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOpaqueCommand(this, ::core::mem::transmute_copy(&pcommand)).into())
        }
        IWMDMStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAttributes: SetAttributes::<Identity, Impl, OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDate: GetDate::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetRights: GetRights::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage2_Impl: ::windows_core::BaseImpl + IWMDMStorage_Impl {
    fn GetStorage(this: &Self::This, pszstoragename: &::windows_core::PCWSTR) -> ::windows_core::Result<IWMDMStorage>;
    fn SetAttributes2(this: &Self::This, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::Result<()>;
    fn GetAttributes2(this: &Self::This, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IWMDMStorage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMStorage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorage(this, ::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAttributes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributes2(this, ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pvideoformat)).into())
        }
        unsafe extern "system" fn GetAttributes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributes2(this, ::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into())
        }
        IWMDMStorage2_Vtbl {
            base__: <IWMDMStorage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            SetAttributes2: SetAttributes2::<Identity, Impl, OFFSET>,
            GetAttributes2: GetAttributes2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage3_Impl: ::windows_core::BaseImpl + IWMDMStorage2_Impl {
    fn GetMetadata(this: &Self::This) -> ::windows_core::Result<IWMDMMetaData>;
    fn SetMetadata(this: &Self::This, pmetadata: ::core::option::Option<&IWMDMMetaData>) -> ::windows_core::Result<()>;
    fn CreateEmptyMetadataObject(this: &Self::This) -> ::windows_core::Result<IWMDMMetaData>;
    fn SetEnumPreference(this: &Self::This, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IWMDMStorage3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMStorage2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorage3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMetadata(this, ::windows_core::from_raw_borrowed(&pmetadata)).into())
        }
        unsafe extern "system" fn CreateEmptyMetadataObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEmptyMetadataObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnumPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnumPreference(this, ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&nviews), ::core::mem::transmute_copy(&pviews)).into())
        }
        IWMDMStorage3_Vtbl {
            base__: <IWMDMStorage2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetadata: GetMetadata::<Identity, Impl, OFFSET>,
            SetMetadata: SetMetadata::<Identity, Impl, OFFSET>,
            CreateEmptyMetadataObject: CreateEmptyMetadataObject::<Identity, Impl, OFFSET>,
            SetEnumPreference: SetEnumPreference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage4_Impl: ::windows_core::BaseImpl + IWMDMStorage3_Impl {
    fn SetReferences(this: &Self::This, dwrefs: u32, ppiwmdmstorage: *const ::core::option::Option<IWMDMStorage>) -> ::windows_core::Result<()>;
    fn GetReferences(this: &Self::This, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows_core::Result<()>;
    fn GetRightsWithProgress(this: &Self::This, piprogresscallback: ::core::option::Option<&IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::Result<()>;
    fn GetSpecifiedMetadata(this: &Self::This, cproperties: u32, ppwszpropnames: *const ::windows_core::PCWSTR) -> ::windows_core::Result<IWMDMMetaData>;
    fn FindStorage(this: &Self::This, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWMDMStorage>;
    fn GetParent(this: &Self::This) -> ::windows_core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for IWMDMStorage4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMStorage3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorage4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReferences(this, ::core::mem::transmute_copy(&dwrefs), ::core::mem::transmute_copy(&ppiwmdmstorage)).into())
        }
        unsafe extern "system" fn GetReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReferences(this, ::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppiwmdmstorage)).into())
        }
        unsafe extern "system" fn GetRightsWithProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRightsWithProgress(this, ::windows_core::from_raw_borrowed(&piprogresscallback), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)).into())
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows_core::PCWSTR, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpecifiedMetadata(this, ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppwszpropnames)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindStorage(this, ::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDMStorage4_Vtbl {
            base__: <IWMDMStorage3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetReferences: SetReferences::<Identity, Impl, OFFSET>,
            GetReferences: GetReferences::<Identity, Impl, OFFSET>,
            GetRightsWithProgress: GetRightsWithProgress::<Identity, Impl, OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMStorageControl_Impl: ::windows_core::BaseImpl {
    fn Insert(this: &Self::This, fumode: u32, pwszfile: &::windows_core::PCWSTR, poperation: ::core::option::Option<&IWMDMOperation>, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<IWMDMStorage>;
    fn Delete(this: &Self::This, fumode: u32, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
    fn Rename(this: &Self::This, fumode: u32, pwsznewname: &::windows_core::PCWSTR, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, fumode: u32, pwszfile: &::windows_core::PCWSTR, pprogress: ::core::option::Option<&IWMDMProgress>, poperation: ::core::option::Option<&IWMDMOperation>) -> ::windows_core::Result<()>;
    fn Move(this: &Self::This, fumode: u32, ptargetobject: ::core::option::Option<&IWMDMStorage>, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMStorageControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorageControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Insert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows_core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Insert(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwszfile), ::windows_core::from_raw_borrowed(&poperation), ::windows_core::from_raw_borrowed(&pprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&fumode), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: ::windows_core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwsznewname), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows_core::PCWSTR, pprogress: *mut ::core::ffi::c_void, poperation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwszfile), ::windows_core::from_raw_borrowed(&pprogress), ::windows_core::from_raw_borrowed(&poperation)).into())
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute_copy(&fumode), ::windows_core::from_raw_borrowed(&ptargetobject), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        IWMDMStorageControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Insert: Insert::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMStorageControl2_Impl: ::windows_core::BaseImpl + IWMDMStorageControl_Impl {
    fn Insert2(this: &Self::This, fumode: u32, pwszfilesource: &::windows_core::PCWSTR, pwszfiledest: &::windows_core::PCWSTR, poperation: ::core::option::Option<&IWMDMOperation>, pprogress: ::core::option::Option<&IWMDMProgress>, punknown: ::core::option::Option<&::windows_core::IUnknown>, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMStorageControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMStorageControl);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorageControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Insert2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: ::windows_core::PCWSTR, pwszfiledest: ::windows_core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert2(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwszfilesource), ::core::mem::transmute(&pwszfiledest), ::windows_core::from_raw_borrowed(&poperation), ::windows_core::from_raw_borrowed(&pprogress), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&ppnewobject)).into())
        }
        IWMDMStorageControl2_Vtbl { base__: <IWMDMStorageControl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Insert2: Insert2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMStorageControl3_Impl: ::windows_core::BaseImpl + IWMDMStorageControl2_Impl {
    fn Insert3(this: &Self::This, fumode: u32, futype: u32, pwszfilesource: &::windows_core::PCWSTR, pwszfiledest: &::windows_core::PCWSTR, poperation: ::core::option::Option<&IWMDMOperation>, pprogress: ::core::option::Option<&IWMDMProgress>, pmetadata: ::core::option::Option<&IWMDMMetaData>, punknown: ::core::option::Option<&::windows_core::IUnknown>, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMStorageControl3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDMStorageControl2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorageControl3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Insert3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageControl3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: ::windows_core::PCWSTR, pwszfiledest: ::windows_core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert3(this, ::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&futype), ::core::mem::transmute(&pwszfilesource), ::core::mem::transmute(&pwszfiledest), ::windows_core::from_raw_borrowed(&poperation), ::windows_core::from_raw_borrowed(&pprogress), ::windows_core::from_raw_borrowed(&pmetadata), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&ppnewobject)).into())
        }
        IWMDMStorageControl3_Vtbl { base__: <IWMDMStorageControl2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Insert3: Insert3::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDMStorageGlobals_Impl: ::windows_core::BaseImpl {
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSerialNumber(this: &Self::This, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_core::Result<()>;
    fn GetTotalSize(this: &Self::This, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetTotalFree(this: &Self::This, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetTotalBad(this: &Self::This, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Initialize(this: &Self::This, fumode: u32, pprogress: ::core::option::Option<&IWMDMProgress>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDMStorageGlobals {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDMStorageGlobals {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerialNumber(this, ::core::mem::transmute_copy(&pserialnum), ::core::mem::transmute_copy(&abmac)).into())
        }
        unsafe extern "system" fn GetTotalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTotalSize(this, ::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)).into())
        }
        unsafe extern "system" fn GetTotalFree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTotalFree(this, ::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)).into())
        }
        unsafe extern "system" fn GetTotalBad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTotalBad(this, ::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&fumode), ::windows_core::from_raw_borrowed(&pprogress)).into())
        }
        IWMDMStorageGlobals_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetTotalSize: GetTotalSize::<Identity, Impl, OFFSET>,
            GetTotalFree: GetTotalFree::<Identity, Impl, OFFSET>,
            GetTotalBad: GetTotalBad::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDeviceManager_Impl: ::windows_core::BaseImpl {
    fn GetRevision(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDeviceCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumDevices(this: &Self::This) -> ::windows_core::Result<IWMDMEnumDevice>;
}
impl ::windows_core::Iids for IWMDeviceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDeviceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRevision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrevision, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMDeviceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRevision: GetRevision::<Identity, Impl, OFFSET>,
            GetDeviceCount: GetDeviceCount::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDeviceManager2_Impl: ::windows_core::BaseImpl + IWMDeviceManager_Impl {
    fn GetDeviceFromCanonicalName(this: &Self::This, pwszcanonicalname: &::windows_core::PCWSTR) -> ::windows_core::Result<IWMDMDevice>;
    fn EnumDevices2(this: &Self::This) -> ::windows_core::Result<IWMDMEnumDevice>;
    fn Reinitialize(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDeviceManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDeviceManager);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDeviceManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceFromCanonicalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcanonicalname: ::windows_core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceFromCanonicalName(this, ::core::mem::transmute(&pwszcanonicalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDevices2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDevices2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reinitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reinitialize(this).into())
        }
        IWMDeviceManager2_Vtbl {
            base__: <IWMDeviceManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceFromCanonicalName: GetDeviceFromCanonicalName::<Identity, Impl, OFFSET>,
            EnumDevices2: EnumDevices2::<Identity, Impl, OFFSET>,
            Reinitialize: Reinitialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWMDeviceManager3_Impl: ::windows_core::BaseImpl + IWMDeviceManager2_Impl {
    fn SetDeviceEnumPreference(this: &Self::This, dwenumpref: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMDeviceManager3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMDeviceManager2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMDeviceManager3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMDeviceManager3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceEnumPreference(this, ::core::mem::transmute_copy(&dwenumpref)).into())
        }
        IWMDeviceManager3_Vtbl {
            base__: <IWMDeviceManager2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
