#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2DataEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Update(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DDiscFormat2DataEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2DataEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DDiscFormat2DataEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2DataEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into())
        }
        DDiscFormat2DataEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2EraseEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Update(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DDiscFormat2EraseEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DDiscFormat2EraseEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&elapsedseconds), ::core::mem::transmute_copy(&estimatedtotalseconds)).into())
        }
        DDiscFormat2EraseEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2RawCDEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Update(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DDiscFormat2RawCDEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DDiscFormat2RawCDEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into())
        }
        DDiscFormat2RawCDEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Update(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DDiscFormat2TrackAtOnceEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DDiscFormat2TrackAtOnceEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into())
        }
        DDiscFormat2TrackAtOnceEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscMaster2Events_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn NotifyDeviceAdded(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, uniqueid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NotifyDeviceRemoved(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, uniqueid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DDiscMaster2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscMaster2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DDiscMaster2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifyDeviceAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscMaster2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyDeviceAdded(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute(&uniqueid)).into())
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DDiscMaster2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyDeviceRemoved(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute(&uniqueid)).into())
        }
        DDiscMaster2Events_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, Impl, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Update(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, currentfile: &::windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DFileSystemImageEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DFileSystemImageEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DFileSystemImageEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DFileSystemImageEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, currentfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute(&currentfile), ::core::mem::transmute_copy(&copiedsectors), ::core::mem::transmute_copy(&totalsectors)).into())
        }
        DFileSystemImageEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageImportEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn UpdateImport(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, filesystem: FsiFileSystems, currentitem: &::windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DFileSystemImageImportEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DFileSystemImageImportEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DFileSystemImageImportEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateImport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DFileSystemImageImportEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, currentitem: ::std::mem::MaybeUninit<::windows_core::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateImport(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&filesystem), ::core::mem::transmute(&currentitem), ::core::mem::transmute_copy(&importeddirectoryitems), ::core::mem::transmute_copy(&totaldirectoryitems), ::core::mem::transmute_copy(&importedfileitems), ::core::mem::transmute_copy(&totalfileitems)).into())
        }
        DFileSystemImageImportEvents_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateImport: UpdateImport::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DWriteEngine2Events_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Update(this: &Self::This, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DWriteEngine2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DWriteEngine2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DWriteEngine2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DWriteEngine2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into())
        }
        DWriteEngine2Events_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRange_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StartLba(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EndLba(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBlockRange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockRange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBlockRange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartLba<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartLba(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndLba<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndLba(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBlockRange_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartLba: StartLba::<Identity, Impl, OFFSET>,
            EndLba: EndLba::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRangeList_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn BlockRanges(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBlockRangeList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockRangeList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBlockRangeList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BlockRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockRangeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockRanges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBlockRangeList_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BlockRanges: BlockRanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBootOptions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn BootImage(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn Manufacturer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetManufacturer(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlatformId(this: &Self::This) -> ::windows_core::Result<PlatformId>;
    fn SetPlatformId(this: &Self::This, newval: PlatformId) -> ::windows_core::Result<()>;
    fn Emulation(this: &Self::This) -> ::windows_core::Result<EmulationType>;
    fn SetEmulation(this: &Self::This, newval: EmulationType) -> ::windows_core::Result<()>;
    fn ImageSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn AssignBootImage(this: &Self::This, newval: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBootOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBootOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BootImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BootImage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Manufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Manufacturer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManufacturer(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn PlatformId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlatformId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlatformId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlatformId(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Emulation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Emulation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEmulation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEmulation(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ImageSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AssignBootImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssignBootImage(this, ::windows_core::from_raw_borrowed(&newval)).into())
        }
        IBootOptions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BootImage: BootImage::<Identity, Impl, OFFSET>,
            Manufacturer: Manufacturer::<Identity, Impl, OFFSET>,
            SetManufacturer: SetManufacturer::<Identity, Impl, OFFSET>,
            PlatformId: PlatformId::<Identity, Impl, OFFSET>,
            SetPlatformId: SetPlatformId::<Identity, Impl, OFFSET>,
            Emulation: Emulation::<Identity, Impl, OFFSET>,
            SetEmulation: SetEmulation::<Identity, Impl, OFFSET>,
            ImageSize: ImageSize::<Identity, Impl, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBurnVerification_Impl: ::windows_core::BaseImpl {
    fn SetBurnVerificationLevel(this: &Self::This, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::Result<()>;
    fn BurnVerificationLevel(this: &Self::This) -> ::windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl ::windows_core::Iids for IBurnVerification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBurnVerification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBurnVerification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBurnVerificationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBurnVerification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBurnVerificationLevel(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn BurnVerificationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBurnVerification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BurnVerificationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBurnVerification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Identity, Impl, OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn IsRecorderSupported(this: &Self::This, recorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCurrentMediaSupported(this: &Self::This, recorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaPhysicallyBlank(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaHeuristicallyBlank(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedMediaTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsRecorderSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRecorderSupported(this, ::windows_core::from_raw_borrowed(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCurrentMediaSupported(this, ::windows_core::from_raw_borrowed(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaPhysicallyBlank(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaHeuristicallyBlank(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedMediaTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedMediaTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscFormat2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsRecorderSupported: IsRecorderSupported::<Identity, Impl, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, Impl, OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Identity, Impl, OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Identity, Impl, OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Data_Impl: ::windows_core::BaseImpl + IDiscFormat2_Impl {
    fn SetRecorder(this: &Self::This, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPostgapAlreadyInImage(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PostgapAlreadyInImage(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentMediaStatus(this: &Self::This) -> ::windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(this: &Self::This) -> ::windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FreeSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NextWritableAddress(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StartAddressOfPreviousSession(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetForceMediaToBeClosed(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ForceMediaToBeClosed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableConsumerDvdCompatibilityMode(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(this: &Self::This) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestedWriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetForceOverwrite(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ForceOverwrite(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MultisessionInterfaces(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Write(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CancelWrite(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetWriteSpeed(this: &Self::This, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2Data {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDiscFormat2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2Data {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecorder(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferUnderrunFreeDisabled(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferUnderrunFreeDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostgapAlreadyInImage(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostgapAlreadyInImage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentMediaStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentMediaStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteProtectStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteProtectStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextWritableAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextWritableAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartAddressOfPreviousSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastWrittenAddressOfPreviousSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForceMediaToBeClosed(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForceMediaToBeClosed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisableConsumerDvdCompatibilityMode(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisableConsumerDvdCompatibilityMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPhysicalMediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedWriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedRotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentWriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedWriteSpeeds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedWriteSpeedDescriptors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeddescriptors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForceOverwrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForceOverwrite(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ForceOverwrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForceOverwrite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MultisessionInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::windows_core::from_raw_borrowed(&data)).into())
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelWrite(this).into())
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWriteSpeed(this, ::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into())
        }
        IDiscFormat2Data_Vtbl {
            base__: <IDiscFormat2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Identity, Impl, OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Identity, Impl, OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Identity, Impl, OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, Impl, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Identity, Impl, OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Identity, Impl, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Identity, Impl, OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Identity, Impl, OFFSET>,
            ForceOverwrite: ForceOverwrite::<Identity, Impl, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2DataEventArgs_Impl: ::windows_core::BaseImpl + IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RemainingTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentAction(this: &Self::This) -> ::windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2DataEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWriteEngine2EventArgs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2DataEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ElapsedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElapsedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemainingTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscFormat2DataEventArgs_Vtbl {
            base__: <IWriteEngine2EventArgs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
            TotalTime: TotalTime::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Erase_Impl: ::windows_core::BaseImpl + IDiscFormat2_Impl {
    fn SetRecorder(this: &Self::This, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetFullErase(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FullErase(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(this: &Self::This) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EraseMedia(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2Erase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDiscFormat2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2Erase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecorder(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFullErase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFullErase(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn FullErase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FullErase(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPhysicalMediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EraseMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EraseMedia(this).into())
        }
        IDiscFormat2Erase_Vtbl {
            base__: <IDiscFormat2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetFullErase: SetFullErase::<Identity, Impl, OFFSET>,
            FullErase: FullErase::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            EraseMedia: EraseMedia::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCD_Impl: ::windows_core::BaseImpl + IDiscFormat2_Impl {
    fn PrepareMedia(this: &Self::This) -> ::windows_core::Result<()>;
    fn WriteMedia(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn WriteMedia2(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>, streamleadinsectors: i32) -> ::windows_core::Result<()>;
    fn CancelWrite(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReleaseMedia(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetWriteSpeed(this: &Self::This, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRecorder(this: &Self::This, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartOfNextSession(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastPossibleStartOfLeadout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentPhysicalMediaType(this: &Self::This) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetRequestedSectorType(this: &Self::This, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::Result<()>;
    fn RequestedSectorType(this: &Self::This) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestedWriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2RawCD {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDiscFormat2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2RawCD {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrepareMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareMedia(this).into())
        }
        unsafe extern "system" fn WriteMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteMedia(this, ::windows_core::from_raw_borrowed(&data)).into())
        }
        unsafe extern "system" fn WriteMedia2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, streamleadinsectors: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteMedia2(this, ::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&streamleadinsectors)).into())
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelWrite(this).into())
        }
        unsafe extern "system" fn ReleaseMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMedia(this).into())
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWriteSpeed(this, ::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into())
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecorder(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferUnderrunFreeDisabled(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferUnderrunFreeDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartOfNextSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartOfNextSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastPossibleStartOfLeadout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPhysicalMediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedSectorTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedSectorTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRequestedSectorType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestedSectorType(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn RequestedSectorType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedSectorType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedWriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedRotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentWriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedWriteSpeeds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedWriteSpeedDescriptors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeddescriptors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscFormat2RawCD_Vtbl {
            base__: <IDiscFormat2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrepareMedia: PrepareMedia::<Identity, Impl, OFFSET>,
            WriteMedia: WriteMedia::<Identity, Impl, OFFSET>,
            WriteMedia2: WriteMedia2::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            StartOfNextSession: StartOfNextSession::<Identity, Impl, OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Identity, Impl, OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Identity, Impl, OFFSET>,
            RequestedSectorType: RequestedSectorType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: ::windows_core::BaseImpl + IWriteEngine2EventArgs_Impl {
    fn CurrentAction(this: &Self::This) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RemainingTime(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2RawCDEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWriteEngine2EventArgs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2RawCDEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElapsedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElapsedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemainingTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscFormat2RawCDEventArgs_Vtbl {
            base__: <IWriteEngine2EventArgs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnce_Impl: ::windows_core::BaseImpl + IDiscFormat2_Impl {
    fn PrepareMedia(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddAudioTrack(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CancelAddTrack(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReleaseMedia(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetWriteSpeed(this: &Self::This, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRecorder(this: &Self::This, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn NumberOfExistingTracks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FreeSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UsedSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDoNotFinalizeMedia(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DoNotFinalizeMedia(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExpectedTableOfContents(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentPhysicalMediaType(this: &Self::This) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestedWriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2TrackAtOnce {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDiscFormat2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2TrackAtOnce {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrepareMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareMedia(this).into())
        }
        unsafe extern "system" fn AddAudioTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAudioTrack(this, ::windows_core::from_raw_borrowed(&data)).into())
        }
        unsafe extern "system" fn CancelAddTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAddTrack(this).into())
        }
        unsafe extern "system" fn ReleaseMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMedia(this).into())
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWriteSpeed(this, ::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into())
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecorder(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferUnderrunFreeDisabled(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferUnderrunFreeDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfExistingTracks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsedSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDoNotFinalizeMedia(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoNotFinalizeMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpectedTableOfContents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPhysicalMediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedWriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedRotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentWriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedWriteSpeeds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedWriteSpeedDescriptors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeddescriptors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscFormat2TrackAtOnce_Vtbl {
            base__: <IDiscFormat2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrepareMedia: PrepareMedia::<Identity, Impl, OFFSET>,
            AddAudioTrack: AddAudioTrack::<Identity, Impl, OFFSET>,
            CancelAddTrack: CancelAddTrack::<Identity, Impl, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Identity, Impl, OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Identity, Impl, OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Identity, Impl, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: ::windows_core::BaseImpl + IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentAction(this: &Self::This) -> ::windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RemainingTime(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscFormat2TrackAtOnceEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWriteEngine2EventArgs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscFormat2TrackAtOnceEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentTrackNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentTrackNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElapsedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElapsedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemainingTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscFormat2TrackAtOnceEventArgs_Vtbl {
            base__: <IWriteEngine2EventArgs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentTrackNumber: CurrentTrackNumber::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDiscMaster_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumDiscMasterFormats(this: &Self::This) -> ::windows_core::Result<IEnumDiscMasterFormats>;
    fn GetActiveDiscMasterFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetActiveDiscMasterFormat(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumDiscRecorders(this: &Self::This) -> ::windows_core::Result<IEnumDiscRecorders>;
    fn GetActiveDiscRecorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder>;
    fn SetActiveDiscRecorder(this: &Self::This, precorder: ::core::option::Option<&IDiscRecorder>) -> ::windows_core::Result<()>;
    fn ClearFormatContent(this: &Self::This) -> ::windows_core::Result<()>;
    fn ProgressAdvise(this: &Self::This, pevents: ::core::option::Option<&IDiscMasterProgressEvents>) -> ::windows_core::Result<usize>;
    fn ProgressUnadvise(this: &Self::This, vcookie: usize) -> ::windows_core::Result<()>;
    fn RecordDisc(this: &Self::This, bsimulate: u8, bejectafterburn: u8) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDiscMaster {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscMaster {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this).into())
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDiscMasterFormats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveDiscMasterFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpiid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveDiscMasterFormat(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn EnumDiscRecorders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDiscRecorders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprecorder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveDiscRecorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecorder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveDiscRecorder(this, ::windows_core::from_raw_borrowed(&precorder)).into())
        }
        unsafe extern "system" fn ClearFormatContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearFormatContent(this).into())
        }
        unsafe extern "system" fn ProgressAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, pvcookie: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProgressAdvise(this, ::windows_core::from_raw_borrowed(&pevents)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProgressUnadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProgressUnadvise(this, ::core::mem::transmute_copy(&vcookie)).into())
        }
        unsafe extern "system" fn RecordDisc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordDisc(this, ::core::mem::transmute_copy(&bsimulate), ::core::mem::transmute_copy(&bejectafterburn)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IDiscMaster_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            EnumDiscMasterFormats: EnumDiscMasterFormats::<Identity, Impl, OFFSET>,
            GetActiveDiscMasterFormat: GetActiveDiscMasterFormat::<Identity, Impl, OFFSET>,
            SetActiveDiscMasterFormat: SetActiveDiscMasterFormat::<Identity, Impl, OFFSET>,
            EnumDiscRecorders: EnumDiscRecorders::<Identity, Impl, OFFSET>,
            GetActiveDiscRecorder: GetActiveDiscRecorder::<Identity, Impl, OFFSET>,
            SetActiveDiscRecorder: SetActiveDiscRecorder::<Identity, Impl, OFFSET>,
            ClearFormatContent: ClearFormatContent::<Identity, Impl, OFFSET>,
            ProgressAdvise: ProgressAdvise::<Identity, Impl, OFFSET>,
            ProgressUnadvise: ProgressUnadvise::<Identity, Impl, OFFSET>,
            RecordDisc: RecordDisc::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscMaster2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsSupportedEnvironment(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscMaster2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscMaster2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSupportedEnvironment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSupportedEnvironment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscMaster2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            IsSupportedEnvironment: IsSupportedEnvironment::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDiscMasterProgressEvents_Impl: ::windows_core::BaseImpl {
    fn QueryCancel(this: &Self::This) -> ::windows_core::Result<u8>;
    fn NotifyPnPActivity(this: &Self::This) -> ::windows_core::Result<()>;
    fn NotifyAddProgress(this: &Self::This, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_core::Result<()>;
    fn NotifyBlockProgress(this: &Self::This, ncompleted: i32, ntotal: i32) -> ::windows_core::Result<()>;
    fn NotifyTrackProgress(this: &Self::This, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_core::Result<()>;
    fn NotifyPreparingBurn(this: &Self::This, nestimatedseconds: i32) -> ::windows_core::Result<()>;
    fn NotifyClosingDisc(this: &Self::This, nestimatedseconds: i32) -> ::windows_core::Result<()>;
    fn NotifyBurnComplete(this: &Self::This, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn NotifyEraseComplete(this: &Self::This, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDiscMasterProgressEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscMasterProgressEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryCancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryCancel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyPnPActivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyPnPActivity(this).into())
        }
        unsafe extern "system" fn NotifyAddProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyAddProgress(this, ::core::mem::transmute_copy(&ncompletedsteps), ::core::mem::transmute_copy(&ntotalsteps)).into())
        }
        unsafe extern "system" fn NotifyBlockProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyBlockProgress(this, ::core::mem::transmute_copy(&ncompleted), ::core::mem::transmute_copy(&ntotal)).into())
        }
        unsafe extern "system" fn NotifyTrackProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyTrackProgress(this, ::core::mem::transmute_copy(&ncurrenttrack), ::core::mem::transmute_copy(&ntotaltracks)).into())
        }
        unsafe extern "system" fn NotifyPreparingBurn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyPreparingBurn(this, ::core::mem::transmute_copy(&nestimatedseconds)).into())
        }
        unsafe extern "system" fn NotifyClosingDisc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyClosingDisc(this, ::core::mem::transmute_copy(&nestimatedseconds)).into())
        }
        unsafe extern "system" fn NotifyBurnComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyBurnComplete(this, ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn NotifyEraseComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyEraseComplete(this, ::core::mem::transmute_copy(&status)).into())
        }
        IDiscMasterProgressEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryCancel: QueryCancel::<Identity, Impl, OFFSET>,
            NotifyPnPActivity: NotifyPnPActivity::<Identity, Impl, OFFSET>,
            NotifyAddProgress: NotifyAddProgress::<Identity, Impl, OFFSET>,
            NotifyBlockProgress: NotifyBlockProgress::<Identity, Impl, OFFSET>,
            NotifyTrackProgress: NotifyTrackProgress::<Identity, Impl, OFFSET>,
            NotifyPreparingBurn: NotifyPreparingBurn::<Identity, Impl, OFFSET>,
            NotifyClosingDisc: NotifyClosingDisc::<Identity, Impl, OFFSET>,
            NotifyBurnComplete: NotifyBurnComplete::<Identity, Impl, OFFSET>,
            NotifyEraseComplete: NotifyEraseComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IDiscRecorder_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows_core::Result<()>;
    fn GetRecorderGUID(this: &Self::This, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetRecorderType(this: &Self::This) -> ::windows_core::Result<RECORDER_TYPES>;
    fn GetDisplayNames(this: &Self::This, pbstrvendorid: *mut ::windows_core::BSTR, pbstrproductid: *mut ::windows_core::BSTR, pbstrrevision: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBasePnPID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRecorderProperties(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetRecorderProperties(this: &Self::This, ppropstg: ::core::option::Option<&super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows_core::Result<()>;
    fn GetRecorderState(this: &Self::This) -> ::windows_core::Result<DISC_RECORDER_STATE_FLAGS>;
    fn OpenExclusive(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryMediaType(this: &Self::This, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_core::Result<()>;
    fn QueryMediaInfo(this: &Self::This, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_core::Result<()>;
    fn Eject(this: &Self::This) -> ::windows_core::Result<()>;
    fn Erase(this: &Self::This, bfullerase: u8) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for IDiscRecorder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscRecorder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&nulidsize), ::core::mem::transmute_copy(&nuldrivenumber)).into())
        }
        unsafe extern "system" fn GetRecorderGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRecorderGUID(this, ::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&ulbuffersize), ::core::mem::transmute_copy(&pulreturnsizerequired)).into())
        }
        unsafe extern "system" fn GetRecorderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecorderType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ftypecode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproductid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrrevision: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayNames(this, ::core::mem::transmute_copy(&pbstrvendorid), ::core::mem::transmute_copy(&pbstrproductid), ::core::mem::transmute_copy(&pbstrrevision)).into())
        }
        unsafe extern "system" fn GetBasePnPID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBasePnPID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbasepnpid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecorderProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecorderProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRecorderProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecorderProperties(this, ::windows_core::from_raw_borrowed(&ppropstg)).into())
        }
        unsafe extern "system" fn GetRecorderState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecorderState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puldevstateflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenExclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenExclusive(this).into())
        }
        unsafe extern "system" fn QueryMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryMediaType(this, ::core::mem::transmute_copy(&fmediatype), ::core::mem::transmute_copy(&fmediaflags)).into())
        }
        unsafe extern "system" fn QueryMediaInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryMediaInfo(this, ::core::mem::transmute_copy(&pbsessions), ::core::mem::transmute_copy(&pblasttrack), ::core::mem::transmute_copy(&ulstartaddress), ::core::mem::transmute_copy(&ulnextwritable), ::core::mem::transmute_copy(&ulfreeblocks)).into())
        }
        unsafe extern "system" fn Eject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Eject(this).into())
        }
        unsafe extern "system" fn Erase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Erase(this, ::core::mem::transmute_copy(&bfullerase)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IDiscRecorder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            GetRecorderGUID: GetRecorderGUID::<Identity, Impl, OFFSET>,
            GetRecorderType: GetRecorderType::<Identity, Impl, OFFSET>,
            GetDisplayNames: GetDisplayNames::<Identity, Impl, OFFSET>,
            GetBasePnPID: GetBasePnPID::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetRecorderProperties: GetRecorderProperties::<Identity, Impl, OFFSET>,
            SetRecorderProperties: SetRecorderProperties::<Identity, Impl, OFFSET>,
            GetRecorderState: GetRecorderState::<Identity, Impl, OFFSET>,
            OpenExclusive: OpenExclusive::<Identity, Impl, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, Impl, OFFSET>,
            QueryMediaInfo: QueryMediaInfo::<Identity, Impl, OFFSET>,
            Eject: Eject::<Identity, Impl, OFFSET>,
            Erase: Erase::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscRecorder2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EjectMedia(this: &Self::This) -> ::windows_core::Result<()>;
    fn CloseTray(this: &Self::This) -> ::windows_core::Result<()>;
    fn AcquireExclusiveAccess(this: &Self::This, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReleaseExclusiveAccess(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableMcn(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableMcn(this: &Self::This) -> ::windows_core::Result<()>;
    fn InitializeDiscRecorder(this: &Self::This, recorderuniqueid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ActiveDiscRecorder(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VendorId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductRevision(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumePathNames(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DeviceCanLoadMedia(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LegacyDeviceNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SupportedFeaturePages(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentFeaturePages(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedProfiles(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentProfiles(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedModePages(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ExclusiveAccessOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDiscRecorder2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscRecorder2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EjectMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EjectMedia(this).into())
        }
        unsafe extern "system" fn CloseTray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseTray(this).into())
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireExclusiveAccess(this, ::core::mem::transmute_copy(&force), ::core::mem::transmute(&__midl__idiscrecorder20000)).into())
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseExclusiveAccess(this).into())
        }
        unsafe extern "system" fn DisableMcn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableMcn(this).into())
        }
        unsafe extern "system" fn EnableMcn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableMcn(this).into())
        }
        unsafe extern "system" fn InitializeDiscRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recorderuniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeDiscRecorder(this, ::core::mem::transmute(&recorderuniqueid)).into())
        }
        unsafe extern "system" fn ActiveDiscRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveDiscRecorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VendorId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VendorId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProductId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProductRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductRevision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumePathNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumePathNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceCanLoadMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LegacyDeviceNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LegacyDeviceNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(legacydevicenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedFeaturePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedFeaturePages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFeaturePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFeaturePages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedProfiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentProfiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedModePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedModePages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExclusiveAccessOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscRecorder2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EjectMedia: EjectMedia::<Identity, Impl, OFFSET>,
            CloseTray: CloseTray::<Identity, Impl, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, Impl, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, Impl, OFFSET>,
            DisableMcn: DisableMcn::<Identity, Impl, OFFSET>,
            EnableMcn: EnableMcn::<Identity, Impl, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, Impl, OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Identity, Impl, OFFSET>,
            VendorId: VendorId::<Identity, Impl, OFFSET>,
            ProductId: ProductId::<Identity, Impl, OFFSET>,
            ProductRevision: ProductRevision::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            VolumePathNames: VolumePathNames::<Identity, Impl, OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Identity, Impl, OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Identity, Impl, OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Identity, Impl, OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Identity, Impl, OFFSET>,
            SupportedProfiles: SupportedProfiles::<Identity, Impl, OFFSET>,
            CurrentProfiles: CurrentProfiles::<Identity, Impl, OFFSET>,
            SupportedModePages: SupportedModePages::<Identity, Impl, OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDiscRecorder2Ex_Impl: ::windows_core::BaseImpl {
    fn SendCommandNoData(this: &Self::This, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows_core::Result<()>;
    fn SendCommandSendDataToDevice(this: &Self::This, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows_core::Result<()>;
    fn SendCommandGetDataFromDevice(this: &Self::This, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows_core::Result<()>;
    fn ReadDvdStructure(this: &Self::This, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn SendDvdStructure(this: &Self::This, format: u32, data: *const u8, count: u32) -> ::windows_core::Result<()>;
    fn GetAdapterDescriptor(this: &Self::This, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceDescriptor(this: &Self::This, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetDiscInformation(this: &Self::This, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetTrackInformation(this: &Self::This, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetFeaturePage(this: &Self::This, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetModePage(this: &Self::This, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn SetModePage(this: &Self::This, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows_core::Result<()>;
    fn GetSupportedFeaturePages(this: &Self::This, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSupportedProfiles(this: &Self::This, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_core::Result<()>;
    fn GetSupportedModePages(this: &Self::This, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_core::Result<()>;
    fn GetByteAlignmentMask(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDiscRecorder2Ex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiscRecorder2Ex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendCommandNoData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendCommandNoData(this, ::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout)).into())
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendCommandSendDataToDevice(this, ::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize)).into())
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendCommandGetDataFromDevice(this, ::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&bufferfetched)).into())
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadDvdStructure(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&layer), ::core::mem::transmute_copy(&agid), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SendDvdStructure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendDvdStructure(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterDescriptor(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceDescriptor(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetDiscInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDiscInformation(this, ::core::mem::transmute_copy(&discinformation), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetTrackInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTrackInformation(this, ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&trackinformation), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetFeaturePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeaturePage(this, ::core::mem::transmute_copy(&requestedfeature), ::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetModePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetModePage(this, ::core::mem::transmute_copy(&requestedmodepage), ::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagedata), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn SetModePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModePage(this, ::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSupportedFeaturePages(this, ::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into())
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSupportedProfiles(this, ::core::mem::transmute_copy(&currentonly), ::core::mem::transmute_copy(&profiletypes), ::core::mem::transmute_copy(&validprofiles)).into())
        }
        unsafe extern "system" fn GetSupportedModePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSupportedModePages(this, ::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagetypes), ::core::mem::transmute_copy(&validpages)).into())
        }
        unsafe extern "system" fn GetByteAlignmentMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByteAlignmentMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumNonPageAlignedTransferSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumPageAlignedTransferSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDiscRecorder2Ex_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendCommandNoData: SendCommandNoData::<Identity, Impl, OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Identity, Impl, OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Identity, Impl, OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Identity, Impl, OFFSET>,
            SendDvdStructure: SendDvdStructure::<Identity, Impl, OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Identity, Impl, OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Identity, Impl, OFFSET>,
            GetDiscInformation: GetDiscInformation::<Identity, Impl, OFFSET>,
            GetTrackInformation: GetTrackInformation::<Identity, Impl, OFFSET>,
            GetFeaturePage: GetFeaturePage::<Identity, Impl, OFFSET>,
            GetModePage: GetModePage::<Identity, Impl, OFFSET>,
            SetModePage: SetModePage::<Identity, Impl, OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Identity, Impl, OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Identity, Impl, OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Identity, Impl, OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Identity, Impl, OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Identity, Impl, OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumDiscMasterFormats_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cformats: u32, lpiidformatid: *mut ::windows_core::GUID, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cformats: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDiscMasterFormats>;
}
impl ::windows_core::Iids for IEnumDiscMasterFormats {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDiscMasterFormats {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows_core::GUID, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&lpiidformatid), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cformats)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDiscMasterFormats_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumDiscRecorders_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, crecorders: u32, pprecorder: *mut ::core::option::Option<IDiscRecorder>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, crecorders: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDiscRecorders>;
}
impl ::windows_core::Iids for IEnumDiscRecorders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDiscRecorders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&crecorders), ::core::mem::transmute_copy(&pprecorder), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&crecorders)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDiscRecorders_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumFsiItems_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IFsiItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumFsiItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumFsiItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumFsiItems_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumProgressItems_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IProgressItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumProgressItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumProgressItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumProgressItems_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Root(this: &Self::This) -> ::windows_core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSessionStartBlock(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn FreeMediaBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFreeMediaBlocks(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(this: &Self::This, discrecorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn UsedBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn VolumeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVolumeName(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportedVolumeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BootImageOptions(this: &Self::This) -> ::windows_core::Result<IBootOptions>;
    fn SetBootImageOptions(this: &Self::This, newval: ::core::option::Option<&IBootOptions>) -> ::windows_core::Result<()>;
    fn FileCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DirectoryCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn WorkingDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetWorkingDirectory(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangePoint(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StrictFileSystemCompliance(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStrictFileSystemCompliance(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseRestrictedCharacterSet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseRestrictedCharacterSet(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FileSystemsToCreate(this: &Self::This) -> ::windows_core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(this: &Self::This, newval: FsiFileSystems) -> ::windows_core::Result<()>;
    fn FileSystemsSupported(this: &Self::This) -> ::windows_core::Result<FsiFileSystems>;
    fn SetUDFRevision(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn UDFRevision(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UDFRevisionsSupported(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ChooseImageDefaults(this: &Self::This, discrecorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn ChooseImageDefaultsForMediaType(this: &Self::This, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::Result<()>;
    fn SetISO9660InterchangeLevel(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn ISO9660InterchangeLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateResultImage(this: &Self::This) -> ::windows_core::Result<IFileSystemImageResult>;
    fn Exists(this: &Self::This, fullpath: &::windows_core::BSTR) -> ::windows_core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IdentifyFileSystemsOnDisc(this: &Self::This, discrecorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(this: &Self::This, filesystems: FsiFileSystems) -> ::windows_core::Result<FsiFileSystems>;
    fn ImportFileSystem(this: &Self::This) -> ::windows_core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(this: &Self::This, filesystemtouse: FsiFileSystems) -> ::windows_core::Result<()>;
    fn RollbackToChangePoint(this: &Self::This, changepoint: i32) -> ::windows_core::Result<()>;
    fn LockInChangePoint(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateDirectoryItem(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsiFileItem>;
    fn VolumeNameUDF(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeNameJoliet(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeNameISO9660(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn StageFiles(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStageFiles(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn MultisessionInterfaces(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetMultisessionInterfaces(this: &Self::This, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFileSystemImage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSystemImage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Root<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Root(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionStartBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionStartBlock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSessionStartBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSessionStartBlock(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn FreeMediaBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeMediaBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFreeMediaBlocks(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxMediaBlocksFromDevice(this, ::windows_core::from_raw_borrowed(&discrecorder)).into())
        }
        unsafe extern "system" fn UsedBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsedBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolumeName(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn ImportedVolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportedVolumeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BootImageOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BootImageOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBootImageOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBootImageOptions(this, ::windows_core::from_raw_borrowed(&newval)).into())
        }
        unsafe extern "system" fn FileCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DirectoryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DirectoryCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WorkingDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkingDirectory(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn ChangePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChangePoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrictFileSystemCompliance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrictFileSystemCompliance(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseRestrictedCharacterSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseRestrictedCharacterSet(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn FileSystemsToCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSystemsToCreate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileSystemsToCreate(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn FileSystemsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSystemsSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUDFRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUDFRevision(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn UDFRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UDFRevision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UDFRevisionsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UDFRevisionsSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChooseImageDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChooseImageDefaults(this, ::windows_core::from_raw_borrowed(&discrecorder)).into())
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChooseImageDefaultsForMediaType(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetISO9660InterchangeLevel(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ISO9660InterchangeLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ISO9660InterchangeLevelsSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateResultImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateResultImage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Exists<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fullpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemtype: *mut FsiItemType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Exists(this, ::core::mem::transmute(&fullpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CalculateDiscIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(discidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void, filesystems: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IdentifyFileSystemsOnDisc(this, ::windows_core::from_raw_borrowed(&discrecorder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesystems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultFileSystemForImport(this, ::core::mem::transmute_copy(&filesystems)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(importdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportFileSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportFileSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(importedfilesystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportSpecificFileSystem(this, ::core::mem::transmute_copy(&filesystemtouse)).into())
        }
        unsafe extern "system" fn RollbackToChangePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RollbackToChangePoint(this, ::core::mem::transmute_copy(&changepoint)).into())
        }
        unsafe extern "system" fn LockInChangePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockInChangePoint(this).into())
        }
        unsafe extern "system" fn CreateDirectoryItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirectoryItem(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileItem(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeNameUDF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeNameUDF(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeNameJoliet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeNameJoliet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VolumeNameISO9660<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeNameISO9660(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StageFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StageFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStageFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStageFiles(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MultisessionInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultisessionInterfaces(this, ::core::mem::transmute_copy(&newval)).into())
        }
        IFileSystemImage_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Root: Root::<Identity, Impl, OFFSET>,
            SessionStartBlock: SessionStartBlock::<Identity, Impl, OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Identity, Impl, OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Identity, Impl, OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Identity, Impl, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, Impl, OFFSET>,
            UsedBlocks: UsedBlocks::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Identity, Impl, OFFSET>,
            BootImageOptions: BootImageOptions::<Identity, Impl, OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Identity, Impl, OFFSET>,
            FileCount: FileCount::<Identity, Impl, OFFSET>,
            DirectoryCount: DirectoryCount::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            ChangePoint: ChangePoint::<Identity, Impl, OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Identity, Impl, OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Identity, Impl, OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Identity, Impl, OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Identity, Impl, OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Identity, Impl, OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Identity, Impl, OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Identity, Impl, OFFSET>,
            SetUDFRevision: SetUDFRevision::<Identity, Impl, OFFSET>,
            UDFRevision: UDFRevision::<Identity, Impl, OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Identity, Impl, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, Impl, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, Impl, OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Identity, Impl, OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Identity, Impl, OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Identity, Impl, OFFSET>,
            CreateResultImage: CreateResultImage::<Identity, Impl, OFFSET>,
            Exists: Exists::<Identity, Impl, OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Identity, Impl, OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Identity, Impl, OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Identity, Impl, OFFSET>,
            ImportFileSystem: ImportFileSystem::<Identity, Impl, OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Identity, Impl, OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Identity, Impl, OFFSET>,
            LockInChangePoint: LockInChangePoint::<Identity, Impl, OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Identity, Impl, OFFSET>,
            CreateFileItem: CreateFileItem::<Identity, Impl, OFFSET>,
            VolumeNameUDF: VolumeNameUDF::<Identity, Impl, OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Identity, Impl, OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Identity, Impl, OFFSET>,
            StageFiles: StageFiles::<Identity, Impl, OFFSET>,
            SetStageFiles: SetStageFiles::<Identity, Impl, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, Impl, OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage2_Impl: ::windows_core::BaseImpl + IFileSystemImage_Impl {
    fn BootImageOptionsArray(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetBootImageOptionsArray(this: &Self::This, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFileSystemImage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFileSystemImage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSystemImage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BootImageOptionsArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BootImageOptionsArray(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBootImageOptionsArray(this, ::core::mem::transmute_copy(&newval)).into())
        }
        IFileSystemImage2_Vtbl {
            base__: <IFileSystemImage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BootImageOptionsArray: BootImageOptionsArray::<Identity, Impl, OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage3_Impl: ::windows_core::BaseImpl + IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCreateRedundantUdfMetadataFiles(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ProbeSpecificFileSystem(this: &Self::This, filesystemtoprobe: FsiFileSystems) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFileSystemImage3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFileSystemImage2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSystemImage3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRedundantUdfMetadataFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreateRedundantUdfMetadataFiles(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProbeSpecificFileSystem(this, ::core::mem::transmute_copy(&filesystemtoprobe)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isappendable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileSystemImage3_Vtbl {
            base__: <IFileSystemImage2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Identity, Impl, OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Identity, Impl, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ImageStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn ProgressItems(this: &Self::This) -> ::windows_core::Result<IProgressItems>;
    fn TotalBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn BlockSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DiscId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFileSystemImageResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSystemImageResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ImageStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProgressItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProgressItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BlockSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DiscId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiscId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileSystemImageResult_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ImageStream: ImageStream::<Identity, Impl, OFFSET>,
            ProgressItems: ProgressItems::<Identity, Impl, OFFSET>,
            TotalBlocks: TotalBlocks::<Identity, Impl, OFFSET>,
            BlockSize: BlockSize::<Identity, Impl, OFFSET>,
            DiscId: DiscId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult2_Impl: ::windows_core::BaseImpl + IFileSystemImageResult_Impl {
    fn ModifiedBlocks(this: &Self::This) -> ::windows_core::Result<IBlockRangeList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFileSystemImageResult2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFileSystemImageResult);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileSystemImageResult2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ModifiedBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileSystemImageResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifiedBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFileSystemImageResult2_Vtbl { base__: <IFileSystemImageResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ModifiedBlocks: ModifiedBlocks::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem_Impl: ::windows_core::BaseImpl + IFsiItem_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsiItem>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumFsiItems(this: &Self::This) -> ::windows_core::Result<IEnumFsiItems>;
    fn AddDirectory(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddFile(this: &Self::This, path: &::windows_core::BSTR, filedata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn AddTree(this: &Self::This, sourcedirectory: &::windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, item: ::core::option::Option<&IFsiItem>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveTree(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsiDirectoryItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsiItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsiDirectoryItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFsiItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFsiItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDirectory(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn AddFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filedata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFile(this, ::core::mem::transmute(&path), ::windows_core::from_raw_borrowed(&filedata)).into())
        }
        unsafe extern "system" fn AddTree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTree(this, ::core::mem::transmute(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&item)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn RemoveTree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTree(this, ::core::mem::transmute(&path)).into())
        }
        IFsiDirectoryItem_Vtbl {
            base__: <IFsiItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            EnumFsiItems: EnumFsiItems::<Identity, Impl, OFFSET>,
            AddDirectory: AddDirectory::<Identity, Impl, OFFSET>,
            AddFile: AddFile::<Identity, Impl, OFFSET>,
            AddTree: AddTree::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveTree: RemoveTree::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem2_Impl: ::windows_core::BaseImpl + IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(this: &Self::This, sourcedirectory: &::windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsiDirectoryItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsiDirectoryItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsiDirectoryItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiDirectoryItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTreeWithNamedStreams(this, ::core::mem::transmute(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into())
        }
        IFsiDirectoryItem2_Vtbl {
            base__: <IFsiDirectoryItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem_Impl: ::windows_core::BaseImpl + IFsiItem_Impl {
    fn DataSize(this: &Self::This) -> ::windows_core::Result<i64>;
    fn DataSize32BitLow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DataSize32BitHigh(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Data(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetData(this: &Self::This, newval: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsiFileItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsiItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsiFileItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataSize32BitLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataSize32BitLow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataSize32BitHigh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataSize32BitHigh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::windows_core::from_raw_borrowed(&newval)).into())
        }
        IFsiFileItem_Vtbl {
            base__: <IFsiItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DataSize: DataSize::<Identity, Impl, OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Identity, Impl, OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem2_Impl: ::windows_core::BaseImpl + IFsiFileItem_Impl {
    fn FsiNamedStreams(this: &Self::This) -> ::windows_core::Result<IFsiNamedStreams>;
    fn IsNamedStream(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AddStream(this: &Self::This, name: &::windows_core::BSTR, streamdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn RemoveStream(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsRealTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRealTime(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsiFileItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFsiFileItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsiFileItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FsiNamedStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FsiNamedStreams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsNamedStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsNamedStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, streamdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStream(this, ::core::mem::transmute(&name), ::windows_core::from_raw_borrowed(&streamdata)).into())
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStream(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRealTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsRealTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsRealTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        IFsiFileItem2_Vtbl {
            base__: <IFsiFileItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FsiNamedStreams: FsiNamedStreams::<Identity, Impl, OFFSET>,
            IsNamedStream: IsNamedStream::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            SetIsRealTime: SetIsRealTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiItem_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FullPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreationTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetCreationTime(this: &Self::This, newval: f64) -> ::windows_core::Result<()>;
    fn LastAccessedTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetLastAccessedTime(this: &Self::This, newval: f64) -> ::windows_core::Result<()>;
    fn LastModifiedTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetLastModifiedTime(this: &Self::This, newval: f64) -> ::windows_core::Result<()>;
    fn IsHidden(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FileSystemName(this: &Self::This, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FileSystemPath(this: &Self::This, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsiItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsiItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FullPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FullPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreationTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn LastAccessedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastAccessedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastAccessedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastAccessedTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn LastModifiedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastModifiedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastModifiedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastModifiedTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn IsHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsHidden(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsHidden(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn FileSystemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSystemName(this, ::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FileSystemPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSystemPath(this, ::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsiItem_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            FullPath: FullPath::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            SetCreationTime: SetCreationTime::<Identity, Impl, OFFSET>,
            LastAccessedTime: LastAccessedTime::<Identity, Impl, OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Identity, Impl, OFFSET>,
            LastModifiedTime: LastModifiedTime::<Identity, Impl, OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Identity, Impl, OFFSET>,
            IsHidden: IsHidden::<Identity, Impl, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, Impl, OFFSET>,
            FileSystemName: FileSystemName::<Identity, Impl, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiNamedStreams_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IFsiFileItem2>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EnumNamedStreams(this: &Self::This) -> ::windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFsiNamedStreams {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFsiNamedStreams {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumNamedStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumNamedStreams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFsiNamedStreams_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIsoImageManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Stream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetPath(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetStream(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Validate(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIsoImageManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIsoImageManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn SetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStream(this, ::windows_core::from_raw_borrowed(&data)).into())
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Validate(this).into())
        }
        IIsoImageManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            SetStream: SetStream::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IJolietDiscMaster_Impl: ::windows_core::BaseImpl {
    fn GetTotalDataBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetUsedDataBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetDataBlockSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AddData(this: &Self::This, pstorage: ::core::option::Option<&super::super::System::Com::StructuredStorage::IStorage>, lfileoverwrite: i32) -> ::windows_core::Result<()>;
    fn GetJolietProperties(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetJolietProperties(this: &Self::This, ppropstg: ::core::option::Option<&super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for IJolietDiscMaster {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJolietDiscMaster {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTotalDataBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTotalDataBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUsedDataBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUsedDataBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataBlockSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataBlockSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblockbytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstorage: *mut ::core::ffi::c_void, lfileoverwrite: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddData(this, ::windows_core::from_raw_borrowed(&pstorage), ::core::mem::transmute_copy(&lfileoverwrite)).into())
        }
        unsafe extern "system" fn GetJolietProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJolietProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJolietProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJolietProperties(this, ::windows_core::from_raw_borrowed(&ppropstg)).into())
        }
        IJolietDiscMaster_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTotalDataBlocks: GetTotalDataBlocks::<Identity, Impl, OFFSET>,
            GetUsedDataBlocks: GetUsedDataBlocks::<Identity, Impl, OFFSET>,
            GetDataBlockSize: GetDataBlockSize::<Identity, Impl, OFFSET>,
            AddData: AddData::<Identity, Impl, OFFSET>,
            GetJolietProperties: GetJolietProperties::<Identity, Impl, OFFSET>,
            SetJolietProperties: SetJolietProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisession_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetInUse(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn InUse(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ImportRecorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMultisession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultisession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSupportedOnCurrentMediaState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInUse(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn InUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InUse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImportRecorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMultisession_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Identity, Impl, OFFSET>,
            SetInUse: SetInUse::<Identity, Impl, OFFSET>,
            InUse: InUse::<Identity, Impl, OFFSET>,
            ImportRecorder: ImportRecorder::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionRandomWrite_Impl: ::windows_core::BaseImpl + IMultisession_Impl {
    fn WriteUnitSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastWrittenAddress(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMultisessionRandomWrite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMultisession);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultisessionRandomWrite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteUnitSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteUnitSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastWrittenAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastWrittenAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMultisessionRandomWrite_Vtbl {
            base__: <IMultisession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteUnitSize: WriteUnitSize::<Identity, Impl, OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential_Impl: ::windows_core::BaseImpl + IMultisession_Impl {
    fn IsFirstDataSession(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartAddressOfPreviousSession(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NextWritableAddress(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FreeSectorsOnMedia(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMultisessionSequential {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMultisession);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultisessionSequential {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsFirstDataSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstDataSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartAddressOfPreviousSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastWrittenAddressOfPreviousSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextWritableAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextWritableAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeSectorsOnMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMultisessionSequential_Vtbl {
            base__: <IMultisession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsFirstDataSession: IsFirstDataSession::<Identity, Impl, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential2_Impl: ::windows_core::BaseImpl + IMultisessionSequential_Impl {
    fn WriteUnitSize(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMultisessionSequential2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMultisessionSequential);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultisessionSequential2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteUnitSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultisessionSequential2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteUnitSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMultisessionSequential2_Vtbl { base__: <IMultisessionSequential as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, WriteUnitSize: WriteUnitSize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItem_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FirstBlock(this: &Self::This) -> ::windows_core::Result<u32>;
    fn LastBlock(this: &Self::This) -> ::windows_core::Result<u32>;
    fn BlockCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IProgressItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProgressItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(desc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirstBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstBlock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastBlock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BlockCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProgressItem_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            FirstBlock: FirstBlock::<Identity, Impl, OFFSET>,
            LastBlock: LastBlock::<Identity, Impl, OFFSET>,
            BlockCount: BlockCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItems_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IProgressItem>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProgressItemFromBlock(this: &Self::This, block: u32) -> ::windows_core::Result<IProgressItem>;
    fn ProgressItemFromDescription(this: &Self::This, description: &::windows_core::BSTR) -> ::windows_core::Result<IProgressItem>;
    fn EnumProgressItems(this: &Self::This) -> ::windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IProgressItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProgressItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, block: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProgressItemFromBlock(this, ::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProgressItemFromDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProgressItemFromDescription(this, ::core::mem::transmute(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumProgressItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumProgressItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProgressItems_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, Impl, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, Impl, OFFSET>,
            EnumProgressItems: EnumProgressItems::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageCreator_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CreateResultImage(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn AddTrack(this: &Self::This, datatype: IMAPI_CD_SECTOR_TYPE, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<i32>;
    fn AddSpecialPregap(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn AddSubcodeRWGenerator(this: &Self::This, subcode: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetResultingImageType(this: &Self::This, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::Result<()>;
    fn ResultingImageType(this: &Self::This) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetStartOfLeadoutLimit(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn StartOfLeadoutLimit(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDisableGaplessAudio(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DisableGaplessAudio(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMediaCatalogNumber(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MediaCatalogNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStartingTrackNumber(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn StartingTrackNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_TrackInfo(this: &Self::This, trackindex: i32) -> ::windows_core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastUsedUserSectorInImage(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ExpectedTableOfContents(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRawCDImageCreator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawCDImageCreator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateResultImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateResultImage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut ::core::ffi::c_void, trackindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddTrack(this, ::core::mem::transmute_copy(&datatype), ::windows_core::from_raw_borrowed(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(trackindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddSpecialPregap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSpecialPregap(this, ::windows_core::from_raw_borrowed(&data)).into())
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subcode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSubcodeRWGenerator(this, ::windows_core::from_raw_borrowed(&subcode)).into())
        }
        unsafe extern "system" fn SetResultingImageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResultingImageType(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ResultingImageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResultingImageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartOfLeadout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartOfLeadout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartOfLeadoutLimit(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartOfLeadoutLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisableGaplessAudio(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn DisableGaplessAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisableGaplessAudio(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMediaCatalogNumber(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn MediaCatalogNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaCatalogNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartingTrackNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartingTrackNumber(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn StartingTrackNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartingTrackNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_TrackInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_TrackInfo(this, ::core::mem::transmute_copy(&trackindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfExistingTracks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastUsedUserSectorInImage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpectedTableOfContents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawCDImageCreator_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateResultImage: CreateResultImage::<Identity, Impl, OFFSET>,
            AddTrack: AddTrack::<Identity, Impl, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, Impl, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, Impl, OFFSET>,
            SetResultingImageType: SetResultingImageType::<Identity, Impl, OFFSET>,
            ResultingImageType: ResultingImageType::<Identity, Impl, OFFSET>,
            StartOfLeadout: StartOfLeadout::<Identity, Impl, OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Identity, Impl, OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Identity, Impl, OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Identity, Impl, OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Identity, Impl, OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Identity, Impl, OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Identity, Impl, OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Identity, Impl, OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Identity, Impl, OFFSET>,
            get_TrackInfo: get_TrackInfo::<Identity, Impl, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, Impl, OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Identity, Impl, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageTrackInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StartingLba(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SectorCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TrackNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SectorType(this: &Self::This) -> ::windows_core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetISRC(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DigitalAudioCopySetting(this: &Self::This) -> ::windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(this: &Self::This, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::Result<()>;
    fn AudioHasPreemphasis(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAudioHasPreemphasis(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn TrackIndexes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddTrackIndex(this: &Self::This, lbaoffset: i32) -> ::windows_core::Result<()>;
    fn ClearTrackIndex(this: &Self::This, lbaoffset: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRawCDImageTrackInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawCDImageTrackInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartingLba<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartingLba(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SectorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SectorCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrackNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrackNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SectorType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SectorType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ISRC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ISRC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetISRC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetISRC(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DigitalAudioCopySetting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDigitalAudioCopySetting(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn AudioHasPreemphasis<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioHasPreemphasis(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioHasPreemphasis(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn TrackIndexes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrackIndexes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTrackIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTrackIndex(this, ::core::mem::transmute_copy(&lbaoffset)).into())
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearTrackIndex(this, ::core::mem::transmute_copy(&lbaoffset)).into())
        }
        IRawCDImageTrackInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartingLba: StartingLba::<Identity, Impl, OFFSET>,
            SectorCount: SectorCount::<Identity, Impl, OFFSET>,
            TrackNumber: TrackNumber::<Identity, Impl, OFFSET>,
            SectorType: SectorType::<Identity, Impl, OFFSET>,
            ISRC: ISRC::<Identity, Impl, OFFSET>,
            SetISRC: SetISRC::<Identity, Impl, OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Identity, Impl, OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Identity, Impl, OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Identity, Impl, OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Identity, Impl, OFFSET>,
            TrackIndexes: TrackIndexes::<Identity, Impl, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, Impl, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRedbookDiscMaster_Impl: ::windows_core::BaseImpl {
    fn GetTotalAudioTracks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetTotalAudioBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetUsedAudioBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetAvailableAudioTrackBlocks(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetAudioBlockSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CreateAudioTrack(this: &Self::This, nblocks: i32) -> ::windows_core::Result<()>;
    fn AddAudioTrackBlocks(this: &Self::This, pby: *const u8, cb: i32) -> ::windows_core::Result<()>;
    fn CloseAudioTrack(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRedbookDiscMaster {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRedbookDiscMaster {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTotalAudioTracks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTotalAudioTracks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pntracks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTotalAudioBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUsedAudioBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableAudioTrackBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAudioBlockSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudioBlockSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblockbytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAudioTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAudioTrack(this, ::core::mem::transmute_copy(&nblocks)).into())
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAudioTrackBlocks(this, ::core::mem::transmute_copy(&pby), ::core::mem::transmute_copy(&cb)).into())
        }
        unsafe extern "system" fn CloseAudioTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseAudioTrack(this).into())
        }
        IRedbookDiscMaster_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTotalAudioTracks: GetTotalAudioTracks::<Identity, Impl, OFFSET>,
            GetTotalAudioBlocks: GetTotalAudioBlocks::<Identity, Impl, OFFSET>,
            GetUsedAudioBlocks: GetUsedAudioBlocks::<Identity, Impl, OFFSET>,
            GetAvailableAudioTrackBlocks: GetAvailableAudioTrackBlocks::<Identity, Impl, OFFSET>,
            GetAudioBlockSize: GetAudioBlockSize::<Identity, Impl, OFFSET>,
            CreateAudioTrack: CreateAudioTrack::<Identity, Impl, OFFSET>,
            AddAudioTrackBlocks: AddAudioTrackBlocks::<Identity, Impl, OFFSET>,
            CloseAudioTrack: CloseAudioTrack::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStreamConcatenate_Impl: ::windows_core::BaseImpl + super::super::System::Com::IStream_Impl {
    fn Initialize(this: &Self::This, stream1: ::core::option::Option<&super::super::System::Com::IStream>, stream2: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Initialize2(this: &Self::This, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Append2(this: &Self::This, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IStreamConcatenate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStreamConcatenate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream1: *mut ::core::ffi::c_void, stream2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&stream1), ::windows_core::from_raw_borrowed(&stream2)).into())
        }
        unsafe extern "system" fn Initialize2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize2(this, ::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&stream)).into())
        }
        unsafe extern "system" fn Append2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append2(this, ::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into())
        }
        IStreamConcatenate_Vtbl {
            base__: <super::super::System::Com::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Initialize2: Initialize2::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Append2: Append2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStreamInterleave_Impl: ::windows_core::BaseImpl + super::super::System::Com::IStream_Impl {
    fn Initialize(this: &Self::This, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IStreamInterleave {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamInterleave_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStreamInterleave {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamInterleave_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&interleavesizes), ::core::mem::transmute_copy(&streamcount)).into())
        }
        IStreamInterleave_Vtbl {
            base__: <super::super::System::Com::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStreamPseudoRandomBased_Impl: ::windows_core::BaseImpl + super::super::System::Com::IStream_Impl {
    fn SetSeed(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn Seed(this: &Self::This) -> ::windows_core::Result<u32>;
    fn put_ExtendedSeed(this: &Self::This, values: *const u32, ecount: u32) -> ::windows_core::Result<()>;
    fn get_ExtendedSeed(this: &Self::This, values: *mut *mut u32, ecount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IStreamPseudoRandomBased {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStreamPseudoRandomBased {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeed(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Seed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Seed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_ExtendedSeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_ExtendedSeed(this, ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into())
        }
        unsafe extern "system" fn get_ExtendedSeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_ExtendedSeed(this, ::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into())
        }
        IStreamPseudoRandomBased_Vtbl {
            base__: <super::super::System::Com::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSeed: SetSeed::<Identity, Impl, OFFSET>,
            Seed: Seed::<Identity, Impl, OFFSET>,
            put_ExtendedSeed: put_ExtendedSeed::<Identity, Impl, OFFSET>,
            get_ExtendedSeed: get_ExtendedSeed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn WriteSection(this: &Self::This, data: ::core::option::Option<&super::super::System::Com::IStream>, startingblockaddress: i32, numberofblocks: i32) -> ::windows_core::Result<()>;
    fn CancelWrite(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetRecorder(this: &Self::This, value: ::core::option::Option<&IDiscRecorder2Ex>) -> ::windows_core::Result<()>;
    fn Recorder(this: &Self::This) -> ::windows_core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(this: &Self::This, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseStreamingWrite12(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStartingSectorsPerSecond(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn StartingSectorsPerSecond(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEndingSectorsPerSecond(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn EndingSectorsPerSecond(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBytesPerSector(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn BytesPerSector(this: &Self::This) -> ::windows_core::Result<i32>;
    fn WriteInProgress(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWriteEngine2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWriteEngine2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteSection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSection(this, ::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&startingblockaddress), ::core::mem::transmute_copy(&numberofblocks)).into())
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelWrite(this).into())
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecorder(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseStreamingWrite12(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn UseStreamingWrite12<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseStreamingWrite12(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartingSectorsPerSecond(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartingSectorsPerSecond(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndingSectorsPerSecond(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndingSectorsPerSecond(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBytesPerSector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBytesPerSector(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn BytesPerSector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BytesPerSector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteInProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteInProgress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWriteEngine2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteSection: WriteSection::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Identity, Impl, OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Identity, Impl, OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Identity, Impl, OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Identity, Impl, OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Identity, Impl, OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Identity, Impl, OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Identity, Impl, OFFSET>,
            BytesPerSector: BytesPerSector::<Identity, Impl, OFFSET>,
            WriteInProgress: WriteInProgress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2EventArgs_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StartLba(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SectorCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastReadLba(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastWrittenLba(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalSystemBuffer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UsedSystemBuffer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FreeSystemBuffer(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWriteEngine2EventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWriteEngine2EventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartLba<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartLba(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SectorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SectorCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastReadLba<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastReadLba(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastWrittenLba<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastWrittenLba(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalSystemBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalSystemBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UsedSystemBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsedSystemBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeSystemBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeSystemBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWriteEngine2EventArgs_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartLba: StartLba::<Identity, Impl, OFFSET>,
            SectorCount: SectorCount::<Identity, Impl, OFFSET>,
            LastReadLba: LastReadLba::<Identity, Impl, OFFSET>,
            LastWrittenLba: LastWrittenLba::<Identity, Impl, OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Identity, Impl, OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Identity, Impl, OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteSpeedDescriptor_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn MediaType(this: &Self::This) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WriteSpeed(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWriteSpeedDescriptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWriteSpeedDescriptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RotationTypeIsPureCAV(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWriteSpeedDescriptor_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            WriteSpeed: WriteSpeed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
