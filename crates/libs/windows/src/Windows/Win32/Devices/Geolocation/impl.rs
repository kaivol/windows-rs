#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICivicAddressReport_Impl: ::windows_core::BaseImpl + ILocationReport_Impl {
    fn GetAddressLine1(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAddressLine2(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCity(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetStateProvince(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPostalCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCountryRegion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDetailLevel(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ICivicAddressReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ILocationReport);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICivicAddressReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAddressLine1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstraddress1: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAddressLine1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraddress1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAddressLine2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstraddress2: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAddressLine2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraddress2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStateProvince<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstateprovince: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStateProvince(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstateprovince, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPostalCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpostalcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPostalCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpostalcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCountryRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcountryregion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCountryRegion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcountryregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDetailLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDetailLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdetaillevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICivicAddressReport_Vtbl {
            base__: <ILocationReport as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAddressLine1: GetAddressLine1::<Identity, Impl, OFFSET>,
            GetAddressLine2: GetAddressLine2::<Identity, Impl, OFFSET>,
            GetCity: GetCity::<Identity, Impl, OFFSET>,
            GetStateProvince: GetStateProvince::<Identity, Impl, OFFSET>,
            GetPostalCode: GetPostalCode::<Identity, Impl, OFFSET>,
            GetCountryRegion: GetCountryRegion::<Identity, Impl, OFFSET>,
            GetDetailLevel: GetDetailLevel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICivicAddressReportFactory_Impl: ::windows_core::BaseImpl + ILocationReportFactory_Impl {
    fn CivicAddressReport(this: &Self::This) -> ::windows_core::Result<IDispCivicAddressReport>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICivicAddressReportFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ILocationReportFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReportFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICivicAddressReportFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CivicAddressReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICivicAddressReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CivicAddressReport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICivicAddressReportFactory_Vtbl {
            base__: <ILocationReportFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CivicAddressReport: CivicAddressReport::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDefaultLocation_Impl: ::windows_core::BaseImpl {
    fn SetReport(this: &Self::This, reporttype: *const ::windows_core::GUID, plocationreport: ::core::option::Option<&ILocationReport>) -> ::windows_core::Result<()>;
    fn GetReport(this: &Self::This, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<ILocationReport>;
}
impl ::windows_core::Iids for IDefaultLocation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDefaultLocation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, plocationreport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReport(this, ::core::mem::transmute_copy(&reporttype), ::windows_core::from_raw_borrowed(&plocationreport)).into())
        }
        unsafe extern "system" fn GetReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pplocationreport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReport(this, ::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplocationreport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDefaultLocation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetReport: SetReport::<Identity, Impl, OFFSET>,
            GetReport: GetReport::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDispCivicAddressReport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AddressLine1(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddressLine2(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn City(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn StateProvince(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PostalCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CountryRegion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DetailLevel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDispCivicAddressReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispCivicAddressReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddressLine1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress1: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddressLine1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddress1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddressLine2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress2: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddressLine2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddress2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn City<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::City(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StateProvince<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstateprovince: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StateProvince(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstateprovince, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PostalCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppostalcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppostalcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CountryRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcountryregion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CountryRegion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcountryregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetailLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetailLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdetaillevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDispCivicAddressReport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddressLine1: AddressLine1::<Identity, Impl, OFFSET>,
            AddressLine2: AddressLine2::<Identity, Impl, OFFSET>,
            City: City::<Identity, Impl, OFFSET>,
            StateProvince: StateProvince::<Identity, Impl, OFFSET>,
            PostalCode: PostalCode::<Identity, Impl, OFFSET>,
            CountryRegion: CountryRegion::<Identity, Impl, OFFSET>,
            DetailLevel: DetailLevel::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDispLatLongReport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Latitude(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Longitude(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ErrorRadius(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Altitude(this: &Self::This) -> ::windows_core::Result<f64>;
    fn AltitudeError(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDispLatLongReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispLatLongReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Latitude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Latitude(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Longitude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Longitude(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ErrorRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorRadius(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Altitude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Altitude(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AltitudeError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AltitudeError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDispLatLongReport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Latitude: Latitude::<Identity, Impl, OFFSET>,
            Longitude: Longitude::<Identity, Impl, OFFSET>,
            ErrorRadius: ErrorRadius::<Identity, Impl, OFFSET>,
            Altitude: Altitude::<Identity, Impl, OFFSET>,
            AltitudeError: AltitudeError::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ILatLongReport_Impl: ::windows_core::BaseImpl + ILocationReport_Impl {
    fn GetLatitude(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetLongitude(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetErrorRadius(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetAltitude(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetAltitudeError(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ILatLongReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ILocationReport);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILatLongReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLatitude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, platitude: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatitude(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(platitude, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLongitude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plongitude: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLongitude(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plongitude, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorradius: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorRadius(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrorradius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAltitude<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paltitude: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAltitude(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paltitude, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAltitudeError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paltitudeerror: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAltitudeError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paltitudeerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILatLongReport_Vtbl {
            base__: <ILocationReport as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLatitude: GetLatitude::<Identity, Impl, OFFSET>,
            GetLongitude: GetLongitude::<Identity, Impl, OFFSET>,
            GetErrorRadius: GetErrorRadius::<Identity, Impl, OFFSET>,
            GetAltitude: GetAltitude::<Identity, Impl, OFFSET>,
            GetAltitudeError: GetAltitudeError::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILatLongReportFactory_Impl: ::windows_core::BaseImpl + ILocationReportFactory_Impl {
    fn LatLongReport(this: &Self::This) -> ::windows_core::Result<IDispLatLongReport>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ILatLongReportFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ILocationReportFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReportFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILatLongReportFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LatLongReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILatLongReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LatLongReport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILatLongReportFactory_Vtbl { base__: <ILocationReportFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LatLongReport: LatLongReport::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`"]
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
pub trait ILocation_Impl: ::windows_core::BaseImpl {
    fn RegisterForReport(this: &Self::This, pevents: ::core::option::Option<&ILocationEvents>, reporttype: *const ::windows_core::GUID, dwrequestedreportinterval: u32) -> ::windows_core::Result<()>;
    fn UnregisterForReport(this: &Self::This, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetReport(this: &Self::This, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<ILocationReport>;
    fn GetReportStatus(this: &Self::This, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<LOCATION_REPORT_STATUS>;
    fn GetReportInterval(this: &Self::This, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn SetReportInterval(this: &Self::This, reporttype: *const ::windows_core::GUID, millisecondsrequested: u32) -> ::windows_core::Result<()>;
    fn GetDesiredAccuracy(this: &Self::This, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY>;
    fn SetDesiredAccuracy(this: &Self::This, reporttype: *const ::windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows_core::Result<()>;
    fn RequestPermissions(this: &Self::This, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows_core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
impl ::windows_core::Iids for ILocation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILocation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterForReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, dwrequestedreportinterval: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterForReport(this, ::windows_core::from_raw_borrowed(&pevents), ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&dwrequestedreportinterval)).into())
        }
        unsafe extern "system" fn UnregisterForReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterForReport(this, ::core::mem::transmute_copy(&reporttype)).into())
        }
        unsafe extern "system" fn GetReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pplocationreport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReport(this, ::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplocationreport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReportStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReportStatus(this, ::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReportInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pmilliseconds: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReportInterval(this, ::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmilliseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReportInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, millisecondsrequested: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReportInterval(this, ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&millisecondsrequested)).into())
        }
        unsafe extern "system" fn GetDesiredAccuracy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDesiredAccuracy(this, ::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesiredaccuracy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredAccuracy(this, ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&desiredaccuracy)).into())
        }
        unsafe extern "system" fn RequestPermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows_core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestPermissions(this, ::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&preporttypes), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&fmodal)).into())
        }
        ILocation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterForReport: RegisterForReport::<Identity, Impl, OFFSET>,
            UnregisterForReport: UnregisterForReport::<Identity, Impl, OFFSET>,
            GetReport: GetReport::<Identity, Impl, OFFSET>,
            GetReportStatus: GetReportStatus::<Identity, Impl, OFFSET>,
            GetReportInterval: GetReportInterval::<Identity, Impl, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, Impl, OFFSET>,
            GetDesiredAccuracy: GetDesiredAccuracy::<Identity, Impl, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILocationEvents_Impl: ::windows_core::BaseImpl {
    fn OnLocationChanged(this: &Self::This, reporttype: *const ::windows_core::GUID, plocationreport: ::core::option::Option<&ILocationReport>) -> ::windows_core::Result<()>;
    fn OnStatusChanged(this: &Self::This, reporttype: *const ::windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILocationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILocationEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLocationChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, plocationreport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLocationChanged(this, ::core::mem::transmute_copy(&reporttype), ::windows_core::from_raw_borrowed(&plocationreport)).into())
        }
        unsafe extern "system" fn OnStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatusChanged(this, ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&newstatus)).into())
        }
        ILocationEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLocationChanged: OnLocationChanged::<Identity, Impl, OFFSET>,
            OnStatusChanged: OnStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILocationPower_Impl: ::windows_core::BaseImpl {
    fn Connect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILocationPower {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILocationPower {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        ILocationPower_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ILocationReport_Impl: ::windows_core::BaseImpl {
    fn GetSensorID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetTimestamp(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetValue(this: &Self::This, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ILocationReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILocationReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSensorID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSensorID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psensorid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcreationtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILocationReport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSensorID: GetSensorID::<Identity, Impl, OFFSET>,
            GetTimestamp: GetTimestamp::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILocationReportFactory_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ListenForReports(this: &Self::This, requestedreportinterval: u32) -> ::windows_core::Result<()>;
    fn StopListeningForReports(this: &Self::This) -> ::windows_core::Result<()>;
    fn Status(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ReportInterval(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetReportInterval(this: &Self::This, millisecondsrequested: u32) -> ::windows_core::Result<()>;
    fn DesiredAccuracy(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetDesiredAccuracy(this: &Self::This, desiredaccuracy: u32) -> ::windows_core::Result<()>;
    fn RequestPermissions(this: &Self::This, hwnd: *const u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ILocationReportFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILocationReportFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ListenForReports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ListenForReports(this, ::core::mem::transmute_copy(&requestedreportinterval)).into())
        }
        unsafe extern "system" fn StopListeningForReports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopListeningForReports(this).into())
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReportInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmilliseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReportInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReportInterval(this, ::core::mem::transmute_copy(&millisecondsrequested)).into())
        }
        unsafe extern "system" fn DesiredAccuracy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredAccuracy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesiredaccuracy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredAccuracy(this, ::core::mem::transmute_copy(&desiredaccuracy)).into())
        }
        unsafe extern "system" fn RequestPermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestPermissions(this, ::core::mem::transmute_copy(&hwnd)).into())
        }
        ILocationReportFactory_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ListenForReports: ListenForReports::<Identity, Impl, OFFSET>,
            StopListeningForReports: StopListeningForReports::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ReportInterval: ReportInterval::<Identity, Impl, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, Impl, OFFSET>,
            DesiredAccuracy: DesiredAccuracy::<Identity, Impl, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ICivicAddressReportFactoryEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _ICivicAddressReportFactoryEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _ICivicAddressReportFactoryEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _ICivicAddressReportFactoryEvents {
    const VTABLE: Self::Vtable = { _ICivicAddressReportFactoryEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ILatLongReportFactoryEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _ILatLongReportFactoryEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _ILatLongReportFactoryEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _ILatLongReportFactoryEvents {
    const VTABLE: Self::Vtable = { _ILatLongReportFactoryEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
