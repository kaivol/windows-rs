pub trait IEnumWbemClassObject_Impl: ::windows_core::BaseImpl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, ltimeout: i32, ucount: u32, apobjects: *mut ::core::option::Option<IWbemClassObject>, pureturned: *mut u32) -> ::windows_core::HRESULT;
    fn NextAsync(this: &Self::This, ucount: u32, psink: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::HRESULT;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWbemClassObject>;
    fn Skip(this: &Self::This, ltimeout: i32, ncount: u32) -> ::windows_core::HRESULT;
}
impl ::windows_core::Iids for IEnumWbemClassObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWbemClassObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&ucount), ::core::mem::transmute_copy(&apobjects), ::core::mem::transmute_copy(&pureturned)))
        }
        unsafe extern "system" fn NextAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ucount: u32, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextAsync(this, ::core::mem::transmute_copy(&ucount), ::windows_core::from_raw_borrowed(&psink)))
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&ncount)))
        }
        IEnumWbemClassObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            NextAsync: NextAsync::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMofCompiler_Impl: ::windows_core::BaseImpl {
    fn CompileFile(this: &Self::This, filename: &::windows_core::PCWSTR, serverandnamespace: &::windows_core::PCWSTR, user: &::windows_core::PCWSTR, authority: &::windows_core::PCWSTR, password: &::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::Result<()>;
    fn CompileBuffer(this: &Self::This, buffsize: i32, pbuffer: *const u8, serverandnamespace: &::windows_core::PCWSTR, user: &::windows_core::PCWSTR, authority: &::windows_core::PCWSTR, password: &::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::Result<()>;
    fn CreateBMOF(this: &Self::This, textfilename: &::windows_core::PCWSTR, bmoffilename: &::windows_core::PCWSTR, serverandnamespace: &::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMofCompiler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMofCompiler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompileFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, serverandnamespace: ::windows_core::PCWSTR, user: ::windows_core::PCWSTR, authority: ::windows_core::PCWSTR, password: ::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompileFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute(&serverandnamespace), ::core::mem::transmute(&user), ::core::mem::transmute(&authority), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn CompileBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: ::windows_core::PCWSTR, user: ::windows_core::PCWSTR, authority: ::windows_core::PCWSTR, password: ::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompileBuffer(this, ::core::mem::transmute_copy(&buffsize), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute(&serverandnamespace), ::core::mem::transmute(&user), ::core::mem::transmute(&authority), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn CreateBMOF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textfilename: ::windows_core::PCWSTR, bmoffilename: ::windows_core::PCWSTR, serverandnamespace: ::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBMOF(this, ::core::mem::transmute(&textfilename), ::core::mem::transmute(&bmoffilename), ::core::mem::transmute(&serverandnamespace), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into())
        }
        IMofCompiler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CompileFile: CompileFile::<Identity, Impl, OFFSET>,
            CompileBuffer: CompileBuffer::<Identity, Impl, OFFSET>,
            CreateBMOF: CreateBMOF::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemDateTime_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetValue(this: &Self::This, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Year(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetYear(this: &Self::This, iyear: i32) -> ::windows_core::Result<()>;
    fn YearSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetYearSpecified(this: &Self::This, byearspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Month(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMonth(this: &Self::This, imonth: i32) -> ::windows_core::Result<()>;
    fn MonthSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMonthSpecified(this: &Self::This, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Day(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDay(this: &Self::This, iday: i32) -> ::windows_core::Result<()>;
    fn DaySpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDaySpecified(this: &Self::This, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Hours(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHours(this: &Self::This, ihours: i32) -> ::windows_core::Result<()>;
    fn HoursSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHoursSpecified(this: &Self::This, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Minutes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMinutes(this: &Self::This, iminutes: i32) -> ::windows_core::Result<()>;
    fn MinutesSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMinutesSpecified(this: &Self::This, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Seconds(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSeconds(this: &Self::This, iseconds: i32) -> ::windows_core::Result<()>;
    fn SecondsSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSecondsSpecified(this: &Self::This, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Microseconds(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMicroseconds(this: &Self::This, imicroseconds: i32) -> ::windows_core::Result<()>;
    fn MicrosecondsSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMicrosecondsSpecified(this: &Self::This, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UTC(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetUTC(this: &Self::This, iutc: i32) -> ::windows_core::Result<()>;
    fn UTCSpecified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUTCSpecified(this: &Self::This, butcspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsInterval(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsInterval(this: &Self::This, bisinterval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetVarDate(this: &Self::This, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<f64>;
    fn SetVarDate(this: &Self::This, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetFileTime(this: &Self::This, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFileTime(this: &Self::This, strfiletime: &::windows_core::BSTR, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemDateTime {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemDateTime {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&strvalue)).into())
        }
        unsafe extern "system" fn Year<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Year(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iyear, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetYear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetYear(this, ::core::mem::transmute_copy(&iyear)).into())
        }
        unsafe extern "system" fn YearSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, byearspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::YearSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(byearspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetYearSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, byearspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetYearSpecified(this, ::core::mem::transmute_copy(&byearspecified)).into())
        }
        unsafe extern "system" fn Month<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Month(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imonth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMonth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMonth(this, ::core::mem::transmute_copy(&imonth)).into())
        }
        unsafe extern "system" fn MonthSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmonthspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MonthSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bmonthspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMonthSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMonthSpecified(this, ::core::mem::transmute_copy(&bmonthspecified)).into())
        }
        unsafe extern "system" fn Day<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Day(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iday, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDay(this, ::core::mem::transmute_copy(&iday)).into())
        }
        unsafe extern "system" fn DaySpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bdayspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DaySpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bdayspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDaySpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaySpecified(this, ::core::mem::transmute_copy(&bdayspecified)).into())
        }
        unsafe extern "system" fn Hours<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hours(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ihours, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHours<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHours(this, ::core::mem::transmute_copy(&ihours)).into())
        }
        unsafe extern "system" fn HoursSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bhoursspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HoursSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bhoursspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHoursSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHoursSpecified(this, ::core::mem::transmute_copy(&bhoursspecified)).into())
        }
        unsafe extern "system" fn Minutes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Minutes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iminutes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinutes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinutes(this, ::core::mem::transmute_copy(&iminutes)).into())
        }
        unsafe extern "system" fn MinutesSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bminutesspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinutesSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bminutesspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinutesSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinutesSpecified(this, ::core::mem::transmute_copy(&bminutesspecified)).into())
        }
        unsafe extern "system" fn Seconds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Seconds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSeconds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeconds(this, ::core::mem::transmute_copy(&iseconds)).into())
        }
        unsafe extern "system" fn SecondsSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecondsSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bsecondsspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecondsSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecondsSpecified(this, ::core::mem::transmute_copy(&bsecondsspecified)).into())
        }
        unsafe extern "system" fn Microseconds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Microseconds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imicroseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMicroseconds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMicroseconds(this, ::core::mem::transmute_copy(&imicroseconds)).into())
        }
        unsafe extern "system" fn MicrosecondsSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MicrosecondsSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bmicrosecondsspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMicrosecondsSpecified(this, ::core::mem::transmute_copy(&bmicrosecondsspecified)).into())
        }
        unsafe extern "system" fn UTC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UTC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iutc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUTC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUTC(this, ::core::mem::transmute_copy(&iutc)).into())
        }
        unsafe extern "system" fn UTCSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, butcspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UTCSpecified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(butcspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUTCSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, butcspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUTCSpecified(this, ::core::mem::transmute_copy(&butcspecified)).into())
        }
        unsafe extern "system" fn IsInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisinterval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisinterval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisinterval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsInterval(this, ::core::mem::transmute_copy(&bisinterval)).into())
        }
        unsafe extern "system" fn GetVarDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, dvardate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVarDate(this, ::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dvardate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVarDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVarDate(this, ::core::mem::transmute_copy(&dvardate), ::core::mem::transmute_copy(&bislocal)).into())
        }
        unsafe extern "system" fn GetFileTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, strfiletime: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileTime(this, ::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strfiletime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFileTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strfiletime: ::std::mem::MaybeUninit<::windows_core::BSTR>, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileTime(this, ::core::mem::transmute(&strfiletime), ::core::mem::transmute_copy(&bislocal)).into())
        }
        ISWbemDateTime_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Year: Year::<Identity, Impl, OFFSET>,
            SetYear: SetYear::<Identity, Impl, OFFSET>,
            YearSpecified: YearSpecified::<Identity, Impl, OFFSET>,
            SetYearSpecified: SetYearSpecified::<Identity, Impl, OFFSET>,
            Month: Month::<Identity, Impl, OFFSET>,
            SetMonth: SetMonth::<Identity, Impl, OFFSET>,
            MonthSpecified: MonthSpecified::<Identity, Impl, OFFSET>,
            SetMonthSpecified: SetMonthSpecified::<Identity, Impl, OFFSET>,
            Day: Day::<Identity, Impl, OFFSET>,
            SetDay: SetDay::<Identity, Impl, OFFSET>,
            DaySpecified: DaySpecified::<Identity, Impl, OFFSET>,
            SetDaySpecified: SetDaySpecified::<Identity, Impl, OFFSET>,
            Hours: Hours::<Identity, Impl, OFFSET>,
            SetHours: SetHours::<Identity, Impl, OFFSET>,
            HoursSpecified: HoursSpecified::<Identity, Impl, OFFSET>,
            SetHoursSpecified: SetHoursSpecified::<Identity, Impl, OFFSET>,
            Minutes: Minutes::<Identity, Impl, OFFSET>,
            SetMinutes: SetMinutes::<Identity, Impl, OFFSET>,
            MinutesSpecified: MinutesSpecified::<Identity, Impl, OFFSET>,
            SetMinutesSpecified: SetMinutesSpecified::<Identity, Impl, OFFSET>,
            Seconds: Seconds::<Identity, Impl, OFFSET>,
            SetSeconds: SetSeconds::<Identity, Impl, OFFSET>,
            SecondsSpecified: SecondsSpecified::<Identity, Impl, OFFSET>,
            SetSecondsSpecified: SetSecondsSpecified::<Identity, Impl, OFFSET>,
            Microseconds: Microseconds::<Identity, Impl, OFFSET>,
            SetMicroseconds: SetMicroseconds::<Identity, Impl, OFFSET>,
            MicrosecondsSpecified: MicrosecondsSpecified::<Identity, Impl, OFFSET>,
            SetMicrosecondsSpecified: SetMicrosecondsSpecified::<Identity, Impl, OFFSET>,
            UTC: UTC::<Identity, Impl, OFFSET>,
            SetUTC: SetUTC::<Identity, Impl, OFFSET>,
            UTCSpecified: UTCSpecified::<Identity, Impl, OFFSET>,
            SetUTCSpecified: SetUTCSpecified::<Identity, Impl, OFFSET>,
            IsInterval: IsInterval::<Identity, Impl, OFFSET>,
            SetIsInterval: SetIsInterval::<Identity, Impl, OFFSET>,
            GetVarDate: GetVarDate::<Identity, Impl, OFFSET>,
            SetVarDate: SetVarDate::<Identity, Impl, OFFSET>,
            GetFileTime: GetFileTime::<Identity, Impl, OFFSET>,
            SetFileTime: SetFileTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemEventSource_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn NextEvent(this: &Self::This, itimeoutms: i32) -> ::windows_core::Result<ISWbemObject>;
    fn Security_(this: &Self::This) -> ::windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemEventSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemEventSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NextEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextEvent(this, ::core::mem::transmute_copy(&itimeoutms)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemEventSource_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NextEvent: NextEvent::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemLastError_Impl: ::windows_core::BaseImpl + ISWbemObject_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemLastError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISWbemObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemLastError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemLastError {
    const VTABLE: Self::Vtable = { ISWbemLastError_Vtbl { base__: <ISWbemObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemLocator_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ConnectServer(this: &Self::This, strserver: &::windows_core::BSTR, strnamespace: &::windows_core::BSTR, struser: &::windows_core::BSTR, strpassword: &::windows_core::BSTR, strlocale: &::windows_core::BSTR, strauthority: &::windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemServices>;
    fn Security_(this: &Self::This) -> ::windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, strnamespace: ::std::mem::MaybeUninit<::windows_core::BSTR>, struser: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, strauthority: ::std::mem::MaybeUninit<::windows_core::BSTR>, isecurityflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectServer(this, ::core::mem::transmute(&strserver), ::core::mem::transmute(&strnamespace), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute(&strauthority), ::core::mem::transmute_copy(&isecurityflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemLocator_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectServer: ConnectServer::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemMethod_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Origin(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InParameters(this: &Self::This) -> ::windows_core::Result<ISWbemObject>;
    fn OutParameters(this: &Self::This) -> ::windows_core::Result<ISWbemObject>;
    fn Qualifiers_(this: &Self::This) -> ::windows_core::Result<ISWbemQualifierSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemMethod {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemMethod {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Origin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strorigin: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Origin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strorigin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbeminparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OutParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemoutparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Qualifiers_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Qualifiers_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifierset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemMethod_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Origin: Origin::<Identity, Impl, OFFSET>,
            InParameters: InParameters::<Identity, Impl, OFFSET>,
            OutParameters: OutParameters::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemMethodSet_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, strname: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<ISWbemMethod>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemMethodSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemMethodSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemmethod: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemMethodSet_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemNamedValue_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, varvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemNamedValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemNamedValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&varvalue)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemNamedValue_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemNamedValueSet_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, strname: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<ISWbemNamedValue>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, strname: &::windows_core::BSTR, varvalue: *const super::Variant::VARIANT, iflags: i32) -> ::windows_core::Result<ISWbemNamedValue>;
    fn Remove(this: &Self::This, strname: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemNamedValueSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemNamedValueSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: *const super::Variant::VARIANT, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&varvalue), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalueset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAll(this).into())
        }
        ISWbemNamedValueSet_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObject_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Put_(this: &Self::This, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectPath>;
    fn PutAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Delete_(this: &Self::This, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn DeleteAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Instances_(this: &Self::This, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Subclasses_(this: &Self::This, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Associators_(this: &Self::This, strassocclass: &::windows_core::BSTR, strresultclass: &::windows_core::BSTR, strresultrole: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &::windows_core::BSTR, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strassocclass: &::windows_core::BSTR, strresultclass: &::windows_core::BSTR, strresultrole: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &::windows_core::BSTR, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn References_(this: &Self::This, strresultclass: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strresultclass: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn ExecMethod_(this: &Self::This, strmethodname: &::windows_core::BSTR, objwbeminparameters: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync_(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strmethodname: &::windows_core::BSTR, objwbeminparameters: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Clone_(this: &Self::This) -> ::windows_core::Result<ISWbemObject>;
    fn GetObjectText_(this: &Self::This, iflags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SpawnDerivedClass_(this: &Self::This, iflags: i32) -> ::windows_core::Result<ISWbemObject>;
    fn SpawnInstance_(this: &Self::This, iflags: i32) -> ::windows_core::Result<ISWbemObject>;
    fn CompareTo_(this: &Self::This, objwbemobject: ::core::option::Option<&super::Com::IDispatch>, iflags: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Qualifiers_(this: &Self::This) -> ::windows_core::Result<ISWbemQualifierSet>;
    fn Properties_(this: &Self::This) -> ::windows_core::Result<ISWbemPropertySet>;
    fn Methods_(this: &Self::This) -> ::windows_core::Result<ISWbemMethodSet>;
    fn Derivation_(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Path_(this: &Self::This) -> ::windows_core::Result<ISWbemObjectPath>;
    fn Security_(this: &Self::This) -> ::windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Put_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Put_(this, ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutAsync_(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Delete_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete_(this, ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into())
        }
        unsafe extern "system" fn DeleteAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAsync_(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Instances_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Instances_(this, ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstancesAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstancesAsync_(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Subclasses_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subclasses_(this, ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubclassesAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubclassesAsync_(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Associators_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            strassocclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemobjectset: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::Associators_(this, ::core::mem::transmute(&strassocclass), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strresultrole), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredassocqualifier), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        unsafe extern "system" fn AssociatorsAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            objwbemsink: *mut ::core::ffi::c_void,
            strassocclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemasynccontext: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AssociatorsAsync_(
                    this,
                    ::windows_core::from_raw_borrowed(&objwbemsink),
                    ::core::mem::transmute(&strassocclass),
                    ::core::mem::transmute(&strresultclass),
                    ::core::mem::transmute(&strresultrole),
                    ::core::mem::transmute(&strrole),
                    ::core::mem::transmute_copy(&bclassesonly),
                    ::core::mem::transmute_copy(&bschemaonly),
                    ::core::mem::transmute(&strrequiredassocqualifier),
                    ::core::mem::transmute(&strrequiredqualifier),
                    ::core::mem::transmute_copy(&iflags),
                    ::windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                    ::windows_core::from_raw_borrowed(&objwbemasynccontext),
                )
                .into()
            })
        }
        unsafe extern "system" fn References_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::References_(this, ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReferencesAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReferencesAsync_(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn ExecMethod_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecMethod_(this, ::core::mem::transmute(&strmethodname), ::windows_core::from_raw_borrowed(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemoutparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecMethodAsync_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecMethodAsync_(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strmethodname), ::windows_core::from_raw_borrowed(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Clone_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectText_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectText_(this, ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strobjecttext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpawnDerivedClass_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpawnDerivedClass_(this, ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpawnInstance_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpawnInstance_(this, ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareTo_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, bresult: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareTo_(this, ::windows_core::from_raw_borrowed(&objwbemobject), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Qualifiers_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Qualifiers_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifierset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbempropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Methods_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Methods_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemmethodset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Derivation_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Derivation_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strclassnamearray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemObject_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Put_: Put_::<Identity, Impl, OFFSET>,
            PutAsync_: PutAsync_::<Identity, Impl, OFFSET>,
            Delete_: Delete_::<Identity, Impl, OFFSET>,
            DeleteAsync_: DeleteAsync_::<Identity, Impl, OFFSET>,
            Instances_: Instances_::<Identity, Impl, OFFSET>,
            InstancesAsync_: InstancesAsync_::<Identity, Impl, OFFSET>,
            Subclasses_: Subclasses_::<Identity, Impl, OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Identity, Impl, OFFSET>,
            Associators_: Associators_::<Identity, Impl, OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Identity, Impl, OFFSET>,
            References_: References_::<Identity, Impl, OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Identity, Impl, OFFSET>,
            ExecMethod_: ExecMethod_::<Identity, Impl, OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Identity, Impl, OFFSET>,
            Clone_: Clone_::<Identity, Impl, OFFSET>,
            GetObjectText_: GetObjectText_::<Identity, Impl, OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Identity, Impl, OFFSET>,
            SpawnInstance_: SpawnInstance_::<Identity, Impl, OFFSET>,
            CompareTo_: CompareTo_::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
            Properties_: Properties_::<Identity, Impl, OFFSET>,
            Methods_: Methods_::<Identity, Impl, OFFSET>,
            Derivation_: Derivation_::<Identity, Impl, OFFSET>,
            Path_: Path_::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectEx_Impl: ::windows_core::BaseImpl + ISWbemObject_Impl {
    fn Refresh_(this: &Self::This, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn SystemProperties_(this: &Self::This) -> ::windows_core::Result<ISWbemPropertySet>;
    fn GetText_(this: &Self::This, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFromText_(this: &Self::This, bstext: &::windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemObjectEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISWbemObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemObjectEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Refresh_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh_(this, ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into())
        }
        unsafe extern "system" fn SystemProperties_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SystemProperties_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbempropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, bstext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText_(this, ::core::mem::transmute_copy(&iobjecttextformat), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFromText_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstext: ::std::mem::MaybeUninit<::windows_core::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFromText_(this, ::core::mem::transmute(&bstext), ::core::mem::transmute_copy(&iobjecttextformat), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into())
        }
        ISWbemObjectEx_Vtbl {
            base__: <ISWbemObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Refresh_: Refresh_::<Identity, Impl, OFFSET>,
            SystemProperties_: SystemProperties_::<Identity, Impl, OFFSET>,
            GetText_: GetText_::<Identity, Impl, OFFSET>,
            SetFromText_: SetFromText_::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectPath_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(this: &Self::This, strpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RelPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRelPath(this: &Self::This, strrelpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Server(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServer(this: &Self::This, strserver: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Namespace(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNamespace(this: &Self::This, strnamespace: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ParentNamespace(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(this: &Self::This, strdisplayname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Class(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClass(this: &Self::This, strclass: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsClass(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAsClass(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsSingleton(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAsSingleton(this: &Self::This) -> ::windows_core::Result<()>;
    fn Keys(this: &Self::This) -> ::windows_core::Result<ISWbemNamedValueSet>;
    fn Security_(this: &Self::This) -> ::windows_core::Result<ISWbemSecurity>;
    fn Locale(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocale(this: &Self::This, strlocale: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Authority(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAuthority(this: &Self::This, strauthority: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemObjectPath {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemObjectPath {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&strpath)).into())
        }
        unsafe extern "system" fn RelPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strrelpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RelPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strrelpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRelPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strrelpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRelPath(this, ::core::mem::transmute(&strrelpath)).into())
        }
        unsafe extern "system" fn Server<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Server(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strserver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strserver: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServer(this, ::core::mem::transmute(&strserver)).into())
        }
        unsafe extern "system" fn Namespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Namespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespace: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamespace(this, ::core::mem::transmute(&strnamespace)).into())
        }
        unsafe extern "system" fn ParentNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strparentnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParentNamespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strparentnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&strdisplayname)).into())
        }
        unsafe extern "system" fn Class<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Class(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClass(this, ::core::mem::transmute(&strclass)).into())
        }
        unsafe extern "system" fn IsClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisclass: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAsClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAsClass(this).into())
        }
        unsafe extern "system" fn IsSingleton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bissingleton: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSingleton(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bissingleton, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAsSingleton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAsSingleton(this).into())
        }
        unsafe extern "system" fn Keys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Keys(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalueset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Locale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strlocale: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Locale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strlocale, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocale(this, ::core::mem::transmute(&strlocale)).into())
        }
        unsafe extern "system" fn Authority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strauthority: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Authority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strauthority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strauthority: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthority(this, ::core::mem::transmute(&strauthority)).into())
        }
        ISWbemObjectPath_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            RelPath: RelPath::<Identity, Impl, OFFSET>,
            SetRelPath: SetRelPath::<Identity, Impl, OFFSET>,
            Server: Server::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            SetNamespace: SetNamespace::<Identity, Impl, OFFSET>,
            ParentNamespace: ParentNamespace::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            IsClass: IsClass::<Identity, Impl, OFFSET>,
            SetAsClass: SetAsClass::<Identity, Impl, OFFSET>,
            IsSingleton: IsSingleton::<Identity, Impl, OFFSET>,
            SetAsSingleton: SetAsSingleton::<Identity, Impl, OFFSET>,
            Keys: Keys::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
            Locale: Locale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            Authority: Authority::<Identity, Impl, OFFSET>,
            SetAuthority: SetAuthority::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectSet_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, strobjectpath: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<ISWbemObject>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Security_(this: &Self::This) -> ::windows_core::Result<ISWbemSecurity>;
    fn ItemIndex(this: &Self::This, lindex: i32) -> ::windows_core::Result<ISWbemObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemObjectSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemObjectSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemIndex(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemObjectSet_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
            ItemIndex: ItemIndex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPrivilege_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsEnabled(this: &Self::This, bisenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Identifier(this: &Self::This) -> ::windows_core::Result<WbemPrivilegeEnum>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemPrivilege {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemPrivilege {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsEnabled(this, ::core::mem::transmute_copy(&bisenabled)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Identifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Identifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iprivilege, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemPrivilege_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Identifier: Identifier::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPrivilegeSet_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, iprivilege: WbemPrivilegeEnum) -> ::windows_core::Result<ISWbemPrivilege>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<ISWbemPrivilege>;
    fn Remove(this: &Self::This, iprivilege: WbemPrivilegeEnum) -> ::windows_core::Result<()>;
    fn DeleteAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddAsString(this: &Self::This, strprivilege: &::windows_core::BSTR, bisenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<ISWbemPrivilege>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemPrivilegeSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemPrivilegeSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&iprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilege, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute_copy(&iprivilege), ::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilege, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&iprivilege)).into())
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAll(this).into())
        }
        unsafe extern "system" fn AddAsString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprivilege: ::std::mem::MaybeUninit<::windows_core::BSTR>, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddAsString(this, ::core::mem::transmute(&strprivilege), ::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilege, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemPrivilegeSet_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
            AddAsString: AddAsString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemProperty_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, varvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsLocal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Origin(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CIMType(this: &Self::This) -> ::windows_core::Result<WbemCimtypeEnum>;
    fn Qualifiers_(this: &Self::This) -> ::windows_core::Result<ISWbemQualifierSet>;
    fn IsArray(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&varvalue)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bislocal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Origin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strorigin: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Origin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strorigin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CIMType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CIMType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icimtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Qualifiers_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Qualifiers_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifierset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisarray: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsArray(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemProperty_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            Origin: Origin::<Identity, Impl, OFFSET>,
            CIMType: CIMType::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
            IsArray: IsArray::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPropertySet_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, strname: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<ISWbemProperty>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, strname: &::windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> ::windows_core::Result<ISWbemProperty>;
    fn Remove(this: &Self::This, strname: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemPropertySet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&icimtype), ::core::mem::transmute_copy(&bisarray), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)).into())
        }
        ISWbemPropertySet_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemQualifier_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, varvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsLocal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PropagatesToSubclass(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPropagatesToSubclass(this: &Self::This, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PropagatesToInstance(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPropagatesToInstance(this: &Self::This, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsOverridable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsOverridable(this: &Self::This, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsAmended(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemQualifier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemQualifier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&varvalue)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bislocal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropagatesToSubclass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropagatesToSubclass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bpropagatestosubclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropagatesToSubclass(this, ::core::mem::transmute_copy(&bpropagatestosubclass)).into())
        }
        unsafe extern "system" fn PropagatesToInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropagatesToInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bpropagatestoinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropagatesToInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropagatesToInstance(this, ::core::mem::transmute_copy(&bpropagatestoinstance)).into())
        }
        unsafe extern "system" fn IsOverridable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisoverridable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOverridable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisoverridable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsOverridable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsOverridable(this, ::core::mem::transmute_copy(&bisoverridable)).into())
        }
        unsafe extern "system" fn IsAmended<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisamended: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAmended(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisamended, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemQualifier_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            PropagatesToSubclass: PropagatesToSubclass::<Identity, Impl, OFFSET>,
            SetPropagatesToSubclass: SetPropagatesToSubclass::<Identity, Impl, OFFSET>,
            PropagatesToInstance: PropagatesToInstance::<Identity, Impl, OFFSET>,
            SetPropagatesToInstance: SetPropagatesToInstance::<Identity, Impl, OFFSET>,
            IsOverridable: IsOverridable::<Identity, Impl, OFFSET>,
            SetIsOverridable: SetIsOverridable::<Identity, Impl, OFFSET>,
            IsAmended: IsAmended::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemQualifierSet_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, name: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<ISWbemQualifier>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, strname: &::windows_core::BSTR, varval: *const super::Variant::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> ::windows_core::Result<ISWbemQualifier>;
    fn Remove(this: &Self::This, strname: &::windows_core::BSTR, iflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemQualifierSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemQualifierSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varval: *const super::Variant::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&varval), ::core::mem::transmute_copy(&bpropagatestosubclass), ::core::mem::transmute_copy(&bpropagatestoinstance), ::core::mem::transmute_copy(&bisoverridable), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)).into())
        }
        ISWbemQualifierSet_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemRefreshableItem_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Index(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Refresher(this: &Self::This) -> ::windows_core::Result<ISWbemRefresher>;
    fn IsSet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Object(this: &Self::This) -> ::windows_core::Result<ISWbemObjectEx>;
    fn ObjectSet(this: &Self::This) -> ::windows_core::Result<ISWbemObjectSet>;
    fn Remove(this: &Self::This, iflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemRefreshableItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemRefreshableItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Index<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Index(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Refresher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefresher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisset: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Object<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Object(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ObjectSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&iflags)).into())
        }
        ISWbemRefreshableItem_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Index: Index::<Identity, Impl, OFFSET>,
            Refresher: Refresher::<Identity, Impl, OFFSET>,
            IsSet: IsSet::<Identity, Impl, OFFSET>,
            Object: Object::<Identity, Impl, OFFSET>,
            ObjectSet: ObjectSet::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemRefresher_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, iindex: i32) -> ::windows_core::Result<ISWbemRefreshableItem>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, objwbemservices: ::core::option::Option<&ISWbemServicesEx>, bsinstancepath: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemRefreshableItem>;
    fn AddEnum(this: &Self::This, objwbemservices: ::core::option::Option<&ISWbemServicesEx>, bsclassname: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemRefreshableItem>;
    fn Remove(this: &Self::This, iindex: i32, iflags: i32) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This, iflags: i32) -> ::windows_core::Result<()>;
    fn AutoReconnect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoReconnect(this: &Self::This, bcount: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DeleteAll(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemRefresher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemRefresher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefreshableitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemservices: *mut ::core::ffi::c_void, bsinstancepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::windows_core::from_raw_borrowed(&objwbemservices), ::core::mem::transmute(&bsinstancepath), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefreshableitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemservices: *mut ::core::ffi::c_void, bsclassname: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddEnum(this, ::windows_core::from_raw_borrowed(&objwbemservices), ::core::mem::transmute(&bsclassname), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefreshableitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iflags)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this, ::core::mem::transmute_copy(&iflags)).into())
        }
        unsafe extern "system" fn AutoReconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcount: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoReconnect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoReconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcount: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoReconnect(this, ::core::mem::transmute_copy(&bcount)).into())
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAll(this).into())
        }
        ISWbemRefresher_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddEnum: AddEnum::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            AutoReconnect: AutoReconnect::<Identity, Impl, OFFSET>,
            SetAutoReconnect: SetAutoReconnect::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSecurity_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ImpersonationLevel(this: &Self::This) -> ::windows_core::Result<WbemImpersonationLevelEnum>;
    fn SetImpersonationLevel(this: &Self::This, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows_core::Result<()>;
    fn AuthenticationLevel(this: &Self::This) -> ::windows_core::Result<WbemAuthenticationLevelEnum>;
    fn SetAuthenticationLevel(this: &Self::This, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows_core::Result<()>;
    fn Privileges(this: &Self::This) -> ::windows_core::Result<ISWbemPrivilegeSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ImpersonationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImpersonationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iimpersonationlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImpersonationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImpersonationLevel(this, ::core::mem::transmute_copy(&iimpersonationlevel)).into())
        }
        unsafe extern "system" fn AuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationLevel(this, ::core::mem::transmute_copy(&iauthenticationlevel)).into())
        }
        unsafe extern "system" fn Privileges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Privileges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilegeset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemSecurity_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ImpersonationLevel: ImpersonationLevel::<Identity, Impl, OFFSET>,
            SetImpersonationLevel: SetImpersonationLevel::<Identity, Impl, OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Identity, Impl, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemServices_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Get(this: &Self::This, strobjectpath: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObject>;
    fn GetAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strobjectpath: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, strobjectpath: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn DeleteAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strobjectpath: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn InstancesOf(this: &Self::This, strclass: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strclass: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn SubclassesOf(this: &Self::This, strsuperclass: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strsuperclass: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn ExecQuery(this: &Self::This, strquery: &::windows_core::BSTR, strquerylanguage: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strquery: &::windows_core::BSTR, strquerylanguage: &::windows_core::BSTR, lflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AssociatorsOf(this: &Self::This, strobjectpath: &::windows_core::BSTR, strassocclass: &::windows_core::BSTR, strresultclass: &::windows_core::BSTR, strresultrole: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &::windows_core::BSTR, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strobjectpath: &::windows_core::BSTR, strassocclass: &::windows_core::BSTR, strresultclass: &::windows_core::BSTR, strresultrole: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &::windows_core::BSTR, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn ReferencesTo(this: &Self::This, strobjectpath: &::windows_core::BSTR, strresultclass: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strobjectpath: &::windows_core::BSTR, strresultclass: &::windows_core::BSTR, strrole: &::windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn ExecNotificationQuery(this: &Self::This, strquery: &::windows_core::BSTR, strquerylanguage: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strquery: &::windows_core::BSTR, strquerylanguage: &::windows_core::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn ExecMethod(this: &Self::This, strobjectpath: &::windows_core::BSTR, strmethodname: &::windows_core::BSTR, objwbeminparameters: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync(this: &Self::This, objwbemsink: ::core::option::Option<&super::Com::IDispatch>, strobjectpath: &::windows_core::BSTR, strmethodname: &::windows_core::BSTR, objwbeminparameters: ::core::option::Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Security_(this: &Self::This) -> ::windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into())
        }
        unsafe extern "system" fn DeleteAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn InstancesOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstancesOf(this, ::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstancesOfAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstancesOfAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn SubclassesOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubclassesOf(this, ::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubclassesOfAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubclassesOfAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn ExecQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecQuery(this, ::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecQueryAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn AssociatorsOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strassocclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemobjectset: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::AssociatorsOf(
                    this,
                    ::core::mem::transmute(&strobjectpath),
                    ::core::mem::transmute(&strassocclass),
                    ::core::mem::transmute(&strresultclass),
                    ::core::mem::transmute(&strresultrole),
                    ::core::mem::transmute(&strrole),
                    ::core::mem::transmute_copy(&bclassesonly),
                    ::core::mem::transmute_copy(&bschemaonly),
                    ::core::mem::transmute(&strrequiredassocqualifier),
                    ::core::mem::transmute(&strrequiredqualifier),
                    ::core::mem::transmute_copy(&iflags),
                    ::windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        unsafe extern "system" fn AssociatorsOfAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            objwbemsink: *mut ::core::ffi::c_void,
            strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strassocclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strresultrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemasynccontext: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AssociatorsOfAsync(
                    this,
                    ::windows_core::from_raw_borrowed(&objwbemsink),
                    ::core::mem::transmute(&strobjectpath),
                    ::core::mem::transmute(&strassocclass),
                    ::core::mem::transmute(&strresultclass),
                    ::core::mem::transmute(&strresultrole),
                    ::core::mem::transmute(&strrole),
                    ::core::mem::transmute_copy(&bclassesonly),
                    ::core::mem::transmute_copy(&bschemaonly),
                    ::core::mem::transmute(&strrequiredassocqualifier),
                    ::core::mem::transmute(&strrequiredqualifier),
                    ::core::mem::transmute_copy(&iflags),
                    ::windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                    ::windows_core::from_raw_borrowed(&objwbemasynccontext),
                )
                .into()
            })
        }
        unsafe extern "system" fn ReferencesTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferencesTo(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReferencesToAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::ReferencesToAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
            })
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemeventsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecNotificationQuery(this, ::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemeventsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecNotificationQueryAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn ExecMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecMethod(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::windows_core::from_raw_borrowed(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemoutparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecMethodAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::windows_core::from_raw_borrowed(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        unsafe extern "system" fn Security_<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security_(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISWbemServices_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Get: Get::<Identity, Impl, OFFSET>,
            GetAsync: GetAsync::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, Impl, OFFSET>,
            InstancesOf: InstancesOf::<Identity, Impl, OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Identity, Impl, OFFSET>,
            SubclassesOf: SubclassesOf::<Identity, Impl, OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Identity, Impl, OFFSET>,
            ExecQuery: ExecQuery::<Identity, Impl, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, Impl, OFFSET>,
            AssociatorsOf: AssociatorsOf::<Identity, Impl, OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Identity, Impl, OFFSET>,
            ReferencesTo: ReferencesTo::<Identity, Impl, OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Identity, Impl, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, Impl, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, Impl, OFFSET>,
            ExecMethod: ExecMethod::<Identity, Impl, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemServicesEx_Impl: ::windows_core::BaseImpl + ISWbemServices_Impl {
    fn Put(this: &Self::This, objwbemobject: ::core::option::Option<&ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<ISWbemObjectPath>;
    fn PutAsync(this: &Self::This, objwbemsink: ::core::option::Option<&ISWbemSink>, objwbemobject: ::core::option::Option<&ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<&super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemServicesEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISWbemServices);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemServicesEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Put<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Put(this, ::windows_core::from_raw_borrowed(&objwbemobject), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutAsync(this, ::windows_core::from_raw_borrowed(&objwbemsink), ::windows_core::from_raw_borrowed(&objwbemobject), ::core::mem::transmute_copy(&iflags), ::windows_core::from_raw_borrowed(&objwbemnamedvalueset), ::windows_core::from_raw_borrowed(&objwbemasynccontext)).into())
        }
        ISWbemServicesEx_Vtbl {
            base__: <ISWbemServices as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Put: Put::<Identity, Impl, OFFSET>,
            PutAsync: PutAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSink_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        ISWbemSink_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Cancel: Cancel::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSinkEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISWbemSinkEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISWbemSinkEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISWbemSinkEvents {
    const VTABLE: Self::Vtable = { ISWbemSinkEvents_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUnsecuredApartment_Impl: ::windows_core::BaseImpl {
    fn CreateObjectStub(this: &Self::This, pobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IUnsecuredApartment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnsecuredApartment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUnsecuredApartment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateObjectStub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnsecuredApartment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateObjectStub(this, ::windows_core::from_raw_borrowed(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUnsecuredApartment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateObjectStub: CreateObjectStub::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMIExtension_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn WMIObjectPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetWMIObject(this: &Self::This) -> ::windows_core::Result<ISWbemObject>;
    fn GetWMIServices(this: &Self::This) -> ::windows_core::Result<ISWbemServices>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMIExtension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMIExtension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WMIObjectPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WMIObjectPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strwmiobjectpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWMIObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwmiobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWMIObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwmiobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWMIServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwmiservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWMIServices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwmiservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMIExtension_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WMIObjectPath: WMIObjectPath::<Identity, Impl, OFFSET>,
            GetWMIObject: GetWMIObject::<Identity, Impl, OFFSET>,
            GetWMIServices: GetWMIServices::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemAddressResolution_Impl: ::windows_core::BaseImpl {
    fn Resolve(this: &Self::This, wsznamespacepath: &::windows_core::PCWSTR, wszaddresstype: ::windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemAddressResolution {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemAddressResolution_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemAddressResolution {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Resolve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemAddressResolution_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznamespacepath: ::windows_core::PCWSTR, wszaddresstype: ::windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resolve(this, ::core::mem::transmute(&wsznamespacepath), ::core::mem::transmute_copy(&wszaddresstype), ::core::mem::transmute_copy(&pdwaddresslength), ::core::mem::transmute_copy(&pabbinaryaddress)).into())
        }
        IWbemAddressResolution_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Resolve: Resolve::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemBackupRestore_Impl: ::windows_core::BaseImpl {
    fn Backup(this: &Self::This, strbackuptofile: &::windows_core::PCWSTR, lflags: i32) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This, strrestorefromfile: &::windows_core::PCWSTR, lflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemBackupRestore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemBackupRestore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Backup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strbackuptofile: ::windows_core::PCWSTR, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Backup(this, ::core::mem::transmute(&strbackuptofile), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strrestorefromfile: ::windows_core::PCWSTR, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this, ::core::mem::transmute(&strrestorefromfile), ::core::mem::transmute_copy(&lflags)).into())
        }
        IWbemBackupRestore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemBackupRestoreEx_Impl: ::windows_core::BaseImpl + IWbemBackupRestore_Impl {
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemBackupRestoreEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWbemBackupRestore);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemBackupRestoreEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        IWbemBackupRestoreEx_Vtbl {
            base__: <IWbemBackupRestore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemCallResult_Impl: ::windows_core::BaseImpl {
    fn GetResultObject(this: &Self::This, ltimeout: i32) -> ::windows_core::Result<IWbemClassObject>;
    fn GetResultString(this: &Self::This, ltimeout: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetResultServices(this: &Self::This, ltimeout: i32) -> ::windows_core::Result<IWbemServices>;
    fn GetCallStatus(this: &Self::This, ltimeout: i32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IWbemCallResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemCallResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResultObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResultObject(this, ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresultobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResultString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResultString(this, ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrresultstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResultServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResultServices(this, ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCallStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCallStatus(this, ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemCallResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResultObject: GetResultObject::<Identity, Impl, OFFSET>,
            GetResultString: GetResultString::<Identity, Impl, OFFSET>,
            GetResultServices: GetResultServices::<Identity, Impl, OFFSET>,
            GetCallStatus: GetCallStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemClassObject_Impl: ::windows_core::BaseImpl {
    fn GetQualifierSet(this: &Self::This) -> ::windows_core::Result<IWbemQualifierSet>;
    fn Get(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::Result<()>;
    fn Put(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32, pval: *const super::Variant::VARIANT, r#type: i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, wszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetNames(this: &Self::This, wszqualifiername: &::windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const super::Variant::VARIANT) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(this: &Self::This, lenumflags: i32) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, lflags: i32, strname: *mut ::windows_core::BSTR, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::Result<()>;
    fn EndEnumeration(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetPropertyQualifierSet(this: &Self::This, wszproperty: &::windows_core::PCWSTR) -> ::windows_core::Result<IWbemQualifierSet>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IWbemClassObject>;
    fn GetObjectText(this: &Self::This, lflags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SpawnDerivedClass(this: &Self::This, lflags: i32) -> ::windows_core::Result<IWbemClassObject>;
    fn SpawnInstance(this: &Self::This, lflags: i32) -> ::windows_core::Result<IWbemClassObject>;
    fn CompareTo(this: &Self::This, lflags: WBEM_COMPARISON_FLAG, pcompareto: ::core::option::Option<&IWbemClassObject>) -> ::windows_core::Result<()>;
    fn GetPropertyOrigin(this: &Self::This, wszname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InheritsFrom(this: &Self::This, strancestor: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetMethod(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()>;
    fn PutMethod(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32, pinsignature: ::core::option::Option<&IWbemClassObject>, poutsignature: ::core::option::Option<&IWbemClassObject>) -> ::windows_core::Result<()>;
    fn DeleteMethod(this: &Self::This, wszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn BeginMethodEnumeration(this: &Self::This, lenumflags: i32) -> ::windows_core::Result<()>;
    fn NextMethod(this: &Self::This, lflags: i32, pstrname: *mut ::windows_core::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()>;
    fn EndMethodEnumeration(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMethodQualifierSet(this: &Self::This, wszmethod: &::windows_core::PCWSTR) -> ::windows_core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(this: &Self::This, wszmethodname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemClassObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemClassObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetQualifierSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQualifierSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plflavor)).into())
        }
        unsafe extern "system" fn Put<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pval: *const super::Variant::VARIANT, r#type: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Put(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&wszname)).into())
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszqualifiername: ::windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const super::Variant::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNames(this, ::core::mem::transmute(&wszqualifiername), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pqualifierval)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEnumeration(this, ::core::mem::transmute_copy(&lenumflags)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plflavor)).into())
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEnumeration(this).into())
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszproperty: ::windows_core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyQualifierSet(this, ::core::mem::transmute(&wszproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcopy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectText(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrobjecttext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpawnDerivedClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpawnDerivedClass(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpawnInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpawnInstance(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: WBEM_COMPARISON_FLAG, pcompareto: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareTo(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcompareto)).into())
        }
        unsafe extern "system" fn GetPropertyOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, pstrclassname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyOrigin(this, ::core::mem::transmute(&wszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrclassname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InheritsFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strancestor: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InheritsFrom(this, ::core::mem::transmute(&strancestor)).into())
        }
        unsafe extern "system" fn GetMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMethod(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)).into())
        }
        unsafe extern "system" fn PutMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pinsignature: *mut ::core::ffi::c_void, poutsignature: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutMethod(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pinsignature), ::windows_core::from_raw_borrowed(&poutsignature)).into())
        }
        unsafe extern "system" fn DeleteMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMethod(this, ::core::mem::transmute(&wszname)).into())
        }
        unsafe extern "system" fn BeginMethodEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginMethodEnumeration(this, ::core::mem::transmute_copy(&lenumflags)).into())
        }
        unsafe extern "system" fn NextMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextMethod(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)).into())
        }
        unsafe extern "system" fn EndMethodEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndMethodEnumeration(this).into())
        }
        unsafe extern "system" fn GetMethodQualifierSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmethod: ::windows_core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMethodQualifierSet(this, ::core::mem::transmute(&wszmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMethodOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmethodname: ::windows_core::PCWSTR, pstrclassname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMethodOrigin(this, ::core::mem::transmute(&wszmethodname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrclassname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemClassObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetQualifierSet: GetQualifierSet::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetObjectText: GetObjectText::<Identity, Impl, OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Identity, Impl, OFFSET>,
            SpawnInstance: SpawnInstance::<Identity, Impl, OFFSET>,
            CompareTo: CompareTo::<Identity, Impl, OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Identity, Impl, OFFSET>,
            InheritsFrom: InheritsFrom::<Identity, Impl, OFFSET>,
            GetMethod: GetMethod::<Identity, Impl, OFFSET>,
            PutMethod: PutMethod::<Identity, Impl, OFFSET>,
            DeleteMethod: DeleteMethod::<Identity, Impl, OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Identity, Impl, OFFSET>,
            NextMethod: NextMethod::<Identity, Impl, OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Identity, Impl, OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Identity, Impl, OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemClientConnectionTransport_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, straddresstype: &::windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &::windows_core::BSTR, struser: &::windows_core::BSTR, strpassword: &::windows_core::BSTR, strlocale: &::windows_core::BSTR, lflags: i32, pctx: ::core::option::Option<&IWbemContext>, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn OpenAsync(this: &Self::This, straddresstype: &::windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &::windows_core::BSTR, struser: &::windows_core::BSTR, strpassword: &::windows_core::BSTR, strlocale: &::windows_core::BSTR, lflags: i32, pctx: ::core::option::Option<&IWbemContext>, riid: *const ::windows_core::GUID, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This, lflags: i32, phandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemClientConnectionTransport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemClientConnectionTransport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straddresstype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::std::mem::MaybeUninit<::windows_core::BSTR>, struser: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::Open(
                    this,
                    ::core::mem::transmute(&straddresstype),
                    ::core::mem::transmute_copy(&dwbinaryaddresslength),
                    ::core::mem::transmute_copy(&abbinaryaddress),
                    ::core::mem::transmute(&strobject),
                    ::core::mem::transmute(&struser),
                    ::core::mem::transmute(&strpassword),
                    ::core::mem::transmute(&strlocale),
                    ::core::mem::transmute_copy(&lflags),
                    ::windows_core::from_raw_borrowed(&pctx),
                    ::core::mem::transmute_copy(&riid),
                    ::core::mem::transmute_copy(&pinterface),
                    ::core::mem::transmute_copy(&pcallres),
                )
                .into()
            })
        }
        unsafe extern "system" fn OpenAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straddresstype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::std::mem::MaybeUninit<::windows_core::BSTR>, struser: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::OpenAsync(this, ::core::mem::transmute(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute(&strobject), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&presponsehandler)).into()
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&phandler)).into())
        }
        IWbemClientConnectionTransport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemClientTransport_Impl: ::windows_core::BaseImpl {
    fn ConnectServer(this: &Self::This, straddresstype: &::windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &::windows_core::BSTR, struser: &::windows_core::BSTR, strpassword: &::windows_core::BSTR, strlocale: &::windows_core::BSTR, lsecurityflags: i32, strauthority: &::windows_core::BSTR, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemServices>;
}
impl ::windows_core::Iids for IWbemClientTransport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClientTransport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemClientTransport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemClientTransport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straddresstype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::std::mem::MaybeUninit<::windows_core::BSTR>, struser: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, lsecurityflags: i32, strauthority: ::std::mem::MaybeUninit<::windows_core::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectServer(this, ::core::mem::transmute(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute(&strnetworkresource), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lsecurityflags), ::core::mem::transmute(&strauthority), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemClientTransport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ConnectServer: ConnectServer::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemConfigureRefresher_Impl: ::windows_core::BaseImpl {
    fn AddObjectByPath(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, wszpath: &::windows_core::PCWSTR, lflags: i32, pcontext: ::core::option::Option<&IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows_core::Result<()>;
    fn AddObjectByTemplate(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, ptemplate: ::core::option::Option<&IWbemClassObject>, lflags: i32, pcontext: ::core::option::Option<&IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows_core::Result<()>;
    fn AddRefresher(this: &Self::This, prefresher: ::core::option::Option<&IWbemRefresher>, lflags: i32, plid: *mut i32) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, lid: i32, lflags: i32) -> ::windows_core::Result<()>;
    fn AddEnum(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, wszclassname: &::windows_core::PCWSTR, lflags: i32, pcontext: ::core::option::Option<&IWbemContext>, ppenum: *mut ::core::option::Option<IWbemHiPerfEnum>, plid: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemConfigureRefresher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemConfigureRefresher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddObjectByPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddObjectByPath(this, ::windows_core::from_raw_borrowed(&pnamespace), ::core::mem::transmute(&wszpath), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into())
        }
        unsafe extern "system" fn AddObjectByTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddObjectByTemplate(this, ::windows_core::from_raw_borrowed(&pnamespace), ::windows_core::from_raw_borrowed(&ptemplate), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into())
        }
        unsafe extern "system" fn AddRefresher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefresher(this, ::windows_core::from_raw_borrowed(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&plid)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&lid), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn AddEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclassname: ::windows_core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddEnum(this, ::windows_core::from_raw_borrowed(&pnamespace), ::core::mem::transmute(&wszclassname), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&plid)).into())
        }
        IWbemConfigureRefresher_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddObjectByPath: AddObjectByPath::<Identity, Impl, OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Identity, Impl, OFFSET>,
            AddRefresher: AddRefresher::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            AddEnum: AddEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemConnectorLogin_Impl: ::windows_core::BaseImpl {
    fn ConnectorLogin(this: &Self::This, wsznetworkresource: &::windows_core::PCWSTR, wszpreferredlocale: &::windows_core::PCWSTR, lflags: i32, pctx: ::core::option::Option<&IWbemContext>, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemConnectorLogin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConnectorLogin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemConnectorLogin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectorLogin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConnectorLogin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows_core::PCWSTR, wszpreferredlocale: ::windows_core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectorLogin(this, ::core::mem::transmute(&wsznetworkresource), ::core::mem::transmute(&wszpreferredlocale), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into())
        }
        IWbemConnectorLogin_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ConnectorLogin: ConnectorLogin::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemConstructClassObject_Impl: ::windows_core::BaseImpl {
    fn SetInheritanceChain(this: &Self::This, lnumantecedents: i32, awszantecedents: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetPropertyOrigin(this: &Self::This, wszpropertyname: &::windows_core::PCWSTR, loriginindex: i32) -> ::windows_core::Result<()>;
    fn SetMethodOrigin(this: &Self::This, wszmethodname: &::windows_core::PCWSTR, loriginindex: i32) -> ::windows_core::Result<()>;
    fn SetServerNamespace(this: &Self::This, wszserver: &::windows_core::PCWSTR, wsznamespace: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemConstructClassObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemConstructClassObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInheritanceChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInheritanceChain(this, ::core::mem::transmute_copy(&lnumantecedents), ::core::mem::transmute_copy(&awszantecedents)).into())
        }
        unsafe extern "system" fn SetPropertyOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows_core::PCWSTR, loriginindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyOrigin(this, ::core::mem::transmute(&wszpropertyname), ::core::mem::transmute_copy(&loriginindex)).into())
        }
        unsafe extern "system" fn SetMethodOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmethodname: ::windows_core::PCWSTR, loriginindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMethodOrigin(this, ::core::mem::transmute(&wszmethodname), ::core::mem::transmute_copy(&loriginindex)).into())
        }
        unsafe extern "system" fn SetServerNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszserver: ::windows_core::PCWSTR, wsznamespace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerNamespace(this, ::core::mem::transmute(&wszserver), ::core::mem::transmute(&wsznamespace)).into())
        }
        IWbemConstructClassObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInheritanceChain: SetInheritanceChain::<Identity, Impl, OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Identity, Impl, OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Identity, Impl, OFFSET>,
            SetServerNamespace: SetServerNamespace::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemContext_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IWbemContext>;
    fn GetNames(this: &Self::This, lflags: i32) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, lflags: i32, pstrname: *mut ::windows_core::BSTR, pvalue: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EndEnumeration(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32, pvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DeleteValue(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32) -> ::windows_core::Result<()>;
    fn DeleteAll(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewcopy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewcopy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNames(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEnumeration(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEnumeration(this).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteValue(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAll(this).into())
        }
        IWbemContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemDecoupledBasicEventProvider_Impl: ::windows_core::BaseImpl + IWbemDecoupledRegistrar_Impl {
    fn GetSink(this: &Self::This, a_flags: i32, a_context: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemObjectSink>;
    fn GetService(this: &Self::This, a_flags: i32, a_context: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemServices>;
}
impl ::windows_core::Iids for IWbemDecoupledBasicEventProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWbemDecoupledRegistrar);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemDecoupledBasicEventProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_sink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSink(this, ::core::mem::transmute_copy(&a_flags), ::windows_core::from_raw_borrowed(&a_context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(a_sink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_service: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetService(this, ::core::mem::transmute_copy(&a_flags), ::windows_core::from_raw_borrowed(&a_context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(a_service, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemDecoupledBasicEventProvider_Vtbl {
            base__: <IWbemDecoupledRegistrar as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSink: GetSink::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemDecoupledRegistrar_Impl: ::windows_core::BaseImpl {
    fn Register(this: &Self::This, a_flags: i32, a_context: ::core::option::Option<&IWbemContext>, a_user: &::windows_core::PCWSTR, a_locale: &::windows_core::PCWSTR, a_scope: &::windows_core::PCWSTR, a_registration: &::windows_core::PCWSTR, piunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn UnRegister(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemDecoupledRegistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemDecoupledRegistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_user: ::windows_core::PCWSTR, a_locale: ::windows_core::PCWSTR, a_scope: ::windows_core::PCWSTR, a_registration: ::windows_core::PCWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Register(this, ::core::mem::transmute_copy(&a_flags), ::windows_core::from_raw_borrowed(&a_context), ::core::mem::transmute(&a_user), ::core::mem::transmute(&a_locale), ::core::mem::transmute(&a_scope), ::core::mem::transmute(&a_registration), ::windows_core::from_raw_borrowed(&piunknown)).into())
        }
        unsafe extern "system" fn UnRegister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegister(this).into())
        }
        IWbemDecoupledRegistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Register: Register::<Identity, Impl, OFFSET>,
            UnRegister: UnRegister::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemEventConsumerProvider_Impl: ::windows_core::BaseImpl {
    fn FindConsumer(this: &Self::This, plogicalconsumer: ::core::option::Option<&IWbemClassObject>) -> ::windows_core::Result<IWbemUnboundObjectSink>;
}
impl ::windows_core::Iids for IWbemEventConsumerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventConsumerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemEventConsumerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindConsumer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventConsumerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogicalconsumer: *mut ::core::ffi::c_void, ppconsumer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindConsumer(this, ::windows_core::from_raw_borrowed(&plogicalconsumer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconsumer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemEventConsumerProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FindConsumer: FindConsumer::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemEventProvider_Impl: ::windows_core::BaseImpl {
    fn ProvideEvents(this: &Self::This, psink: ::core::option::Option<&IWbemObjectSink>, lflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemEventProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemEventProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProvideEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProvideEvents(this, ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&lflags)).into())
        }
        IWbemEventProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ProvideEvents: ProvideEvents::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemEventProviderQuerySink_Impl: ::windows_core::BaseImpl {
    fn NewQuery(this: &Self::This, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows_core::Result<()>;
    fn CancelQuery(this: &Self::This, dwid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemEventProviderQuerySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemEventProviderQuerySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NewQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewQuery(this, ::core::mem::transmute_copy(&dwid), ::core::mem::transmute_copy(&wszquerylanguage), ::core::mem::transmute_copy(&wszquery)).into())
        }
        unsafe extern "system" fn CancelQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelQuery(this, ::core::mem::transmute_copy(&dwid)).into())
        }
        IWbemEventProviderQuerySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NewQuery: NewQuery::<Identity, Impl, OFFSET>,
            CancelQuery: CancelQuery::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemEventProviderSecurity_Impl: ::windows_core::BaseImpl {
    fn AccessCheck(this: &Self::This, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemEventProviderSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProviderSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemEventProviderSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccessCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventProviderSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AccessCheck(this, ::core::mem::transmute_copy(&wszquerylanguage), ::core::mem::transmute_copy(&wszquery), ::core::mem::transmute_copy(&lsidlength), ::core::mem::transmute_copy(&psid)).into())
        }
        IWbemEventProviderSecurity_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AccessCheck: AccessCheck::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemEventSink_Impl: ::windows_core::BaseImpl + IWbemObjectSink_Impl {
    fn SetSinkSecurity(this: &Self::This, lsdlength: i32, psd: *const u8) -> ::windows_core::Result<()>;
    fn IsActive(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRestrictedSink(this: &Self::This, lnumqueries: i32, awszqueries: *const ::windows_core::PCWSTR, pcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IWbemEventSink>;
    fn SetBatchingParameters(this: &Self::This, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWbemObjectSink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSinkSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSinkSecurity(this, ::core::mem::transmute_copy(&lsdlength), ::core::mem::transmute_copy(&psd)).into())
        }
        unsafe extern "system" fn IsActive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsActive(this).into())
        }
        unsafe extern "system" fn GetRestrictedSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const ::windows_core::PCWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictedSink(this, ::core::mem::transmute_copy(&lnumqueries), ::core::mem::transmute_copy(&awszqueries), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBatchingParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBatchingParameters(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&dwmaxsendlatency)).into())
        }
        IWbemEventSink_Vtbl {
            base__: <IWbemObjectSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSinkSecurity: SetSinkSecurity::<Identity, Impl, OFFSET>,
            IsActive: IsActive::<Identity, Impl, OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Identity, Impl, OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemHiPerfEnum_Impl: ::windows_core::BaseImpl {
    fn AddObjects(this: &Self::This, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::core::option::Option<IWbemObjectAccess>) -> ::windows_core::Result<()>;
    fn RemoveObjects(this: &Self::This, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows_core::Result<()>;
    fn GetObjects(this: &Self::This, lflags: i32, unumobjects: u32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, pureturned: *mut u32) -> ::windows_core::Result<()>;
    fn RemoveAll(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemHiPerfEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemHiPerfEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddObjects(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apids), ::core::mem::transmute_copy(&apobj)).into())
        }
        unsafe extern "system" fn RemoveObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveObjects(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apids)).into())
        }
        unsafe extern "system" fn GetObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjects(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&pureturned)).into())
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAll(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        IWbemHiPerfEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddObjects: AddObjects::<Identity, Impl, OFFSET>,
            RemoveObjects: RemoveObjects::<Identity, Impl, OFFSET>,
            GetObjects: GetObjects::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemHiPerfProvider_Impl: ::windows_core::BaseImpl {
    fn QueryInstances(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, wszclass: &::windows_core::PCWSTR, lflags: i32, pctx: ::core::option::Option<&IWbemContext>, psink: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn CreateRefresher(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, lflags: i32) -> ::windows_core::Result<IWbemRefresher>;
    fn CreateRefreshableObject(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, ptemplate: ::core::option::Option<&IWbemObjectAccess>, prefresher: ::core::option::Option<&IWbemRefresher>, lflags: i32, pcontext: ::core::option::Option<&IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemObjectAccess>, plid: *mut i32) -> ::windows_core::Result<()>;
    fn StopRefreshing(this: &Self::This, prefresher: ::core::option::Option<&IWbemRefresher>, lid: i32, lflags: i32) -> ::windows_core::Result<()>;
    fn CreateRefreshableEnum(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, wszclass: &::windows_core::PCWSTR, prefresher: ::core::option::Option<&IWbemRefresher>, lflags: i32, pcontext: ::core::option::Option<&IWbemContext>, phiperfenum: ::core::option::Option<&IWbemHiPerfEnum>) -> ::windows_core::Result<i32>;
    fn GetObjects(this: &Self::This, pnamespace: ::core::option::Option<&IWbemServices>, lnumobjects: i32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, lflags: i32, pcontext: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemHiPerfProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemHiPerfProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows_core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInstances(this, ::windows_core::from_raw_borrowed(&pnamespace), ::core::mem::transmute(&wszclass), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn CreateRefresher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, lflags: i32, pprefresher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRefresher(this, ::windows_core::from_raw_borrowed(&pnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprefresher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRefreshableObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRefreshableObject(this, ::windows_core::from_raw_borrowed(&pnamespace), ::windows_core::from_raw_borrowed(&ptemplate), ::windows_core::from_raw_borrowed(&prefresher), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into())
        }
        unsafe extern "system" fn StopRefreshing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopRefreshing(this, ::windows_core::from_raw_borrowed(&prefresher), ::core::mem::transmute_copy(&lid), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn CreateRefreshableEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows_core::PCWSTR, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, phiperfenum: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRefreshableEnum(this, ::windows_core::from_raw_borrowed(&pnamespace), ::core::mem::transmute(&wszclass), ::windows_core::from_raw_borrowed(&prefresher), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcontext), ::windows_core::from_raw_borrowed(&phiperfenum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjects(this, ::windows_core::from_raw_borrowed(&pnamespace), ::core::mem::transmute_copy(&lnumobjects), ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        IWbemHiPerfProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryInstances: QueryInstances::<Identity, Impl, OFFSET>,
            CreateRefresher: CreateRefresher::<Identity, Impl, OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Identity, Impl, OFFSET>,
            StopRefreshing: StopRefreshing::<Identity, Impl, OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Identity, Impl, OFFSET>,
            GetObjects: GetObjects::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemLevel1Login_Impl: ::windows_core::BaseImpl {
    fn EstablishPosition(this: &Self::This, wszlocalelist: &::windows_core::PCWSTR, dwnumlocales: u32) -> ::windows_core::Result<u32>;
    fn RequestChallenge(this: &Self::This, wsznetworkresource: &::windows_core::PCWSTR, wszuser: &::windows_core::PCWSTR) -> ::windows_core::Result<u8>;
    fn WBEMLogin(this: &Self::This, wszpreferredlocale: &::windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemServices>;
    fn NTLMLogin(this: &Self::This, wsznetworkresource: &::windows_core::PCWSTR, wszpreferredlocale: &::windows_core::PCWSTR, lflags: i32, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemServices>;
}
impl ::windows_core::Iids for IWbemLevel1Login {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemLevel1Login {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EstablishPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszlocalelist: ::windows_core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EstablishPosition(this, ::core::mem::transmute(&wszlocalelist), ::core::mem::transmute_copy(&dwnumlocales)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reserved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestChallenge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows_core::PCWSTR, wszuser: ::windows_core::PCWSTR, nonce: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestChallenge(this, ::core::mem::transmute(&wsznetworkresource), ::core::mem::transmute(&wszuser)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nonce, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WBEMLogin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpreferredlocale: ::windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WBEMLogin(this, ::core::mem::transmute(&wszpreferredlocale), ::core::mem::transmute_copy(&accesstoken), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NTLMLogin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows_core::PCWSTR, wszpreferredlocale: ::windows_core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NTLMLogin(this, ::core::mem::transmute(&wsznetworkresource), ::core::mem::transmute(&wszpreferredlocale), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemLevel1Login_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EstablishPosition: EstablishPosition::<Identity, Impl, OFFSET>,
            RequestChallenge: RequestChallenge::<Identity, Impl, OFFSET>,
            WBEMLogin: WBEMLogin::<Identity, Impl, OFFSET>,
            NTLMLogin: NTLMLogin::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemLocator_Impl: ::windows_core::BaseImpl {
    fn ConnectServer(this: &Self::This, strnetworkresource: &::windows_core::BSTR, struser: &::windows_core::BSTR, strpassword: &::windows_core::BSTR, strlocale: &::windows_core::BSTR, lsecurityflags: i32, strauthority: &::windows_core::BSTR, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemServices>;
}
impl ::windows_core::Iids for IWbemLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnetworkresource: ::std::mem::MaybeUninit<::windows_core::BSTR>, struser: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, lsecurityflags: i32, strauthority: ::std::mem::MaybeUninit<::windows_core::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectServer(this, ::core::mem::transmute(&strnetworkresource), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lsecurityflags), ::core::mem::transmute(&strauthority), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemLocator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ConnectServer: ConnectServer::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemObjectAccess_Impl: ::windows_core::BaseImpl + IWbemClassObject_Impl {
    fn GetPropertyHandle(this: &Self::This, wszpropertyname: &::windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows_core::Result<()>;
    fn WritePropertyValue(this: &Self::This, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows_core::Result<()>;
    fn ReadPropertyValue(this: &Self::This, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows_core::Result<()>;
    fn ReadDWORD(this: &Self::This, lhandle: i32) -> ::windows_core::Result<u32>;
    fn WriteDWORD(this: &Self::This, lhandle: i32, dw: u32) -> ::windows_core::Result<()>;
    fn ReadQWORD(this: &Self::This, lhandle: i32) -> ::windows_core::Result<u64>;
    fn WriteQWORD(this: &Self::This, lhandle: i32, pw: u64) -> ::windows_core::Result<()>;
    fn GetPropertyInfoByHandle(this: &Self::This, lhandle: i32, pstrname: *mut ::windows_core::BSTR, ptype: *mut i32) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemObjectAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWbemClassObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemObjectAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyHandle(this, ::core::mem::transmute(&wszpropertyname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plhandle)).into())
        }
        unsafe extern "system" fn WritePropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WritePropertyValue(this, ::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&lnumbytes), ::core::mem::transmute_copy(&adata)).into())
        }
        unsafe extern "system" fn ReadPropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadPropertyValue(this, ::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&lbuffersize), ::core::mem::transmute_copy(&plnumbytes), ::core::mem::transmute_copy(&adata)).into())
        }
        unsafe extern "system" fn ReadDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadDWORD(this, ::core::mem::transmute_copy(&lhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdw, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteDWORD(this, ::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&dw)).into())
        }
        unsafe extern "system" fn ReadQWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadQWORD(this, ::core::mem::transmute_copy(&lhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pqw, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteQWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteQWORD(this, ::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&pw)).into())
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ptype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyInfoByHandle(this, ::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ptype)).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        IWbemObjectAccess_Vtbl {
            base__: <IWbemClassObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyHandle: GetPropertyHandle::<Identity, Impl, OFFSET>,
            WritePropertyValue: WritePropertyValue::<Identity, Impl, OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Identity, Impl, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, Impl, OFFSET>,
            WriteDWORD: WriteDWORD::<Identity, Impl, OFFSET>,
            ReadQWORD: ReadQWORD::<Identity, Impl, OFFSET>,
            WriteQWORD: WriteQWORD::<Identity, Impl, OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemObjectSink_Impl: ::windows_core::BaseImpl {
    fn Indicate(this: &Self::This, lobjectcount: i32, apobjarray: *const ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, lflags: i32, hresult: ::windows_core::HRESULT, strparam: &::windows_core::BSTR, pobjparam: ::core::option::Option<&IWbemClassObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemObjectSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemObjectSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Indicate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Indicate(this, ::core::mem::transmute_copy(&lobjectcount), ::core::mem::transmute_copy(&apobjarray)).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows_core::HRESULT, strparam: ::std::mem::MaybeUninit<::windows_core::BSTR>, pobjparam: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&strparam), ::windows_core::from_raw_borrowed(&pobjparam)).into())
        }
        IWbemObjectSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Indicate: Indicate::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemObjectSinkEx_Impl: ::windows_core::BaseImpl + IWbemObjectSink_Impl {
    fn WriteMessage(this: &Self::This, uchannel: u32, strmessage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn WriteError(this: &Self::This, pobjerror: ::core::option::Option<&IWbemClassObject>) -> ::windows_core::Result<u8>;
    fn PromptUser(this: &Self::This, strmessage: &::windows_core::BSTR, uprompttype: u8) -> ::windows_core::Result<u8>;
    fn WriteProgress(this: &Self::This, stractivity: &::windows_core::BSTR, strcurrentoperation: &::windows_core::BSTR, strstatusdescription: &::windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> ::windows_core::Result<()>;
    fn WriteStreamParameter(this: &Self::This, strname: &::windows_core::BSTR, vtvalue: *const super::Variant::VARIANT, ultype: u32, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemObjectSinkEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWbemObjectSink);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemObjectSinkEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteMessage(this, ::core::mem::transmute_copy(&uchannel), ::core::mem::transmute(&strmessage)).into())
        }
        unsafe extern "system" fn WriteError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjerror: *mut ::core::ffi::c_void, pureturned: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteError(this, ::windows_core::from_raw_borrowed(&pobjerror)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pureturned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PromptUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PromptUser(this, ::core::mem::transmute(&strmessage), ::core::mem::transmute_copy(&uprompttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pureturned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stractivity: ::std::mem::MaybeUninit<::windows_core::BSTR>, strcurrentoperation: ::std::mem::MaybeUninit<::windows_core::BSTR>, strstatusdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteProgress(this, ::core::mem::transmute(&stractivity), ::core::mem::transmute(&strcurrentoperation), ::core::mem::transmute(&strstatusdescription), ::core::mem::transmute_copy(&upercentcomplete), ::core::mem::transmute_copy(&usecondsremaining)).into())
        }
        unsafe extern "system" fn WriteStreamParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vtvalue: *const super::Variant::VARIANT, ultype: u32, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStreamParameter(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&vtvalue), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&ulflags)).into())
        }
        IWbemObjectSinkEx_Vtbl {
            base__: <IWbemObjectSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteMessage: WriteMessage::<Identity, Impl, OFFSET>,
            WriteError: WriteError::<Identity, Impl, OFFSET>,
            PromptUser: PromptUser::<Identity, Impl, OFFSET>,
            WriteProgress: WriteProgress::<Identity, Impl, OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemObjectTextSrc_Impl: ::windows_core::BaseImpl {
    fn GetText(this: &Self::This, lflags: i32, pobj: ::core::option::Option<&IWbemClassObject>, uobjtextformat: u32, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateFromText(this: &Self::This, lflags: i32, strtext: &::windows_core::BSTR, uobjtextformat: u32, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IWbemClassObject>;
}
impl ::windows_core::Iids for IWbemObjectTextSrc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemObjectTextSrc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pobj: *mut ::core::ffi::c_void, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, strtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pobj), ::core::mem::transmute_copy(&uobjtextformat), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFromText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::std::mem::MaybeUninit<::windows_core::BSTR>, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, pnewobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFromText(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&strtext), ::core::mem::transmute_copy(&uobjtextformat), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnewobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemObjectTextSrc_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetText: GetText::<Identity, Impl, OFFSET>,
            CreateFromText: CreateFromText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemPath_Impl: ::windows_core::BaseImpl {
    fn SetText(this: &Self::This, umode: u32, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetInfo(this: &Self::This, urequestedinfo: u32) -> ::windows_core::Result<u64>;
    fn SetServer(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetServer(this: &Self::This, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetNamespaceCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNamespaceAt(this: &Self::This, uindex: u32, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetNamespaceAt(this: &Self::This, uindex: u32, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn RemoveNamespaceAt(this: &Self::This, uindex: u32) -> ::windows_core::Result<()>;
    fn RemoveAllNamespaces(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetScopeCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetScope(this: &Self::This, uindex: u32, pszclass: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetScopeFromText(this: &Self::This, uindex: u32, psztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetScope(this: &Self::This, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows_core::PWSTR, pkeylist: *mut ::core::option::Option<IWbemPathKeyList>) -> ::windows_core::Result<()>;
    fn GetScopeAsText(this: &Self::This, uindex: u32, putextbufsize: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn RemoveScope(this: &Self::This, uindex: u32) -> ::windows_core::Result<()>;
    fn RemoveAllScopes(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetClassName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetClassName(this: &Self::This, pubufflength: *mut u32, pszname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetKeyList(this: &Self::This) -> ::windows_core::Result<IWbemPathKeyList>;
    fn CreateClassPart(this: &Self::This, lflags: i32, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteClassPart(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn IsRelative(this: &Self::This, wszmachine: &::windows_core::PCWSTR, wsznamespace: &::windows_core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsRelativeOrChild(this: &Self::This, wszmachine: &::windows_core::PCWSTR, wsznamespace: &::windows_core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL;
    fn IsLocal(this: &Self::This, wszmachine: &::windows_core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsSameClassName(this: &Self::This, wszclass: &::windows_core::PCWSTR) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWbemPath {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemPath {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, umode: u32, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, ::core::mem::transmute_copy(&umode), ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&psztext)).into())
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInfo(this, ::core::mem::transmute_copy(&urequestedinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServer(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn GetServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetServer(this, ::core::mem::transmute_copy(&punamebuflength), ::core::mem::transmute_copy(&pname)).into())
        }
        unsafe extern "system" fn GetNamespaceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamespaceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamespaceAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamespaceAt(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute(&pszname)).into())
        }
        unsafe extern "system" fn GetNamespaceAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamespaceAt(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&punamebuflength), ::core::mem::transmute_copy(&pname)).into())
        }
        unsafe extern "system" fn RemoveNamespaceAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNamespaceAt(this, ::core::mem::transmute_copy(&uindex)).into())
        }
        unsafe extern "system" fn RemoveAllNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllNamespaces(this).into())
        }
        unsafe extern "system" fn GetScopeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScopeCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScope(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute(&pszclass)).into())
        }
        unsafe extern "system" fn SetScopeFromText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScopeFromText(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute(&psztext)).into())
        }
        unsafe extern "system" fn GetScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows_core::PWSTR, pkeylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScope(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&puclassnamebufsize), ::core::mem::transmute_copy(&pszclass), ::core::mem::transmute_copy(&pkeylist)).into())
        }
        unsafe extern "system" fn GetScopeAsText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScopeAsText(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&putextbufsize), ::core::mem::transmute_copy(&psztext)).into())
        }
        unsafe extern "system" fn RemoveScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveScope(this, ::core::mem::transmute_copy(&uindex)).into())
        }
        unsafe extern "system" fn RemoveAllScopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllScopes(this).into())
        }
        unsafe extern "system" fn SetClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn GetClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassName(this, ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&pszname)).into())
        }
        unsafe extern "system" fn GetKeyList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeyList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateClassPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClassPart(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn DeleteClassPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteClassPart(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn IsRelative<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmachine: ::windows_core::PCWSTR, wsznamespace: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRelative(this, ::core::mem::transmute(&wszmachine), ::core::mem::transmute(&wsznamespace)))
        }
        unsafe extern "system" fn IsRelativeOrChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmachine: ::windows_core::PCWSTR, wsznamespace: ::windows_core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRelativeOrChild(this, ::core::mem::transmute(&wszmachine), ::core::mem::transmute(&wsznamespace), ::core::mem::transmute_copy(&lflags)))
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmachine: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLocal(this, ::core::mem::transmute(&wszmachine)))
        }
        unsafe extern "system" fn IsSameClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszclass: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSameClassName(this, ::core::mem::transmute(&wszclass)))
        }
        IWbemPath_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            GetServer: GetServer::<Identity, Impl, OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Identity, Impl, OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Identity, Impl, OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Identity, Impl, OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Identity, Impl, OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Identity, Impl, OFFSET>,
            GetScopeCount: GetScopeCount::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            SetScopeFromText: SetScopeFromText::<Identity, Impl, OFFSET>,
            GetScope: GetScope::<Identity, Impl, OFFSET>,
            GetScopeAsText: GetScopeAsText::<Identity, Impl, OFFSET>,
            RemoveScope: RemoveScope::<Identity, Impl, OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Identity, Impl, OFFSET>,
            SetClassName: SetClassName::<Identity, Impl, OFFSET>,
            GetClassName: GetClassName::<Identity, Impl, OFFSET>,
            GetKeyList: GetKeyList::<Identity, Impl, OFFSET>,
            CreateClassPart: CreateClassPart::<Identity, Impl, OFFSET>,
            DeleteClassPart: DeleteClassPart::<Identity, Impl, OFFSET>,
            IsRelative: IsRelative::<Identity, Impl, OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            IsSameClassName: IsSameClassName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemPathKeyList_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetKey(this: &Self::This, wszname: &::windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetKey2(this: &Self::This, wszname: &::windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetKey(this: &Self::This, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows_core::Result<()>;
    fn GetKey2(this: &Self::This, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pkeyvalue: *mut super::Variant::VARIANT, puapparentcimtype: *mut u32) -> ::windows_core::Result<()>;
    fn RemoveKey(this: &Self::This, wszname: &::windows_core::PCWSTR, uflags: u32) -> ::windows_core::Result<()>;
    fn RemoveAllKeys(this: &Self::This, uflags: u32) -> ::windows_core::Result<()>;
    fn MakeSingleton(this: &Self::This, bset: u8) -> ::windows_core::Result<()>;
    fn GetInfo(this: &Self::This, urequestedinfo: u32) -> ::windows_core::Result<u64>;
    fn GetText(this: &Self::This, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemPathKeyList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemPathKeyList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pukeycount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKey(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ucimtype), ::core::mem::transmute_copy(&pkeyval)).into())
        }
        unsafe extern "system" fn SetKey2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKey2(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ucimtype), ::core::mem::transmute_copy(&pkeyval)).into())
        }
        unsafe extern "system" fn GetKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKey(this, ::core::mem::transmute_copy(&ukeyix), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&punamebufsize), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&pukeyvalbufsize), ::core::mem::transmute_copy(&pkeyval), ::core::mem::transmute_copy(&puapparentcimtype)).into())
        }
        unsafe extern "system" fn GetKey2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pkeyvalue: *mut super::Variant::VARIANT, puapparentcimtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKey2(this, ::core::mem::transmute_copy(&ukeyix), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&punamebufsize), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&pkeyvalue), ::core::mem::transmute_copy(&puapparentcimtype)).into())
        }
        unsafe extern "system" fn RemoveKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, uflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveKey(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&uflags)).into())
        }
        unsafe extern "system" fn RemoveAllKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllKeys(this, ::core::mem::transmute_copy(&uflags)).into())
        }
        unsafe extern "system" fn MakeSingleton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeSingleton(this, ::core::mem::transmute_copy(&bset)).into())
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInfo(this, ::core::mem::transmute_copy(&urequestedinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&psztext)).into())
        }
        IWbemPathKeyList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            SetKey: SetKey::<Identity, Impl, OFFSET>,
            SetKey2: SetKey2::<Identity, Impl, OFFSET>,
            GetKey: GetKey::<Identity, Impl, OFFSET>,
            GetKey2: GetKey2::<Identity, Impl, OFFSET>,
            RemoveKey: RemoveKey::<Identity, Impl, OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Identity, Impl, OFFSET>,
            MakeSingleton: MakeSingleton::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemPropertyProvider_Impl: ::windows_core::BaseImpl {
    fn GetProperty(this: &Self::This, lflags: i32, strlocale: &::windows_core::BSTR, strclassmapping: &::windows_core::BSTR, strinstmapping: &::windows_core::BSTR, strpropmapping: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn PutProperty(this: &Self::This, lflags: i32, strlocale: &::windows_core::BSTR, strclassmapping: &::windows_core::BSTR, strinstmapping: &::windows_core::BSTR, strpropmapping: &::windows_core::BSTR, pvvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemPropertyProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemPropertyProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, strclassmapping: ::std::mem::MaybeUninit<::windows_core::BSTR>, strinstmapping: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpropmapping: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&strlocale), ::core::mem::transmute(&strclassmapping), ::core::mem::transmute(&strinstmapping), ::core::mem::transmute(&strpropmapping)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::std::mem::MaybeUninit<::windows_core::BSTR>, strclassmapping: ::std::mem::MaybeUninit<::windows_core::BSTR>, strinstmapping: ::std::mem::MaybeUninit<::windows_core::BSTR>, strpropmapping: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutProperty(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&strlocale), ::core::mem::transmute(&strclassmapping), ::core::mem::transmute(&strinstmapping), ::core::mem::transmute(&strpropmapping), ::core::mem::transmute_copy(&pvvalue)).into())
        }
        IWbemPropertyProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            PutProperty: PutProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemProviderIdentity_Impl: ::windows_core::BaseImpl {
    fn SetRegistrationObject(this: &Self::This, lflags: i32, pprovreg: ::core::option::Option<&IWbemClassObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemProviderIdentity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemProviderIdentity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemProviderIdentity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRegistrationObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemProviderIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRegistrationObject(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pprovreg)).into())
        }
        IWbemProviderIdentity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRegistrationObject: SetRegistrationObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemProviderInit_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, wszuser: &::windows_core::PCWSTR, lflags: i32, wsznamespace: &::windows_core::PCWSTR, wszlocale: &::windows_core::PCWSTR, pnamespace: ::core::option::Option<&IWbemServices>, pctx: ::core::option::Option<&IWbemContext>, pinitsink: ::core::option::Option<&IWbemProviderInitSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemProviderInit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemProviderInit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemProviderInit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemProviderInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuser: ::windows_core::PCWSTR, lflags: i32, wsznamespace: ::windows_core::PCWSTR, wszlocale: ::windows_core::PCWSTR, pnamespace: *mut ::core::ffi::c_void, pctx: *mut ::core::ffi::c_void, pinitsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&wszuser), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&wsznamespace), ::core::mem::transmute(&wszlocale), ::windows_core::from_raw_borrowed(&pnamespace), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&pinitsink)).into())
        }
        IWbemProviderInit_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemProviderInitSink_Impl: ::windows_core::BaseImpl {
    fn SetStatus(this: &Self::This, lstatus: i32, lflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemProviderInitSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemProviderInitSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemProviderInitSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemProviderInitSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lflags)).into())
        }
        IWbemProviderInitSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetStatus: SetStatus::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemQualifierSet_Impl: ::windows_core::BaseImpl {
    fn Get(this: &Self::This, wszname: &::windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> ::windows_core::Result<()>;
    fn Put(this: &Self::This, wszname: &::windows_core::PCWSTR, pval: *const super::Variant::VARIANT, lflavor: i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, wszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetNames(this: &Self::This, lflags: i32) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, lflags: i32, pstrname: *mut ::windows_core::BSTR, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> ::windows_core::Result<()>;
    fn EndEnumeration(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWbemQualifierSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemQualifierSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&plflavor)).into())
        }
        unsafe extern "system" fn Put<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, pval: *const super::Variant::VARIANT, lflavor: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Put(this, ::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&lflavor)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&wszname)).into())
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNames(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEnumeration(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&plflavor)).into())
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEnumeration(this).into())
        }
        IWbemQualifierSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemQuery_Impl: ::windows_core::BaseImpl {
    fn Empty(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetLanguageFeatures(this: &Self::This, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows_core::Result<()>;
    fn TestLanguageFeatures(this: &Self::This, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows_core::Result<()>;
    fn Parse(this: &Self::This, pszlang: &::windows_core::PCWSTR, pszquery: &::windows_core::PCWSTR, uflags: u32) -> ::windows_core::Result<()>;
    fn GetAnalysis(this: &Self::This, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FreeMemory(this: &Self::This, pmem: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetQueryInfo(this: &Self::This, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Empty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Empty(this).into())
        }
        unsafe extern "system" fn SetLanguageFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguageFeatures(this, ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&uarraysize), ::core::mem::transmute_copy(&pufeatures)).into())
        }
        unsafe extern "system" fn TestLanguageFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TestLanguageFeatures(this, ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&uarraysize), ::core::mem::transmute_copy(&pufeatures)).into())
        }
        unsafe extern "system" fn Parse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlang: ::windows_core::PCWSTR, pszquery: ::windows_core::PCWSTR, uflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Parse(this, ::core::mem::transmute(&pszlang), ::core::mem::transmute(&pszquery), ::core::mem::transmute_copy(&uflags)).into())
        }
        unsafe extern "system" fn GetAnalysis<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAnalysis(this, ::core::mem::transmute_copy(&uanalysistype), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&panalysis)).into())
        }
        unsafe extern "system" fn FreeMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeMemory(this, ::core::mem::transmute_copy(&pmem)).into())
        }
        unsafe extern "system" fn GetQueryInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQueryInfo(this, ::core::mem::transmute_copy(&uanalysistype), ::core::mem::transmute_copy(&uinfoid), ::core::mem::transmute_copy(&ubufsize), ::core::mem::transmute_copy(&pdestbuf)).into())
        }
        IWbemQuery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Empty: Empty::<Identity, Impl, OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Identity, Impl, OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Identity, Impl, OFFSET>,
            Parse: Parse::<Identity, Impl, OFFSET>,
            GetAnalysis: GetAnalysis::<Identity, Impl, OFFSET>,
            FreeMemory: FreeMemory::<Identity, Impl, OFFSET>,
            GetQueryInfo: GetQueryInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemRefresher_Impl: ::windows_core::BaseImpl {
    fn Refresh(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemRefresher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemRefresher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemRefresher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemRefresher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        IWbemRefresher_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Refresh: Refresh::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemServices_Impl: ::windows_core::BaseImpl {
    fn OpenNamespace(this: &Self::This, strnamespace: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, ppworkingnamespace: *mut ::core::option::Option<IWbemServices>, ppresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn CancelAsyncCall(this: &Self::This, psink: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn QueryObjectSink(this: &Self::This, lflags: WBEM_GENERIC_FLAG_TYPE) -> ::windows_core::Result<IWbemObjectSink>;
    fn GetObject(this: &Self::This, strobjectpath: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, ppobject: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn GetObjectAsync(this: &Self::This, strobjectpath: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn PutClass(this: &Self::This, pobject: ::core::option::Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn PutClassAsync(this: &Self::This, pobject: ::core::option::Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn DeleteClass(this: &Self::This, strclass: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn DeleteClassAsync(this: &Self::This, strclass: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn CreateClassEnum(this: &Self::This, strsuperclass: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(this: &Self::This, strsuperclass: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn PutInstance(this: &Self::This, pinst: ::core::option::Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn PutInstanceAsync(this: &Self::This, pinst: ::core::option::Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn DeleteInstance(this: &Self::This, strobjectpath: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn DeleteInstanceAsync(this: &Self::This, strobjectpath: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn CreateInstanceEnum(this: &Self::This, strfilter: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(this: &Self::This, strfilter: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn ExecQuery(this: &Self::This, strquerylanguage: &::windows_core::BSTR, strquery: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(this: &Self::This, strquerylanguage: &::windows_core::BSTR, strquery: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn ExecNotificationQuery(this: &Self::This, strquerylanguage: &::windows_core::BSTR, strquery: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(this: &Self::This, strquerylanguage: &::windows_core::BSTR, strquery: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
    fn ExecMethod(this: &Self::This, strobjectpath: &::windows_core::BSTR, strmethodname: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, pinparams: ::core::option::Option<&IWbemClassObject>, ppoutparams: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()>;
    fn ExecMethodAsync(this: &Self::This, strobjectpath: &::windows_core::BSTR, strmethodname: &::windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: ::core::option::Option<&IWbemContext>, pinparams: ::core::option::Option<&IWbemClassObject>, presponsehandler: ::core::option::Option<&IWbemObjectSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespace: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppworkingnamespace: *mut *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenNamespace(this, ::core::mem::transmute(&strnamespace), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&ppworkingnamespace), ::core::mem::transmute_copy(&ppresult)).into())
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncCall(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn QueryObjectSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, ppresponsehandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryObjectSink(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresponsehandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&ppobject), ::core::mem::transmute_copy(&ppcallresult)).into())
        }
        unsafe extern "system" fn GetObjectAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectAsync(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn PutClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutClass(this, ::windows_core::from_raw_borrowed(&pobject), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into())
        }
        unsafe extern "system" fn PutClassAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutClassAsync(this, ::windows_core::from_raw_borrowed(&pobject), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn DeleteClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteClass(this, ::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into())
        }
        unsafe extern "system" fn DeleteClassAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteClassAsync(this, ::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn CreateClassEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateClassEnum(this, ::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateClassEnumAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClassEnumAsync(this, ::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn PutInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinst: *mut ::core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutInstance(this, ::windows_core::from_raw_borrowed(&pinst), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into())
        }
        unsafe extern "system" fn PutInstanceAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinst: *mut ::core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutInstanceAsync(this, ::windows_core::from_raw_borrowed(&pinst), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn DeleteInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteInstance(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into())
        }
        unsafe extern "system" fn DeleteInstanceAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteInstanceAsync(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn CreateInstanceEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstanceEnum(this, ::core::mem::transmute(&strfilter), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstanceEnumAsync(this, ::core::mem::transmute(&strfilter), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn ExecQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecQuery(this, ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecQueryAsync(this, ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecNotificationQuery(this, ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecNotificationQueryAsync(this, ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        unsafe extern "system" fn ExecMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, ppoutparams: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecMethod(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&pinparams), ::core::mem::transmute_copy(&ppoutparams), ::core::mem::transmute_copy(&ppcallresult)).into())
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecMethodAsync(this, ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pctx), ::windows_core::from_raw_borrowed(&pinparams), ::windows_core::from_raw_borrowed(&presponsehandler)).into())
        }
        IWbemServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenNamespace: OpenNamespace::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
            QueryObjectSink: QueryObjectSink::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectAsync: GetObjectAsync::<Identity, Impl, OFFSET>,
            PutClass: PutClass::<Identity, Impl, OFFSET>,
            PutClassAsync: PutClassAsync::<Identity, Impl, OFFSET>,
            DeleteClass: DeleteClass::<Identity, Impl, OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Identity, Impl, OFFSET>,
            CreateClassEnum: CreateClassEnum::<Identity, Impl, OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Identity, Impl, OFFSET>,
            PutInstance: PutInstance::<Identity, Impl, OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Identity, Impl, OFFSET>,
            DeleteInstance: DeleteInstance::<Identity, Impl, OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Identity, Impl, OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Identity, Impl, OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Identity, Impl, OFFSET>,
            ExecQuery: ExecQuery::<Identity, Impl, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, Impl, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, Impl, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, Impl, OFFSET>,
            ExecMethod: ExecMethod::<Identity, Impl, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemShutdown_Impl: ::windows_core::BaseImpl {
    fn Shutdown(this: &Self::This, ureason: i32, umaxmilliseconds: u32, pctx: ::core::option::Option<&IWbemContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemShutdown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemShutdown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemShutdown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemShutdown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this, ::core::mem::transmute_copy(&ureason), ::core::mem::transmute_copy(&umaxmilliseconds), ::windows_core::from_raw_borrowed(&pctx)).into())
        }
        IWbemShutdown_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Shutdown: Shutdown::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemStatusCodeText_Impl: ::windows_core::BaseImpl {
    fn GetErrorCodeText(this: &Self::This, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFacilityCodeText(this: &Self::This, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IWbemStatusCodeText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemStatusCodeText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetErrorCodeText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorCodeText(this, ::core::mem::transmute_copy(&hres), ::core::mem::transmute_copy(&localeid), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFacilityCodeText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFacilityCodeText(this, ::core::mem::transmute_copy(&hres), ::core::mem::transmute_copy(&localeid), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemStatusCodeText_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetErrorCodeText: GetErrorCodeText::<Identity, Impl, OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemTransport_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemTransport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemTransport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemTransport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemTransport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        IWbemTransport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemUnboundObjectSink_Impl: ::windows_core::BaseImpl {
    fn IndicateToConsumer(this: &Self::This, plogicalconsumer: ::core::option::Option<&IWbemClassObject>, lnumobjects: i32, apobjects: *const ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWbemUnboundObjectSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemUnboundObjectSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemUnboundObjectSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IndicateToConsumer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemUnboundObjectSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogicalconsumer: *mut ::core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IndicateToConsumer(this, ::windows_core::from_raw_borrowed(&plogicalconsumer), ::core::mem::transmute_copy(&lnumobjects), ::core::mem::transmute_copy(&apobjects)).into())
        }
        IWbemUnboundObjectSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IndicateToConsumer: IndicateToConsumer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWbemUnsecuredApartment_Impl: ::windows_core::BaseImpl + IUnsecuredApartment_Impl {
    fn CreateSinkStub(this: &Self::This, psink: ::core::option::Option<&IWbemObjectSink>, dwflags: u32, wszreserved: &::windows_core::PCWSTR) -> ::windows_core::Result<IWbemObjectSink>;
}
impl ::windows_core::Iids for IWbemUnsecuredApartment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUnsecuredApartment);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemUnsecuredApartment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWbemUnsecuredApartment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSinkStub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWbemUnsecuredApartment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, dwflags: u32, wszreserved: ::windows_core::PCWSTR, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSinkStub(this, ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&wszreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWbemUnsecuredApartment_Vtbl { base__: <IUnsecuredApartment as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateSinkStub: CreateSinkStub::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
