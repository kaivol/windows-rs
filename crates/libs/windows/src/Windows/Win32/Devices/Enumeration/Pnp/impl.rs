pub trait IUPnPAddressFamilyControl_Impl: ::windows_core::BaseImpl {
    fn SetAddressFamily(this: &Self::This, dwflags: i32) -> ::windows_core::Result<()>;
    fn GetAddressFamily(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IUPnPAddressFamilyControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPAddressFamilyControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddressFamily(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetAddressFamily<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAddressFamily(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPAddressFamilyControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            GetAddressFamily: GetAddressFamily::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPAsyncResult_Impl: ::windows_core::BaseImpl {
    fn AsyncOperationComplete(this: &Self::This, ullrequestid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPAsyncResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPAsyncResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPAsyncResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncOperationComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncOperationComplete(this, ::core::mem::transmute_copy(&ullrequestid)).into())
        }
        IUPnPAsyncResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncOperationComplete: AsyncOperationComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDescriptionDocument_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn ReadyState(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Load(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoadAsync(this: &Self::This, bstrurl: &::windows_core::BSTR, punkcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn LoadResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn RootDevice(this: &Self::This) -> ::windows_core::Result<IUPnPDevice>;
    fn DeviceByUDN(this: &Self::This, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPDescriptionDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDescriptionDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plreadystate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::core::mem::transmute(&bstrurl)).into())
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadAsync(this, ::core::mem::transmute(&bstrurl), ::windows_core::from_raw_borrowed(&punkcallback)).into())
        }
        unsafe extern "system" fn LoadResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn RootDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudrootdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceByUDN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppuddevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceByUDN(this, ::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuddevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDescriptionDocument_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadyState: ReadyState::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            LoadResult: LoadResult::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            RootDevice: RootDevice::<Identity, Impl, OFFSET>,
            DeviceByUDN: DeviceByUDN::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPDescriptionDocumentCallback_Impl: ::windows_core::BaseImpl {
    fn LoadComplete(this: &Self::This, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPDescriptionDocumentCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDescriptionDocumentCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadComplete(this, ::core::mem::transmute_copy(&hrloadresult)).into())
        }
        IUPnPDescriptionDocumentCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LoadComplete: LoadComplete::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDevice_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn IsRootDevice(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn RootDevice(this: &Self::This) -> ::windows_core::Result<IUPnPDevice>;
    fn ParentDevice(this: &Self::This) -> ::windows_core::Result<IUPnPDevice>;
    fn HasChildren(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Children(this: &Self::This) -> ::windows_core::Result<IUPnPDevices>;
    fn UniqueDeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PresentationURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ManufacturerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ManufacturerURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModelName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModelNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModelURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UPC(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SerialNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IconURL(this: &Self::This, bstrencodingformat: &::windows_core::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Services(this: &Self::This) -> ::windows_core::Result<IUPnPServices>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsRootDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRootDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RootDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudrootdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParentDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParentDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuddeviceparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppudchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UniqueDeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UniqueDeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PresentationURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PresentationURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ManufacturerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ManufacturerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ManufacturerURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ManufacturerURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UPC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UPC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SerialNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SerialNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IconURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrencodingformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IconURL(this, ::core::mem::transmute(&bstrencodingformat), ::core::mem::transmute_copy(&lsizex), ::core::mem::transmute_copy(&lsizey), ::core::mem::transmute_copy(&lbitdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstriconurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Services<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppusservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Services(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppusservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDevice_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsRootDevice: IsRootDevice::<Identity, Impl, OFFSET>,
            RootDevice: RootDevice::<Identity, Impl, OFFSET>,
            ParentDevice: ParentDevice::<Identity, Impl, OFFSET>,
            HasChildren: HasChildren::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            UniqueDeviceName: UniqueDeviceName::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            PresentationURL: PresentationURL::<Identity, Impl, OFFSET>,
            ManufacturerName: ManufacturerName::<Identity, Impl, OFFSET>,
            ManufacturerURL: ManufacturerURL::<Identity, Impl, OFFSET>,
            ModelName: ModelName::<Identity, Impl, OFFSET>,
            ModelNumber: ModelNumber::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            ModelURL: ModelURL::<Identity, Impl, OFFSET>,
            UPC: UPC::<Identity, Impl, OFFSET>,
            SerialNumber: SerialNumber::<Identity, Impl, OFFSET>,
            IconURL: IconURL::<Identity, Impl, OFFSET>,
            Services: Services::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceControl_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, bstrxmldesc: &::windows_core::BSTR, bstrdeviceidentifier: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetServiceObject(this: &Self::This, bstrudn: &::windows_core::BSTR, bstrserviceid: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUPnPDeviceControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrinitstring)).into())
        }
        unsafe extern "system" fn GetServiceObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdispservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceObject(this, ::core::mem::transmute(&bstrudn), ::core::mem::transmute(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDeviceControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetServiceObject: GetServiceObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPDeviceControlHttpHeaders_Impl: ::windows_core::BaseImpl {
    fn GetAdditionalResponseHeaders(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IUPnPDeviceControlHttpHeaders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceControlHttpHeaders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAdditionalResponseHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrhttpresponseheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDeviceControlHttpHeaders_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAdditionalResponseHeaders: GetAdditionalResponseHeaders::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPDeviceDocumentAccess_Impl: ::windows_core::BaseImpl {
    fn GetDocumentURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IUPnPDeviceDocumentAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceDocumentAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDeviceDocumentAccess_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDocumentURL: GetDocumentURL::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPDeviceDocumentAccessEx_Impl: ::windows_core::BaseImpl {
    fn GetDocument(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IUPnPDeviceDocumentAccessEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceDocumentAccessEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDeviceDocumentAccessEx_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDocument: GetDocument::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDeviceFinder_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn FindByType(this: &Self::This, bstrtypeuri: &::windows_core::BSTR, dwflags: u32) -> ::windows_core::Result<IUPnPDevices>;
    fn CreateAsyncFind(this: &Self::This, bstrtypeuri: &::windows_core::BSTR, dwflags: u32, punkdevicefindercallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<i32>;
    fn StartAsyncFind(this: &Self::This, lfinddata: i32) -> ::windows_core::Result<()>;
    fn CancelAsyncFind(this: &Self::This, lfinddata: i32) -> ::windows_core::Result<()>;
    fn FindByUDN(this: &Self::This, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPDeviceFinder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceFinder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32, pdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindByType(this, ::core::mem::transmute(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAsyncFind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAsyncFind(this, ::core::mem::transmute(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&punkdevicefindercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfinddata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartAsyncFind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartAsyncFind(this, ::core::mem::transmute_copy(&lfinddata)).into())
        }
        unsafe extern "system" fn CancelAsyncFind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncFind(this, ::core::mem::transmute_copy(&lfinddata)).into())
        }
        unsafe extern "system" fn FindByUDN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindByUDN(this, ::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDeviceFinder_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindByType: FindByType::<Identity, Impl, OFFSET>,
            CreateAsyncFind: CreateAsyncFind::<Identity, Impl, OFFSET>,
            StartAsyncFind: StartAsyncFind::<Identity, Impl, OFFSET>,
            CancelAsyncFind: CancelAsyncFind::<Identity, Impl, OFFSET>,
            FindByUDN: FindByUDN::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderAddCallbackWithInterface_Impl: ::windows_core::BaseImpl {
    fn DeviceAddedWithInterface(this: &Self::This, lfinddata: i32, pdevice: ::core::option::Option<&IUPnPDevice>, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUPnPDeviceFinderAddCallbackWithInterface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceFinderAddCallbackWithInterface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceAddedWithInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceAddedWithInterface(this, ::core::mem::transmute_copy(&lfinddata), ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pguidinterface)).into())
        }
        IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceAddedWithInterface: DeviceAddedWithInterface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderCallback_Impl: ::windows_core::BaseImpl {
    fn DeviceAdded(this: &Self::This, lfinddata: i32, pdevice: ::core::option::Option<&IUPnPDevice>) -> ::windows_core::Result<()>;
    fn DeviceRemoved(this: &Self::This, lfinddata: i32, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SearchComplete(this: &Self::This, lfinddata: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUPnPDeviceFinderCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceFinderCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceAdded(this, ::core::mem::transmute_copy(&lfinddata), ::windows_core::from_raw_borrowed(&pdevice)).into())
        }
        unsafe extern "system" fn DeviceRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceRemoved(this, ::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&bstrudn)).into())
        }
        unsafe extern "system" fn SearchComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SearchComplete(this, ::core::mem::transmute_copy(&lfinddata)).into())
        }
        IUPnPDeviceFinderCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceAdded: DeviceAdded::<Identity, Impl, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, Impl, OFFSET>,
            SearchComplete: SearchComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPDeviceProvider_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This, bstrinitstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPDeviceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDeviceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute(&bstrinitstring)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        IUPnPDeviceProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDevices_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPDevices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPDevices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPDevices_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPEventSink_Impl: ::windows_core::BaseImpl {
    fn OnStateChanged(this: &Self::This, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_core::Result<()>;
    fn OnStateChangedSafe(this: &Self::This, varsadispidchanges: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStateChanged(this, ::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&rgdispidchanges)).into())
        }
        unsafe extern "system" fn OnStateChangedSafe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsadispidchanges: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStateChangedSafe(this, ::core::mem::transmute(&varsadispidchanges)).into())
        }
        IUPnPEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnStateChangedSafe: OnStateChangedSafe::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPEventSource_Impl: ::windows_core::BaseImpl {
    fn Advise(this: &Self::This, pessubscriber: ::core::option::Option<&IUPnPEventSink>) -> ::windows_core::Result<()>;
    fn Unadvise(this: &Self::This, pessubscriber: ::core::option::Option<&IUPnPEventSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPEventSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPEventSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Advise(this, ::windows_core::from_raw_borrowed(&pessubscriber)).into())
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::windows_core::from_raw_borrowed(&pessubscriber)).into())
        }
        IUPnPEventSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPHttpHeaderControl_Impl: ::windows_core::BaseImpl {
    fn AddRequestHeaders(this: &Self::This, bstrhttpheaders: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPHttpHeaderControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPHttpHeaderControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRequestHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRequestHeaders(this, ::core::mem::transmute(&bstrhttpheaders)).into())
        }
        IUPnPHttpHeaderControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRequestHeaders: AddRequestHeaders::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPRegistrar_Impl: ::windows_core::BaseImpl {
    fn RegisterDevice(this: &Self::This, bstrxmldesc: &::windows_core::BSTR, bstrprogiddevicecontrolclass: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR, bstrcontainerid: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterRunningDevice(this: &Self::This, bstrxmldesc: &::windows_core::BSTR, punkdevicecontrol: ::core::option::Option<&::windows_core::IUnknown>, bstrinitstring: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterDeviceProvider(this: &Self::This, bstrprovidername: &::windows_core::BSTR, bstrprogidproviderclass: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR, bstrcontainerid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetUniqueDeviceName(this: &Self::This, bstrdeviceidentifier: &::windows_core::BSTR, bstrtemplateudn: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UnregisterDevice(this: &Self::This, bstrdeviceidentifier: &::windows_core::BSTR, fpermanent: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UnregisterDeviceProvider(this: &Self::This, bstrprovidername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUPnPRegistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPRegistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogiddevicecontrolclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterDevice(this, ::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrprogiddevicecontrolclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterRunningDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterRunningDevice(this, ::core::mem::transmute(&bstrxmldesc), ::windows_core::from_raw_borrowed(&punkdevicecontrol), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogidproviderclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterDeviceProvider(this, ::core::mem::transmute(&bstrprovidername), ::core::mem::transmute(&bstrprogidproviderclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid)).into())
        }
        unsafe extern "system" fn GetUniqueDeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtemplateudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrudn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUniqueDeviceName(this, ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrtemplateudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrudn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDevice(this, ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute_copy(&fpermanent)).into())
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDeviceProvider(this, ::core::mem::transmute(&bstrprovidername)).into())
        }
        IUPnPRegistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterDevice: RegisterDevice::<Identity, Impl, OFFSET>,
            RegisterRunningDevice: RegisterRunningDevice::<Identity, Impl, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, Impl, OFFSET>,
            GetUniqueDeviceName: GetUniqueDeviceName::<Identity, Impl, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, Impl, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPRemoteEndpointInfo_Impl: ::windows_core::BaseImpl {
    fn GetDwordValue(this: &Self::This, bstrvaluename: &::windows_core::BSTR) -> ::windows_core::Result<u32>;
    fn GetStringValue(this: &Self::This, bstrvaluename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetGuidValue(this: &Self::This, bstrvaluename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IUPnPRemoteEndpointInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPRemoteEndpointInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDwordValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDwordValue(this, ::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringValue(this, ::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGuidValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pguidvalue: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGuidValue(this, ::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPRemoteEndpointInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDwordValue: GetDwordValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPReregistrar_Impl: ::windows_core::BaseImpl {
    fn ReregisterDevice(this: &Self::This, bstrdeviceidentifier: &::windows_core::BSTR, bstrxmldesc: &::windows_core::BSTR, bstrprogiddevicecontrolclass: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR, bstrcontainerid: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<()>;
    fn ReregisterRunningDevice(this: &Self::This, bstrdeviceidentifier: &::windows_core::BSTR, bstrxmldesc: &::windows_core::BSTR, punkdevicecontrol: ::core::option::Option<&::windows_core::IUnknown>, bstrinitstring: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPReregistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPReregistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReregisterDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogiddevicecontrolclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReregisterDevice(this, ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrprogiddevicecontrolclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into())
        }
        unsafe extern "system" fn ReregisterRunningDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReregisterRunningDevice(this, ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrxmldesc), ::windows_core::from_raw_borrowed(&punkdevicecontrol), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into())
        }
        IUPnPReregistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReregisterDevice: ReregisterDevice::<Identity, Impl, OFFSET>,
            ReregisterRunningDevice: ReregisterRunningDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPService_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn QueryStateVariable(this: &Self::This, bstrvariablename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn InvokeAction(this: &Self::This, bstractionname: &::windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ServiceTypeIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddCallback(this: &Self::This, punkcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastTransportStatus(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryStateVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryStateVariable(this, ::core::mem::transmute(&bstrvariablename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InvokeAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstractionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vinactionargs: super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeAction(this, ::core::mem::transmute(&bstractionname), ::core::mem::transmute(&vinactionargs), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into())
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceTypeIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddCallback(this, ::windows_core::from_raw_borrowed(&punkcallback)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastTransportStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastTransportStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPService_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryStateVariable: QueryStateVariable::<Identity, Impl, OFFSET>,
            InvokeAction: InvokeAction::<Identity, Impl, OFFSET>,
            ServiceTypeIdentifier: ServiceTypeIdentifier::<Identity, Impl, OFFSET>,
            AddCallback: AddCallback::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            LastTransportStatus: LastTransportStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServiceAsync_Impl: ::windows_core::BaseImpl {
    fn BeginInvokeAction(this: &Self::This, bstractionname: &::windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndInvokeAction(this: &Self::This, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BeginQueryStateVariable(this: &Self::This, bstrvariablename: &::windows_core::BSTR, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndQueryStateVariable(this: &Self::This, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BeginSubscribeToEvents(this: &Self::This, punkcallback: ::core::option::Option<&::windows_core::IUnknown>, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndSubscribeToEvents(this: &Self::This, ullrequestid: u64) -> ::windows_core::Result<()>;
    fn BeginSCPDDownload(this: &Self::This, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndSCPDDownload(this: &Self::This, ullrequestid: u64) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CancelAsyncOperation(this: &Self::This, ullrequestid: u64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPServiceAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPServiceAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginInvokeAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstractionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vinactionargs: super::super::super::System::Variant::VARIANT, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginInvokeAction(this, ::core::mem::transmute(&bstractionname), ::core::mem::transmute(&vinactionargs), ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndInvokeAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndInvokeAction(this, ::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into())
        }
        unsafe extern "system" fn BeginQueryStateVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginQueryStateVariable(this, ::core::mem::transmute(&bstrvariablename), ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndQueryStateVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndQueryStateVariable(this, ::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginSubscribeToEvents(this, ::windows_core::from_raw_borrowed(&punkcallback), ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndSubscribeToEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSubscribeToEvents(this, ::core::mem::transmute_copy(&ullrequestid)).into())
        }
        unsafe extern "system" fn BeginSCPDDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginSCPDDownload(this, ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndSCPDDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndSCPDDownload(this, ::core::mem::transmute_copy(&ullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrscpddoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CancelAsyncOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncOperation(this, ::core::mem::transmute_copy(&ullrequestid)).into())
        }
        IUPnPServiceAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginInvokeAction: BeginInvokeAction::<Identity, Impl, OFFSET>,
            EndInvokeAction: EndInvokeAction::<Identity, Impl, OFFSET>,
            BeginQueryStateVariable: BeginQueryStateVariable::<Identity, Impl, OFFSET>,
            EndQueryStateVariable: EndQueryStateVariable::<Identity, Impl, OFFSET>,
            BeginSubscribeToEvents: BeginSubscribeToEvents::<Identity, Impl, OFFSET>,
            EndSubscribeToEvents: EndSubscribeToEvents::<Identity, Impl, OFFSET>,
            BeginSCPDDownload: BeginSCPDDownload::<Identity, Impl, OFFSET>,
            EndSCPDDownload: EndSCPDDownload::<Identity, Impl, OFFSET>,
            CancelAsyncOperation: CancelAsyncOperation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServiceCallback_Impl: ::windows_core::BaseImpl {
    fn StateVariableChanged(this: &Self::This, pus: ::core::option::Option<&IUPnPService>, pcwszstatevarname: &::windows_core::PCWSTR, vavalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ServiceInstanceDied(this: &Self::This, pus: ::core::option::Option<&IUPnPService>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPServiceCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPServiceCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StateVariableChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void, pcwszstatevarname: ::windows_core::PCWSTR, vavalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StateVariableChanged(this, ::windows_core::from_raw_borrowed(&pus), ::core::mem::transmute(&pcwszstatevarname), ::core::mem::transmute(&vavalue)).into())
        }
        unsafe extern "system" fn ServiceInstanceDied<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServiceInstanceDied(this, ::windows_core::from_raw_borrowed(&pus)).into())
        }
        IUPnPServiceCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StateVariableChanged: StateVariableChanged::<Identity, Impl, OFFSET>,
            ServiceInstanceDied: ServiceInstanceDied::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPServiceDocumentAccess_Impl: ::windows_core::BaseImpl {
    fn GetDocumentURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDocument(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IUPnPServiceDocumentAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPServiceDocumentAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdoc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPServiceDocumentAccess_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentURL: GetDocumentURL::<Identity, Impl, OFFSET>,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUPnPServiceEnumProperty_Impl: ::windows_core::BaseImpl {
    fn SetServiceEnumProperty(this: &Self::This, dwmask: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUPnPServiceEnumProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPServiceEnumProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetServiceEnumProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceEnumProperty(this, ::core::mem::transmute_copy(&dwmask)).into())
        }
        IUPnPServiceEnumProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetServiceEnumProperty: SetServiceEnumProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServices_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, bstrserviceid: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPService>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPServices_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
