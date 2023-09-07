#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSAT_Impl: ::windows_core::BaseImpl + super::super::UI::Accessibility::IAccessible_Impl {
    fn SetAccessiblityData(this: &Self::This, wsname: &::windows_core::PCWSTR, wsvalue: &::windows_core::PCWSTR, wsdesc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
impl ::windows_core::Iids for IAccessibleWinSAT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::UI::Accessibility::IAccessible);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWinSAT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessibleWinSAT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAccessiblityData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWinSAT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsname: ::windows_core::PCWSTR, wsvalue: ::windows_core::PCWSTR, wsdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccessiblityData(this, ::core::mem::transmute(&wsname), ::core::mem::transmute(&wsvalue), ::core::mem::transmute(&wsdesc)).into())
        }
        IAccessibleWinSAT_Vtbl {
            base__: <super::super::UI::Accessibility::IAccessible as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAccessiblityData: SetAccessiblityData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInitiateWinSATAssessment_Impl: ::windows_core::BaseImpl {
    fn InitiateAssessment(this: &Self::This, cmdline: &::windows_core::PCWSTR, pcallbacks: ::core::option::Option<&IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn InitiateFormalAssessment(this: &Self::This, pcallbacks: ::core::option::Option<&IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn CancelAssessment(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInitiateWinSATAssessment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInitiateWinSATAssessment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitiateAssessment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmdline: ::windows_core::PCWSTR, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitiateAssessment(this, ::core::mem::transmute(&cmdline), ::windows_core::from_raw_borrowed(&pcallbacks), ::core::mem::transmute_copy(&callerhwnd)).into())
        }
        unsafe extern "system" fn InitiateFormalAssessment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitiateFormalAssessment(this, ::windows_core::from_raw_borrowed(&pcallbacks), ::core::mem::transmute_copy(&callerhwnd)).into())
        }
        unsafe extern "system" fn CancelAssessment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAssessment(this).into())
        }
        IInitiateWinSATAssessment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitiateAssessment: InitiateAssessment::<Identity, Impl, OFFSET>,
            InitiateFormalAssessment: InitiateFormalAssessment::<Identity, Impl, OFFSET>,
            CancelAssessment: CancelAssessment::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProvideWinSATAssessmentInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Score(this: &Self::This) -> ::windows_core::Result<f32>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IProvideWinSATAssessmentInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideWinSATAssessmentInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Score<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Score(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(score, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideWinSATAssessmentInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Score: Score::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProvideWinSATResultsInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetAssessmentInfo(this: &Self::This, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows_core::Result<IProvideWinSATAssessmentInfo>;
    fn AssessmentState(this: &Self::This) -> ::windows_core::Result<WINSAT_ASSESSMENT_STATE>;
    fn AssessmentDateTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SystemRating(this: &Self::This) -> ::windows_core::Result<f32>;
    fn RatingStateDesc(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IProvideWinSATResultsInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideWinSATResultsInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAssessmentInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAssessmentInfo(this, ::core::mem::transmute_copy(&assessment)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AssessmentState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssessmentState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AssessmentDateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssessmentDateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SystemRating<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SystemRating(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(level, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RatingStateDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RatingStateDesc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideWinSATResultsInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAssessmentInfo: GetAssessmentInfo::<Identity, Impl, OFFSET>,
            AssessmentState: AssessmentState::<Identity, Impl, OFFSET>,
            AssessmentDateTime: AssessmentDateTime::<Identity, Impl, OFFSET>,
            SystemRating: SystemRating::<Identity, Impl, OFFSET>,
            RatingStateDesc: RatingStateDesc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IProvideWinSATVisuals_Impl: ::windows_core::BaseImpl {
    fn get_Bitmap(this: &Self::This, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IProvideWinSATVisuals {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATVisuals_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideWinSATVisuals {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Bitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideWinSATVisuals_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Bitmap(this, ::core::mem::transmute_copy(&bitmapsize), ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&rating)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideWinSATVisuals_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, get_Bitmap: get_Bitmap::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IQueryAllWinSATAssessments_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_AllXML(this: &Self::This, xpath: &::windows_core::BSTR, namespaces: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IQueryAllWinSATAssessments {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryAllWinSATAssessments_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQueryAllWinSATAssessments {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_AllXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryAllWinSATAssessments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaces: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AllXML(this, ::core::mem::transmute(&xpath), ::core::mem::transmute(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdomnodelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IQueryAllWinSATAssessments_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, get_AllXML: get_AllXML::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IQueryOEMWinSATCustomization_Impl: ::windows_core::BaseImpl {
    fn GetOEMPrePopulationInfo(this: &Self::This) -> ::windows_core::Result<WINSAT_OEM_CUSTOMIZATION_STATE>;
}
impl ::windows_core::Iids for IQueryOEMWinSATCustomization {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryOEMWinSATCustomization_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQueryOEMWinSATCustomization {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOEMPrePopulationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryOEMWinSATCustomization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_CUSTOMIZATION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOEMPrePopulationInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IQueryOEMWinSATCustomization_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOEMPrePopulationInfo: GetOEMPrePopulationInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IQueryRecentWinSATAssessment_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_XML(this: &Self::This, xpath: &::windows_core::BSTR, namespaces: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
    fn Info(this: &Self::This) -> ::windows_core::Result<IProvideWinSATResultsInfo>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IQueryRecentWinSATAssessment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQueryRecentWinSATAssessment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_XML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaces: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_XML(this, ::core::mem::transmute(&xpath), ::core::mem::transmute(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdomnodelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Info(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwinsatassessmentinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IQueryRecentWinSATAssessment_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_XML: get_XML::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinSATInitiateEvents_Impl: ::windows_core::BaseImpl {
    fn WinSATComplete(this: &Self::This, hresult: ::windows_core::HRESULT, strdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WinSATUpdate(this: &Self::This, ucurrenttick: u32, uticktotal: u32, strcurrentstate: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinSATInitiateEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinSATInitiateEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WinSATComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, strdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WinSATComplete(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&strdescription)).into())
        }
        unsafe extern "system" fn WinSATUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WinSATUpdate(this, ::core::mem::transmute_copy(&ucurrenttick), ::core::mem::transmute_copy(&uticktotal), ::core::mem::transmute(&strcurrentstate)).into())
        }
        IWinSATInitiateEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WinSATComplete: WinSATComplete::<Identity, Impl, OFFSET>,
            WinSATUpdate: WinSATUpdate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
