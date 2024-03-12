#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceModelPlugIn_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, bstrxml: &::windows_core::BSTR, cnummodels: u32, imodelposition: u32) -> ::windows_core::Result<()>;
    fn GetNumChannels(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DeviceToColorimetricColors(this: &Self::This, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows_core::Result<()>;
    fn ColorimetricToDeviceColors(this: &Self::This, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF) -> ::windows_core::Result<f32>;
    fn ColorimetricToDeviceColorsWithBlack(this: &Self::This, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> ::windows_core::Result<f32>;
    fn SetTransformDeviceModelInfo(this: &Self::This, imodelposition: u32, pidevicemodelother: ::core::option::Option<&IDeviceModelPlugIn>) -> ::windows_core::Result<()>;
    fn GetPrimarySamples(this: &Self::This, pprimarycolor: *mut PrimaryXYZColors) -> ::windows_core::Result<()>;
    fn GetGamutBoundaryMeshSize(this: &Self::This, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows_core::Result<()>;
    fn GetGamutBoundaryMesh(this: &Self::This, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows_core::Result<()>;
    fn GetNeutralAxisSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNeutralAxis(this: &Self::This, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDeviceModelPlugIn {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceModelPlugIn {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows_core::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&bstrxml), ::core::mem::transmute_copy(&cnummodels), ::core::mem::transmute_copy(&imodelposition)).into())
        }
        unsafe extern "system" fn GetNumChannels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumChannels(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumchannels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceToColorimetricColors(this, ::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pdevicevalues), ::core::mem::transmute_copy(&pxyzcolors)).into())
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColorimetricToDeviceColors(this, ::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pxyzcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevicevalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColorimetricToDeviceColorsWithBlack(this, ::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pxyzcolors), ::core::mem::transmute_copy(&pblackinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevicevalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformDeviceModelInfo(this, ::core::mem::transmute_copy(&imodelposition), ::windows_core::from_raw_borrowed(&pidevicemodelother)).into())
        }
        unsafe extern "system" fn GetPrimarySamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrimarySamples(this, ::core::mem::transmute_copy(&pprimarycolor)).into())
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGamutBoundaryMeshSize(this, ::core::mem::transmute_copy(&pnumvertices), ::core::mem::transmute_copy(&pnumtriangles)).into())
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGamutBoundaryMesh(this, ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&cvertices), ::core::mem::transmute_copy(&ctriangles), ::core::mem::transmute_copy(&pvertices), ::core::mem::transmute_copy(&ptriangles)).into())
        }
        unsafe extern "system" fn GetNeutralAxisSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNeutralAxisSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccolors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNeutralAxis<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNeutralAxis(this, ::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&pxyzcolors)).into())
        }
        IDeviceModelPlugIn_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetNumChannels: GetNumChannels::<Identity, Impl, OFFSET>,
            DeviceToColorimetricColors: DeviceToColorimetricColors::<Identity, Impl, OFFSET>,
            ColorimetricToDeviceColors: ColorimetricToDeviceColors::<Identity, Impl, OFFSET>,
            ColorimetricToDeviceColorsWithBlack: ColorimetricToDeviceColorsWithBlack::<Identity, Impl, OFFSET>,
            SetTransformDeviceModelInfo: SetTransformDeviceModelInfo::<Identity, Impl, OFFSET>,
            GetPrimarySamples: GetPrimarySamples::<Identity, Impl, OFFSET>,
            GetGamutBoundaryMeshSize: GetGamutBoundaryMeshSize::<Identity, Impl, OFFSET>,
            GetGamutBoundaryMesh: GetGamutBoundaryMesh::<Identity, Impl, OFFSET>,
            GetNeutralAxisSize: GetNeutralAxisSize::<Identity, Impl, OFFSET>,
            GetNeutralAxis: GetNeutralAxis::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGamutMapModelPlugIn_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, bstrxml: &::windows_core::BSTR, psrcplugin: ::core::option::Option<&IDeviceModelPlugIn>, pdestplugin: ::core::option::Option<&IDeviceModelPlugIn>, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows_core::Result<()>;
    fn SourceToDestinationAppearanceColors(this: &Self::This, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGamutMapModelPlugIn {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGamutMapModelPlugIn {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows_core::BSTR>, psrcplugin: *mut ::core::ffi::c_void, pdestplugin: *mut ::core::ffi::c_void, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&bstrxml), ::windows_core::from_raw_borrowed(&psrcplugin), ::windows_core::from_raw_borrowed(&pdestplugin), ::core::mem::transmute_copy(&psrcgbd), ::core::mem::transmute_copy(&pdestgbd)).into())
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SourceToDestinationAppearanceColors(this, ::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&pinputcolors), ::core::mem::transmute_copy(&poutputcolors)).into())
        }
        IGamutMapModelPlugIn_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SourceToDestinationAppearanceColors: SourceToDestinationAppearanceColors::<Identity, Impl, OFFSET>,
        }
    };
}
