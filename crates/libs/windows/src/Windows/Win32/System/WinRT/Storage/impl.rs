pub trait IOplockBreakingHandler_Impl: ::windows_core::BaseImpl {
    fn OplockBreaking(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOplockBreakingHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOplockBreakingHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOplockBreakingHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OplockBreaking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOplockBreakingHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OplockBreaking(this).into())
        }
        IOplockBreakingHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OplockBreaking: OplockBreaking::<Identity, Impl, OFFSET> }
    };
}
pub trait IRandomAccessStreamFileAccessMode_Impl: ::windows_core::BaseImpl {
    fn GetMode(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IRandomAccessStreamFileAccessMode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRandomAccessStreamFileAccessMode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileaccessmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRandomAccessStreamFileAccessMode_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetMode: GetMode::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageFolderHandleAccess_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, filename: &::windows_core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::core::option::Option<&IOplockBreakingHandler>) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IStorageFolderHandleAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderHandleAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFolderHandleAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderHandleAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&creationoptions), ::core::mem::transmute_copy(&accessoptions), ::core::mem::transmute_copy(&sharingoptions), ::core::mem::transmute_copy(&options), ::windows_core::from_raw_borrowed(&oplockbreakinghandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interophandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFolderHandleAccess_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageItemHandleAccess_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::core::option::Option<&IOplockBreakingHandler>) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IStorageItemHandleAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemHandleAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemHandleAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemHandleAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&accessoptions), ::core::mem::transmute_copy(&sharingoptions), ::core::mem::transmute_copy(&options), ::windows_core::from_raw_borrowed(&oplockbreakinghandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interophandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItemHandleAccess_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
pub trait IUnbufferedFileHandleOplockCallback_Impl: ::windows_core::BaseImpl {
    fn OnBrokenCallback(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUnbufferedFileHandleOplockCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUnbufferedFileHandleOplockCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnBrokenCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBrokenCallback(this).into())
        }
        IUnbufferedFileHandleOplockCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnBrokenCallback: OnBrokenCallback::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUnbufferedFileHandleProvider_Impl: ::windows_core::BaseImpl {
    fn OpenUnbufferedFileHandle(this: &Self::This, oplockbreakcallback: ::core::option::Option<&IUnbufferedFileHandleOplockCallback>) -> ::windows_core::Result<usize>;
    fn CloseUnbufferedFileHandle(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUnbufferedFileHandleProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUnbufferedFileHandleProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oplockbreakcallback: *mut ::core::ffi::c_void, filehandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenUnbufferedFileHandle(this, ::windows_core::from_raw_borrowed(&oplockbreakcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filehandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseUnbufferedFileHandle(this).into())
        }
        IUnbufferedFileHandleProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenUnbufferedFileHandle: OpenUnbufferedFileHandle::<Identity, Impl, OFFSET>,
            CloseUnbufferedFileHandle: CloseUnbufferedFileHandle::<Identity, Impl, OFFSET>,
        }
    };
}
